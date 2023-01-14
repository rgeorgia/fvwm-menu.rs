mod menu_model;

use std::io::Read;
use std::fs;
use std::path::Path;
use directories::UserDirs;

/*
[ ] Find application directory
[ ] list dir and get all files with .desktop ext
[ ] read each file
[X] build structs for menus
[ ] build menu categories
[ ] print menu
 */


fn main() {
    let file_name = ".fvwmrc";
    let menu_config = config(file_name);

    if Path::new(&menu_config.app_path.path).exists() {
        let paths = fs::read_dir(menu_config.app_path.path).unwrap();
        for path in paths {
            println!("{}", path.unwrap().path().display())
        }
    } else {
        print!("Cannot locate {}", &menu_config.app_path.path);
    }
    if let Some(users_dir) = UserDirs::new() {
        println!("Home: {:?}\n", users_dir.home_dir());
    }
}

fn config(file_name: &str) -> menu_model::Config {
    let mut file = std::fs::File::open(&file_name).expect("Error opening File");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: menu_model::Config = toml::from_str(&contents).unwrap();
    config
}
