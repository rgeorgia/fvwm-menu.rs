mod menu;

use std::fs::{DirEntry, File};
use std::io::{BufReader, Read};
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn main() {

    let app_dir = Path::new("/home/rgeorgia/.local/share/applications");

    let files = list_app_dir(&app_dir);
    for name in files.iter() {
        let f = name.path();
        read_file_to_struct(f).expect("Error reading file");
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
fn read_file_to_struct(file_to_read: PathBuf) -> std::io::Result<()> {
    let file = File::open(file_to_read.to_str().unwrap().trim())?;
    let reader = BufReader::new(file);

    println!("***************** {}", file_to_read.display());
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn build_menu(){}

fn write_menu(){}