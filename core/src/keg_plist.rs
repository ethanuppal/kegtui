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
pub struct KegPlist {
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
    #[serde(default)]
    pub use_start_exe: bool,

    #[serde(rename = "CNC_DDRAW")]
    #[serde(with = "bool_as_int")]
    #[serde(default)]
    pub cnc_ddraw: bool,

    #[serde(rename = "D9VK")]
    #[serde(with = "bool_as_int")]
    pub d9vk: bool,
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
