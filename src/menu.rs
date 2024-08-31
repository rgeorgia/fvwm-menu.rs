/// menu.rs
/// holds the structs for the menus

/*
The .desktop is a toml file, or it looks like one.

    The freedesktop Desktop Entry specification provides a standard for applications to integrate
    into a desktop environment. Desktop entries are the configuration files that describe how an
    application is launched and which data it can handle. They also configure how an application
    appears in a menu with an icon, which is subject to the related menu specification standard.

Ref:
https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys

 */

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DesktopEntry {
    pub name: String,
    pub app_type: String,
    pub icon: String, // Path
    pub exec: String, // Path
    pub comment: String,
    pub categories: Vec<String>,
    pub terminal: bool,
}

