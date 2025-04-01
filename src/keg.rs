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
    name: String,
    config_file: PathBuf,
    wineskin_launcher: OsString,
    c_drive: PathBuf,
}

pub struct CurrentKeg {
    name: String,
    wineskin_launcher: OsString,
    c_drive: PathBuf,
    plist: KegPlist,
    config_file: PathBuf,
}

impl Keg {
    pub fn from_path(path: &Path) -> Self {
        Self {
            name: path
                .file_name()
                .expect("Missing Keg name")
                .to_string_lossy()
                .to_string(),
            config_file: path.join("Contents/Info.plist"),
            c_drive: path.join("Contents/drive_c"),
            wineskin_launcher: path
                .join("Contents/MacOS/wineskinLauncher")
                .into_os_string(),
        }
    }
}
