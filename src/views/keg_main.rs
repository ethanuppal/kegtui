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

use std::{env, fs, process::Command};

use crate::{
    app::{App, AsyncState},
    view::prelude::*,
};

pub struct KegMainView;

impl View for KegMainView {}

pub fn open_c_drive(app: &mut App, _state: &AsyncState) -> Result<()> {
    let Some(current_keg) = &app.current_keg else {
        return Ok(());
    };
    if let Ok(explorer) = env::var("EXPLORER") {
        Command::new(explorer)
            .arg(current_keg.c_drive.to_string_lossy().to_string())
            .status()?;
    } else {
        Command::new("open")
            .arg(current_keg.c_drive.to_string_lossy().to_string())
            .status()?;
    }
    Ok(())
}

pub fn edit_config(app: &mut App, _state: &AsyncState) -> Result<()> {
    if let Some(current_keg) = &mut app.current_keg {
        let toml_config =
            toml::to_string_pretty(&current_keg.plist.extract_config())?;
        let file = "/tmp/kegtui.toml";
        fs::write(file, toml_config)?;
        let editor = env::var("EDITOR").unwrap_or("vim".into());
        Command::new(editor).arg(file).status()?;
        let new_toml_config = toml::from_str(&fs::read_to_string(file)?)?;
        current_keg.plist.update_from_config(&new_toml_config);
        plist::to_file_xml(&current_keg.config_file, &current_keg.plist)?;
    }
    Ok(())
}
