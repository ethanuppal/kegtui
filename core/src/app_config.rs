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

use std::{
    env,
    path::{Path, PathBuf},
};

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

pub fn default_keg_location() -> &'static Path {
    Path::new("~/Applications/kegtui")
}

pub fn default_keg_search_paths() -> Vec<PathBuf> {
    [
        "/Applications",
        "~/Applications/",
        "~/Applications/Kegworks/",
        "~/Applications/Sikarugir/",
        default_keg_location()
            .to_str()
            .expect("Bug: default_keg_location should be a valid UTF-8 path"),
    ]
    .into_iter()
    .map(PathBuf::from)
    .collect()
}

pub fn default_engine_search_paths() -> Vec<PathBuf> {
    [
        "~/Library/Application Support/Kegworks/Engines/",
        "~/Library/Application Support/Sikarugir/Engines/",
    ]
    .into_iter()
    .map(PathBuf::from)
    .collect()
}

pub fn default_wrapper_search_paths() -> Vec<PathBuf> {
    [
        "~/Library/Application Support/Kegworks/Wrapper/",
        "~/Library/Application Support/Sikarugir/Wrapper/",
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
    /// Directories with full Kegworks wrappers.
    #[serde(rename = "keg-search-paths", default = "default_keg_search_paths")]
    pub keg_search_paths: Vec<PathBuf>,

    /// Directories with Kegworks engines.
    #[serde(
        rename = "engine-search-paths",
        default = "default_engine_search_paths"
    )]
    pub engine_search_paths: Vec<PathBuf>,

    /// Directories with template Kegworks wrappers.
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
