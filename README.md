# fvwm menu maker

---

## XDG ENV settings

Taken from openSUSE Gnome

```bash
XDG_CONFIG_DIRS=/etc/xdg:/usr/local/etc/xdg:/usr/etc/xdg
XDG_CONFIG_DIRS=/usr/pkg/etc/xdg/
XDG_MENU_PREFIX=gnome-
XDG_SESSION_DESKTOP=default
XDG_SESSION_TYPE=wayland
XDG_CURRENT_DESKTOP=GNOME
XDG_SESSION_CLASS=user
XDG_RUNTIME_DIR=/run/user/1000
XDG_RUNTIME_DIR=/tmp/runtime-${LOGNAME}
XDG_DATA_DIRS=/home/rgeorgia/.local/share/flatpak/exports/share:/var/lib/flatpak/exports/share:/usr/local/share/:/usr/share/
XDG_DATA_DIRS=/home/rgeorgia/.local/share/applications/
```
Taken from ArcoLinux - fvwm3 desktop

```bash
XDG_SESSION_PATH=/org/freedesktop/DisplayManager/Session1
XDG_SEAT=seat0
XDG_SESSION_DESKTOP=
XDG_SESSION_TYPE=x11
XDG_SEAT_PATH=/org/freedesktop/DisplayManager/Seat0
XDG_SESSION_CLASS=user
XDG_VTNR=2
XDG_SESSION_ID=2
XDG_RUNTIME_DIR=/run/user/1000
XDG_DATA_DIRS=/home/rgeorgia/.local/share/flatpak/exports/share:/var/lib/flatpak/exports/share:/usr/local/share:/usr/share:/var/lib/snapd/desktop
```
## Steps, Path, Roadmap

- Check XDG environment variables
  - if empty load default values
- Read applications directory
  - NetBSD - `/usr/pkg/share/applications`
  - Almost everyone else - `/usr/local/share/applications/`
  - Check local applications directory. `$HOME/.local/share/applicatons`
- Get all the files that have a .desktop extension
- Read each file
- Parse each file for specific properties
- Create a menu item based on the applications category
- Write to menu file

---

## Focus

### Application Directory

- get location of applications directory
  - NetBSD - `/usr/pkg/share/applications`
  - Almost everyone else - `/usr/local/share/applications/`
  - Verify with [XDG specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- **Verify path** exists
  - Check all locations based on XDG env variables and XDG spec
  - Check local paths
- List application directory
  - get all files with .desktop extension

### Read desktop files

- Cycle through file list and read each file
- Parse out all required (needed) data
  - Type
  - Category
  - Name
  - Exec
  - Terminal (bool)

### Create Menu

- generate structs
- organize by category
- Print meun based on fvwm spec

---

## Percolate

- Read and parse each file concurrently or in parallel
- Insert menu into .fvwmrc file
