use tauri::State;

use walkdir::{WalkDir,DirEntry};

use crate::config;
use config::{Data,FolderData,Subfolder,FileData};

use rayon::prelude::*;

#[derive(serde::Serialize)]
pub struct YAMLChangePayload{
    config: config::Config,
    success:bool,
    msg:String
}

#[tauri::command]
// This handles the controlled change of the config file ensuring everything stays updated
pub fn change_yaml(handle: tauri::AppHandle, path: String, state:State<Data>)-> YAMLChangePayload{
    println!("Path selected: {}",&path);

    let mut p = state.data.lock().unwrap();
    // First we load the file which is already chosen
    // If there is no file mentioned in data.json, we load the default
    let resource_path = if p.pathToConfig.len() <=0 {
        handle.path_resolver().resolve_resource("resources/config.default.yaml").expect("failed to resolve resource config.default.yaml")
    } else {
        std::path::PathBuf::from(p.pathToConfig.clone())
    };
    if p.pathToConfig.len() > 0{
        println!("Config file present.");
    }
    let def_file = std::fs::File::open(&resource_path).unwrap();
    let def_conf: config::Config = serde_yml::from_reader(def_file).unwrap();

    // Here we try loading the path chosen
    // We can unwrap the path as it is chosen from the file picker
    // But we cannot unwrap the serialisation as there couldbe syntac errors
    let file = std::fs::File::open(std::path::PathBuf::from(path.clone())).unwrap();
    let conf = serde_yml::from_reader(file);
    let _res: YAMLChangePayload= match conf{
        Err(e) => {
            println!("Error handled: {}",&e.to_string()); 
            return YAMLChangePayload{config:def_conf,success:false,msg:e.to_string()}
        }
        Ok(c) => {
            // We update the data.json file as well
            p.pathToConfig=path;
            let res_path = handle
                .path_resolver()
                .resolve_resource("resources/data.json")
                .expect("Error loading data.json");

            let dataToWrite = serde_json::to_string(&*p).unwrap();
            println!("New data.json to write: {}",&dataToWrite);
            let _ = std::fs::write(res_path,dataToWrite);
            return YAMLChangePayload{config:c,success:true,msg:"".to_string()}
        }
    };
    //let res: YAMLChangePayload = YAMLChangePayload{config:conf,success:true,msg:"".to_string()};
}

#[tauri::command]
pub fn dir_data(path: String)->FolderData{
    let mut folders: Vec<Subfolder> = Vec::new();
    let mut files: Vec<FileData> = Vec::new();
    for entry in WalkDir::new(&path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir(){
            print!("DIR:");
            folders.push(Subfolder{
                path: entry.path().display().to_string(),
            });
        }else{
            files.push(FileData{
                path:entry.path().display().to_string(),
            });
        }
    }
    FolderData{
        path,
        files,
        folders
    }
}

fn thefilter(e: &DirEntry, name: String, extension: String, searchtype: String)->bool{
    if searchtype=="folder"{
        return e.metadata().expect("ERROR lol").file_type().is_dir();
    }
    else{
        if !e.metadata().expect("ERROR lol").file_type().is_dir() {
            if searchtype=="dotfile"{
                return e.file_name().to_str().map(|s| s.starts_with(&(".".to_owned()+&extension))).expect("ERROR lol")
            }else{
                if e.file_name().to_str().map(|s| {
                    s.starts_with(&name) && s.ends_with(&extension)
                }).expect("ERROR lol"){
                    return true;
                }
            }
        }
    }
    false
}

#[tauri::command]
pub fn search(name: String, extension: String, searchtype: String, beginfrom: String){
    let path = if beginfrom!=""{
        beginfrom
    }else{
        "C:/".to_string()
    };
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()).filter(|e| thefilter(&e,name.clone(),extension.clone(),searchtype.clone())){
        println!("{}",entry.path().display());
    }
}