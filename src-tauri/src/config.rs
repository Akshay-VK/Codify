use serde::{Deserialize, Serialize};

use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub baseLocation: String,
    pub folders: Vec<Folder>,
    pub actions: Vec<Action>,
    pub procedures: Vec<Template>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub name: String,
    pub folder: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub name: String,
    pub arguments: Vec<String>,
    pub commands: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub arguments: Vec<String>,
    pub commands: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStore {
    pub pathToConfig: String,
}

pub struct Data {
    pub data: Mutex<DataStore>,
    pub commandCount:Mutex<i32>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct FolderData{
    pub path: String,
    pub files: Vec<FileData>,
    pub folders: Vec<Subfolder>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Subfolder{
    pub path: String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct FileData{
    pub path: String
}