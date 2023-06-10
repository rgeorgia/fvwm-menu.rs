/// model.menu.rs
/// holds the structs for the menus

/*
The .desktop is a toml file, or it looks like one.

    The freedesktop Desktop Entry specification provides a standard for applications to integrate
    into a desktop environment. Desktop entries are the configuration files that describe how an
    application is launched and which data it can handle. They also configure how an application
    appears in a menu with an icon, which is subject to the related menu specification standard.

Ref:
https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys


EXAMPLE RAW FILE
======================================
[Desktop Entry]
Comment=An easy to use bible study tool
Comment[C]=An easy to use bible study tool
Comment[cs]=Snadno použitelný nástroj pro studium Bible
Comment[da]=Et letanvendeligt bibelstudieprogram
Comment[de]=Ein einfach zu benutzendes Bibelprogramm
Comment[et]=Lihtne Piibliuurimise vahend
Comment[fi]=Helppokäyttöinen raamatunlukuohjelmisto
Comment[fr]=Un outil d'étude biblique facile à utiliser
Comment[it]=Un semplice strumento per studiare la Bibbia
Exec=bibletime
Icon=bibletime
Name=BibleTime
Name[C]=BibleTime
Name[cs]=BibleTime
Name[da]=Bibletime
Name[de]=BibleTime
Name[et]=BibleTime
Name[fr]=BibleTime
Name[it]=BibleTime
GenericName=Bible Study Tool
Terminal=false
Type=Application
Categories=X-Bible;X-Religion;Literature;Education;Dictionary;Qt;
Keywords=bible;study;religion;literature;education;
DocPath=bibletime/handbook/index.html

WHAT WE WANT
======================================
Comment=An easy to use bible study tool
Exec=bibletime
Icon=bibletime
Name=BibleTime
GenericName=Bible Study Tool
Terminal=false
Type=Application
Categories=X-Bible;X-Religion;Literature;Education;Dictionary;Qt;
Keywords=bible;study;religion;literature;education;
 */

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Menu {
    pub comment: String,
    pub exec: String,
    pub icon: String,
    pub name: String,
    pub generic_name: String,
    pub terminal: bool,
    pub app_type: String,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
}