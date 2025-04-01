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

use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod bool_as_int {
    use super::*;

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(if *value { 1 } else { 0 })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(value != 0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KegworksPlist {
    #[serde(rename = "ADVERTISE_AVX")]
    #[serde(with = "bool_as_int")]
    pub advertise_avx: bool,

    #[serde(rename = "Associations")]
    pub associations: String,

    #[serde(rename = "CFBundleDevelopmentRegion")]
    pub cf_bundle_development_region: String,

    #[serde(rename = "CFBundleDocumentTypes")]
    pub cf_bundle_document_types: Vec<CFBundleDocumentType>,

    #[serde(rename = "CFBundleExecutable")]
    pub cf_bundle_executable: String,

    #[serde(rename = "CFBundleIconFile")]
    pub cf_bundle_icon_file: String,

    #[serde(rename = "CFBundleIdentifier")]
    pub cf_bundle_identifier: String,

    #[serde(rename = "CFBundleInfoDictionaryVersion")]
    pub cf_bundle_info_dictionary_version: String,

    #[serde(rename = "CFBundleName")]
    pub cf_bundle_name: String,

    #[serde(rename = "CFBundlePackageType")]
    pub cf_bundle_package_type: String,

    #[serde(rename = "CFBundleShortVersionString")]
    pub cf_bundle_short_version_string: String,

    #[serde(rename = "CFBundleVersion")]
    pub cf_bundle_version: String,

    #[serde(rename = "CLI Custom Commands")]
    pub cli_custom_commands: String,

    #[serde(rename = "CSResourcesFileMapped")]
    pub cs_resources_file_mapped: bool,

    #[serde(rename = "D3DMETAL")]
    #[serde(with = "bool_as_int")]
    pub d3d_metal: bool,

    #[serde(rename = "DXMT")]
    #[serde(with = "bool_as_int")]
    pub dxmt: bool,

    #[serde(rename = "DXVK")]
    #[serde(with = "bool_as_int")]
    pub dxvk: bool,

    #[serde(rename = "Debug Mode")]
    #[serde(with = "bool_as_int")]
    pub debug_mode: bool,

    #[serde(rename = "Disable CPUs")]
    #[serde(with = "bool_as_int")]
    pub disable_cpus: bool,

    #[serde(rename = "FASTMATH")]
    #[serde(with = "bool_as_int")]
    pub fast_math: bool,

    #[serde(rename = "Gamma Correction")]
    pub gamma_correction: String,

    #[serde(rename = "LSMinimumSystemVersion")]
    pub ls_minimum_system_version: String,

    #[serde(rename = "METAL_HUD")]
    #[serde(with = "bool_as_int")]
    pub metal_hud: bool,

    #[serde(rename = "MOLTENVKCX")]
    #[serde(with = "bool_as_int")]
    pub molten_vkcx: bool,

    #[serde(rename = "NSAppTransportSecurity")]
    pub ns_app_transport_security: NSAppTransportSecurity,

    #[serde(rename = "NSBGOnly")]
    pub ns_bg_only: String,

    #[serde(rename = "NSBluetoothAlwaysUsageDescription")]
    pub ns_bluetooth_always_usage_description: String,

    #[serde(rename = "NSBluetoothPeripheralUsageDescription")]
    pub ns_bluetooth_peripheral_usage_description: String,

    #[serde(rename = "NSCameraUsageDescription")]
    pub ns_camera_usage_description: String,

    #[serde(rename = "NSDesktopFolderUsageDescription")]
    pub ns_desktop_folder_usage_description: String,

    #[serde(rename = "NSDocumentsFolderUsageDescription")]
    pub ns_documents_folder_usage_description: String,

    #[serde(rename = "NSDownloadsFolderUsageDescription")]
    pub ns_downloads_folder_usage_description: String,

    #[serde(rename = "NSMainNibFile")]
    pub ns_main_nib_file: String,

    #[serde(rename = "NSMicrophoneUsageDescription")]
    pub ns_microphone_usage_description: String,

    #[serde(rename = "NSNetworkVolumesUsageDescription")]
    pub ns_network_volumes_usage_description: String,

    #[serde(rename = "NSPrincipalClass")]
    pub ns_principal_class: String,

    #[serde(rename = "NSRemovableVolumesUsageDescription")]
    pub ns_removable_volumes_usage_description: String,

    #[serde(rename = "Program Flags")]
    pub program_flags: String,

    #[serde(rename = "Program Name and Path")]
    pub program_name_and_path: String,

    #[serde(rename = "Skip Gecko")]
    #[serde(with = "bool_as_int")]
    pub skip_gecko: bool,

    #[serde(rename = "Skip Mono")]
    #[serde(with = "bool_as_int")]
    pub skip_mono: bool,

    #[serde(rename = "Symlink Desktop")]
    pub symlink_desktop: String,

    #[serde(rename = "Symlink Downloads")]
    pub symlink_downloads: String,

    #[serde(rename = "Symlink My Documents")]
    pub symlink_my_documents: String,

    #[serde(rename = "Symlink My Music")]
    pub symlink_my_music: String,

    #[serde(rename = "Symlink My Pictures")]
    pub symlink_my_pictures: String,

    #[serde(rename = "Symlink My Videos")]
    pub symlink_my_videos: String,

    #[serde(rename = "Symlink Templates")]
    pub symlink_templates: String,

    #[serde(rename = "Symlinks In User Folder")]
    #[serde(with = "bool_as_int")]
    pub symlinks_in_user_folder: bool,

    #[serde(rename = "Try To Use GPU Info")]
    #[serde(with = "bool_as_int")]
    pub try_to_use_gpu_info: bool,

    #[serde(rename = "WINEDEBUG")]
    pub wine_debug: String,

    #[serde(rename = "WINEESYNC")]
    #[serde(with = "bool_as_int")]
    pub wine_esync: bool,

    #[serde(rename = "WINEMSYNC")]
    #[serde(with = "bool_as_int")]
    pub wine_msync: bool,

    #[serde(rename = "Winetricks disable logging")]
    #[serde(with = "bool_as_int")]
    pub winetricks_disable_logging: bool,

    #[serde(rename = "Winetricks force")]
    #[serde(with = "bool_as_int")]
    pub winetricks_force: bool,

    #[serde(rename = "Winetricks silent")]
    #[serde(with = "bool_as_int")]
    pub winetricks_silent: bool,

    #[serde(rename = "use start.exe")]
    #[serde(with = "bool_as_int")]
    pub use_start_exe: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFBundleDocumentType {
    #[serde(rename = "CFBundleTypeExtensions")]
    pub cf_bundle_type_extensions: Vec<String>,

    #[serde(rename = "CFBundleTypeRole")]
    pub cf_bundle_type_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NSAppTransportSecurity {
    #[serde(rename = "NSAllowsArbitraryLoads")]
    pub ns_allows_arbitrary_loads: bool,
}

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

impl KegworksPlist {
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
