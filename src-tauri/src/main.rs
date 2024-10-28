// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use tauri::{Menu, Submenu, MenuItem, CustomMenuItem};

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

use serde::{Serialize,Deserialize};

mod config;
use config::Action;

#[derive(Debug,Serialize,Deserialize)]
struct Data{
  pathToConfig: String
}

fn main() {
  let submenu = Submenu::new("File",Menu::new()
    .add_item(CustomMenuItem::new("quit".to_string(),"Quit"))
    .add_item(CustomMenuItem::new("close".to_string(),"Close"))
  );
  let menu = Menu::new()
    .add_submenu(submenu);
  //this generates the main app, adds the commands and builds it
  tauri::Builder::default()
    .menu(menu)
    .setup(|app| {
      let res_path= app.path_resolver()
          .resolve_resource("resources/data.json")
          .expect("Error loading data.json");
      let file = std::fs::File::open(&res_path).unwrap();
      let data: Data = serde_json::from_reader(file).unwrap();
      println!("{:?}",data);
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_config,run_command,run_command_stream])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
  
}

///This command reads the specified YAML file and builds it according to types in config.rs and finally returns it
#[tauri::command]
fn get_config(handle: tauri::AppHandle) -> config::Config {
  let resource_path = handle.path_resolver()
     .resolve_resource("resources/config.default.yaml")
     .expect("failed to resolve resource");

  let file = std::fs::File::open(&resource_path).unwrap();
  let conf: config::Config = serde_yml::from_reader(file).unwrap();
  conf
}

#[tauri::command]
///This is the original command to run actions
/// It runs on the main thread and causes the app to hang it the command is long
/// It also doesn't support arguments
fn run_command(baseLocation:String, action: Action) -> String{
  println!("Executing {}",action.name);
  let output = Command::new("cmd")
        .arg("/C")
        .arg(format!("cd {} & {}",baseLocation,&action.commands.join(" & ")))
        .output()
        .expect("Errorrr");

  String::from_utf8(output.stdout).unwrap()
}

///This is the shape of the payload output
/// Whenever the command returns a line of output, it gets packed to this and is emitted as an event
#[derive(Clone, serde::Serialize)]
struct OutputPayload{
  data:String,
}

///This is the main function that runs commands
/// It creates a thread to run each command seperately
/// If not, longer commands cause the app to hang.
/// It runs the commands and waits for output
/// As a string of output arrives, it emits just that line as a event to the frontend.
#[tauri::command]
fn run_command_stream(window: Window, baseLocation:String, action: Action, args: Vec<String>){
  println!("Executing {}",action.name);
  // This creates a new thread for running the process
  // This is done so that the main app doesn't hang
  thread::spawn(move ||{
    let s = "echo --------RUNNING ".to_owned()+&action.name+" ------- & " + &action.commands.join(" & ") + " & echo --------------------------------";
    
    //INSERT ARGUMENTS
    //Here the main command is generated which is all commands concatenated with the ' & ' 
    let words: Vec<&str> =s.split(" ").collect();
    let mut res="".to_string();
    for word in words{
      //This is the part which inserts arguments
      if word.starts_with("$"){
        //We split it at the '.' so that we can allow things like
        // $name.txt
        let parts: Vec<&str> =word.split(".").collect();
        //Here the name of the argument used is retrieved
        let m=&parts[0][1..];
        //We find the index, in the order of arguments mentioned in the command, at which this argument is present
        //We take the value of args at that index as the arguments are passed in that same order
        let value = &args[action.arguments.iter().position(|x| x==m).unwrap()];
        res=res+" "+value;
        //If we used the '.' property we add back the rest of the filename or whatever was used
        if parts.len() > 1{
          res=res+"."+parts[1];
        }
      }else{
        // If no argument was used we simply add the word
        res = res+" "+word;
      }
    }
    println!("{}",&res);

    // Here the command is spawned
    let mut command = Command::new("cmd")
          .arg("/C")
          //We must first change to the base directory as each 'command' is independent and all of them run independently.
          // For example, command A makes a folder, makes a file in that folder and writes data to it
          //Command B makes a folder and in it makes a npm project and adds a library
          //If A and B are run one after the other, both the folder will be in the base location and not nested
          .arg(format!("cd {} & {}",baseLocation,res))
          //We pipe it to read the output live and not wait for the entire process to finish
          .stdout(Stdio::piped())
          .spawn()
          .expect("Error while running command");

    //Here we make a buffer to read the continuously arriving output and emit it as events with the output line as payload.
    let stdout = command.stdout.take().unwrap();
    let lines = BufReader::new(stdout).lines();
    for line in lines {
      let pl = line.unwrap().to_string();
      if pl.contains("Ctrl-C") || pl.contains("Ctrl+C") || pl.contains("^C"){
        return;
      }
      let _ = window.emit("output_data",OutputPayload{data:pl});
    }
  });
}