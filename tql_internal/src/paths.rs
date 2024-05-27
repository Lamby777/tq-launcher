use std::path::PathBuf;

use crate::consts::*;

/// Returns the path to do all the TQL stuff in.
/// Makes the folder if not exists
pub fn launcher_folder() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("could not find a data directory. This is a bug!");
    let path = data_dir.join(LAUNCHER_FOLDER_NAME);

    if !path.exists() {
        std::fs::create_dir_all(&path)
            .expect("could not create the launcher folder");
    }

    path
}

pub fn instances_folder() -> PathBuf {
    let path = launcher_folder().join(INSTANCES_FOLDER_NAME);

    if !path.exists() {
        std::fs::create_dir_all(&path)
            .expect("could not create the instances folder");
    }

    path
}

pub fn instance_folder(name: &str) -> PathBuf {
    instances_folder().join(name)
}

pub fn instance_info_file(instance: &str) -> PathBuf {
    instance_folder(instance).join(INSTANCE_INFO_FILE_NAME)
}

pub fn executable(instance: &str) -> PathBuf {
    let bin_name = os_bin_name();
    instance_folder(instance).join(bin_name)
}

fn os_bin_name() -> String {
    let ending = match std::env::consts::OS {
        "macos" => "Mac OS)",
        "windows" => "Windows).exe",
        "linux" => "Linux)",
        _ => unimplemented!("platform not supported"),
    };

    format!("TerraQuest ({}", ending)
}
