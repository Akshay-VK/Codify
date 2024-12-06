// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State};

use std::sync::Mutex;


mod config;
use config::{Data,DataStore,FolderData,Subfolder,FileData};

mod files;
mod commands;

fn main() {
    //this generates the main app, adds the commands and builds it
    tauri::Builder::default()
        .setup(|app| {
            let res_path = app
                .path_resolver()
                .resolve_resource("resources/data.json")
                .expect("Error loading data.json");
            let file = std::fs::File::open(&res_path).unwrap();
            let dat: DataStore = serde_json::from_reader(file).unwrap();
            println!("{:?}", dat);

            app.manage(Data {
                data: Mutex::new(dat),
                commandCount:Mutex::new(0)
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            commands::run_command,
            commands::run_command_stream,
            state_test,
            files::change_yaml,
            files::dir_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

///This command reads the specified YAML file and builds it according to types in config.rs and finally returns it
#[tauri::command]
fn get_config(handle: tauri::AppHandle, state: State<Data>) -> config::Config {
    let p = state.data.lock().unwrap();
    let resource_path = if p.pathToConfig.len() <=0 {
        handle.path_resolver().resolve_resource("resources/config.default.yaml").expect("failed to resolve resource config.default.yaml")
    } else {
        std::path::PathBuf::from(p.pathToConfig.clone())
    };
    if p.pathToConfig.len() > 0{
        println!("Config file present.");
    }

    let file = std::fs::File::open(&resource_path).unwrap();
    let conf: config::Config = serde_yml::from_reader(file).unwrap();

    
    conf
}
#[tauri::command]
fn state_test(state: State<Data>) -> String {
    let d = state.data.lock().unwrap();
    (*d.pathToConfig).to_string()
}