// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![file_test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
}

#[derive(Serialize, Deserialize, Debug)]
struct Config{
  actions: Vec<Action>
}

#[derive(Serialize, Deserialize, Debug)]
struct Action{
  name: String,
  commands: Vec<String>
}

#[tauri::command]
fn file_test(handle: tauri::AppHandle) -> String {
  let resource_path = handle.path_resolver()
     .resolve_resource("resources/config.default.yaml")
     .expect("failed to resolve resource");

   let file = std::fs::File::open(&resource_path).unwrap();
   let conf: Config = serde_yml::from_reader(file).unwrap();

   conf.actions[1].name.clone()
}