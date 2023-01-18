mod model;

use std::io::Read;
use std::fs;
use std::path::Path;
use directories::UserDirs;
use std::env;


/*
[ ] Find application directory
[ ] list dir and get all files with .desktop ext
[ ] read each file
[X] build structs for menus
[ ] build menu categories
[ ] print menu
 */




fn main() {
    let app_path = get_application_dir();

    if Path::new(app_path).exists() {
        let paths = fs::read_dir(app_path).unwrap();
        for path in paths {
            println!("{}", path.unwrap().path().display())
        }
    } else {
        print!("Cannot locate {}", &app_path);
    }

}


fn get_application_dir() -> &'static str {
    let os_name = env::consts::OS ;
    match os_name {
        "netbsd" => "/usr/pkg/share/applications/",
        _ => "/usr/local/share/applications/"
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_get_application_dir() {
        // need to mock OS, but I don't know how
        assert_eq!(get_application_dir(), "/usr/local/share/applications/") ;
    }
}