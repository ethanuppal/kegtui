// Copyright (C) 2024 Ethan Uppal.
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::keg_plist::KegPlist;

#[derive(Debug, Clone)]
pub struct Keg {
    pub name: String,
    /// The parent folder of the keg `.app` used only for display purposes.
    pub enclosing_location: PathBuf,
    pub config_file: PathBuf,
    pub wineskin_launcher: OsString,
    pub c_drive: PathBuf,
    pub log_directory: PathBuf,
    pub winetricks_logfile: PathBuf,
    pub wine_prefix: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Wrapper {
    pub path: PathBuf,
}

pub struct CurrentKeg {
    pub name: String,
    pub wineskin_launcher: OsString,
    pub c_drive: PathBuf,
    pub plist: KegPlist,
    pub config_file: PathBuf,
    pub log_directory: PathBuf,
    pub winetricks_logfile: PathBuf,
    pub wine_prefix: PathBuf,
}

impl Keg {
    pub fn from_path(path: &Path) -> Self {
        Self {
            name: path
                .file_name()
                .expect("Missing Keg name")
                .to_string_lossy()
                .to_string(),
            enclosing_location: path
                .parent()
                .expect("Missing Keg name")
                .to_path_buf(),
            config_file: path.join("Contents/Info.plist"),
            c_drive: path.join("Contents/SharedSupport/prefix/drive_c"),
            wineskin_launcher: path
                .join("Contents/MacOS/wineskinLauncher")
                .into_os_string(),
            log_directory: path.join("Contents/Logs"),
            winetricks_logfile: path
                .join("Contents/SharedSupport/Logs/Winetricks.log"),
            wine_prefix: path.join("Contents/SharedSupport/wine/bin"),
        }
    }
}

impl TryFrom<&Keg> for CurrentKeg {
    type Error = plist::Error;

    fn try_from(value: &Keg) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            wineskin_launcher: value.wineskin_launcher.clone(),
            c_drive: value.c_drive.clone(),
            plist: plist::from_file(&value.config_file)?,
            config_file: value.config_file.clone(),
            log_directory: value.log_directory.clone(),
            winetricks_logfile: value.winetricks_logfile.clone(),
            wine_prefix: value.wine_prefix.clone(),
        })
    }
}
