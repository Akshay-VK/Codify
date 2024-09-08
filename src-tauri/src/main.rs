// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![file_test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
}

#[tauri::command]
fn file_test(handle: tauri::AppHandle) -> Vec<String> {
  let resource_path = handle.path_resolver()
     .resolve_resource("resources/config.default.yaml")
     .expect("failed to resolve resource");

   let file = std::fs::File::open(&resource_path).unwrap();
   let conf: config::Config = serde_yml::from_reader(file).unwrap();

   conf.actions[0].commands.clone()
}