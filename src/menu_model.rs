use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub owner: Owner,
    pub menu: Menu,
    pub short_names: ShortNames,
    pub app_path: AppPath,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub name: String,
    pub handle: String,
}

#[derive(Deserialize, Debug)]
pub struct Menu {
    pub keys: Vec<String>,
    pub categories: Vec<String>,
    pub skip_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ShortNames {
    pub inkscape: Vec<String>,
    pub ristretto: Vec<String>,
    pub gimp: Vec<String>,
    pub xfburn: Vec<String>,
    pub pcmanfm: Vec<String>,
    pub pycharm: Vec<String>,
    pub bluefish: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AppPath {
    pub path: String
}