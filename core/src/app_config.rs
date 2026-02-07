// Copyright (C) 2026 Ethan Uppal.
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

use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "kegtui.toml";

pub fn app_config_file_path() -> PathBuf {
    let config_home_guess = PathBuf::from(
        env::var("HOME").expect("User does not have $HOME directory set"),
    )
    .join(".config");

    env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or(config_home_guess)
        .join(CONFIG_FILE_NAME)
}

pub fn default_wrapper_search_paths() -> Vec<PathBuf> {
    [
        "/Applications",
        "~/Applications/",
        "~/Applications/Kegworks/",
    ]
    .into_iter()
    .map(PathBuf::from)
    .collect()
}

fn default_editor() -> String {
    env::var("EDITOR").unwrap_or("vim".into())
}

fn default_explorer() -> String {
    env::var("EXPLORER").unwrap_or("open".into())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(
        rename = "wrapper-search-paths",
        default = "default_wrapper_search_paths"
    )]
    pub wrapper_search_paths: Vec<PathBuf>,

    #[serde(default = "default_editor")]
    pub editor: String,

    #[serde(default = "default_explorer")]
    pub explorer: String,
}
