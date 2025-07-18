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

use serde::{Deserialize, Serialize};

use crate::keg_plist::KegPlist;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TranslationConfig {
    pub d3d_metal: bool,
    pub dxvk: bool,
    pub dxmt: bool,
    pub molten_vkcx: bool,
    pub fast_math: bool,
    pub advertise_avx: bool,
    pub metal_hud: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WineConfig {
    pub wine_esync: bool,
    pub wine_msync: bool,
    pub wine_debug: String,
    pub use_start_exe: bool,
    //pub try_to_shutdown_gracefully: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WinetricksConfig {
    pub winetricks_disable_logging: bool,
    pub winetricks_force: bool,
    pub winetricks_silent: bool,
    pub skip_gecko: bool,
    pub skip_mono: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyboardConfig {
    pub map_option_to_alt: bool,
    pub map_command_to_ctrl: bool,
    pub use_standard_function_keys: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FolderMappingConfig {
    //pub map_mac_folders: bool,
    pub symlinks_in_user_folder: bool,
    pub symlink_desktop: String,
    pub symlink_downloads: String,
    pub symlink_documents: String,
    pub symlink_music: String,
    pub symlink_pictures: String,
    pub symlink_videos: String,
    pub symlink_templates: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugConfig {
    pub debug_mode: bool,
    //pub always_create_log: bool,
    pub disable_cpus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KegworksConfig {
    pub translation: TranslationConfig,
    pub wine: WineConfig,
    pub winetricks: WinetricksConfig,
    //pub keyboard: KeyboardConfig,
    pub folders: FolderMappingConfig,
    pub debug: DebugConfig,

    pub gamma_correction: String,
    pub program_flags: String,
    pub program_path: String,
}

impl KegPlist {
    pub fn update_from_config(&mut self, config: &KegworksConfig) {
        self.advertise_avx = config.translation.advertise_avx;
        self.d3d_metal = config.translation.d3d_metal;
        self.dxvk = config.translation.dxvk;
        self.dxmt = config.translation.dxmt;
        self.molten_vkcx = config.translation.molten_vkcx;
        self.fast_math = config.translation.fast_math;
        self.metal_hud = config.translation.metal_hud;

        self.wine_esync = config.wine.wine_esync;
        self.wine_msync = config.wine.wine_msync;
        self.wine_debug = config.wine.wine_debug.clone();
        self.use_start_exe = config.wine.use_start_exe;

        self.winetricks_disable_logging =
            config.winetricks.winetricks_disable_logging;
        self.winetricks_force = config.winetricks.winetricks_force;
        self.winetricks_silent = config.winetricks.winetricks_silent;
        self.skip_gecko = config.winetricks.skip_gecko;
        self.skip_mono = config.winetricks.skip_mono;

        self.symlinks_in_user_folder = config.folders.symlinks_in_user_folder;
        self.symlink_desktop = config.folders.symlink_desktop.clone();
        self.symlink_downloads = config.folders.symlink_downloads.clone();
        self.symlink_my_documents = config.folders.symlink_documents.clone();
        self.symlink_my_music = config.folders.symlink_music.clone();
        self.symlink_my_pictures = config.folders.symlink_pictures.clone();
        self.symlink_my_videos = config.folders.symlink_videos.clone();
        self.symlink_templates = config.folders.symlink_templates.clone();

        self.debug_mode = config.debug.debug_mode;
        self.disable_cpus = config.debug.disable_cpus;

        self.gamma_correction = config.gamma_correction.clone();
        self.program_flags = config.program_flags.clone();
        self.program_name_and_path = config.program_path.clone();
    }

    // Extract config from plist
    pub fn extract_config(&self) -> KegworksConfig {
        KegworksConfig {
            translation: TranslationConfig {
                advertise_avx: self.advertise_avx,
                d3d_metal: self.d3d_metal,
                dxvk: self.dxvk,
                dxmt: self.dxmt,
                molten_vkcx: self.molten_vkcx,
                fast_math: self.fast_math,
                metal_hud: self.metal_hud,
            },
            wine: WineConfig {
                wine_esync: self.wine_esync,
                wine_msync: self.wine_msync,
                wine_debug: self.wine_debug.clone(),
                use_start_exe: self.use_start_exe,
            },
            winetricks: WinetricksConfig {
                winetricks_disable_logging: self.winetricks_disable_logging,
                winetricks_force: self.winetricks_force,
                winetricks_silent: self.winetricks_silent,
                skip_gecko: self.skip_gecko,
                skip_mono: self.skip_mono,
            },
            folders: FolderMappingConfig {
                symlinks_in_user_folder: self.symlinks_in_user_folder,
                symlink_desktop: self.symlink_desktop.clone(),
                symlink_downloads: self.symlink_downloads.clone(),
                symlink_documents: self.symlink_my_documents.clone(),
                symlink_music: self.symlink_my_music.clone(),
                symlink_pictures: self.symlink_my_pictures.clone(),
                symlink_videos: self.symlink_my_videos.clone(),
                symlink_templates: self.symlink_templates.clone(),
            },
            debug: DebugConfig {
                debug_mode: self.debug_mode,
                disable_cpus: self.disable_cpus,
            },
            gamma_correction: self.gamma_correction.clone(),
            program_flags: self.program_flags.clone(),
            program_path: self.program_name_and_path.clone(),
        }
    }
}
