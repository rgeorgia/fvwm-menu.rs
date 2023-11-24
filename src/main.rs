mod menu;

use std::fs::DirEntry;
use std::path::Path;
/*
[X] Read $HOME/.local/share/applications
[I] List all files with .desktop ext
[ ] read each file
[ ] build structs for menus
[ ] build menu categories
[ ] print menu
 */


fn main() {

    let app_dir = Path::new("/home/rgeorgia/.local/share/applications");

    let files = list_app_dir(&app_dir);
    for name in files.iter() {
        println!("{:?}", name.path());
    }
}

fn list_app_dir(dir_path: &Path) -> Vec<DirEntry>{
    let mut files = Vec::new();
    for entry in dir_path.read_dir().expect("Failure reading dir") {
        if let Ok(entry) = entry {
            files.push(entry);
        }
    }
    files
}
fn read_files(){}

fn build_menu(){}

fn write_menu(){}