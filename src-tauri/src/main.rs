// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::Action;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_config])
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