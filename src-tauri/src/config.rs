use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub baseLocation: String,
    pub folders: Vec<Folder>,
    pub actions: Vec<Action>,
    pub projectTemplates: Vec<Template>,
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
