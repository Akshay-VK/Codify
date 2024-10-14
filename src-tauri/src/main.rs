// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

mod config;
use config::Action;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_config,run_command,run_command_stream])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
}

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
fn run_command(baseLocation:String, action: Action) -> String{
  println!("Executing {}",action.name);
  let output = Command::new("cmd")
        .arg("/C")
        .arg(format!("cd {} & {}",baseLocation,&action.commands.join(" & ")))
        .output()
        .expect("Errorrr");

  String::from_utf8(output.stdout).unwrap()
}

#[derive(Clone, serde::Serialize)]
struct OutputPayload{
  data:String,
}

#[tauri::command]
fn run_command_stream(window: Window, baseLocation:String, action: Action){
  println!("Executing {}",action.name);
  thread::spawn(move ||{
    let mut command = Command::new("cmd")
          .arg("/C")
          .arg(format!("cd {} & {}",baseLocation,&action.commands.join(" & ")))
          .stdout(Stdio::piped())
          .spawn()
          .expect("Errorrr");
          
    let stdout = command.stdout.take().unwrap();

    // Stream output.
    let lines = BufReader::new(stdout).lines();
    for line in lines {
      let _ = window.emit("output_data",OutputPayload{data:line.unwrap().to_string()});
    }
  });
}