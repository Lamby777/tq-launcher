use std::path::PathBuf;

use crate::INSTANCES_FOLDER_NAME;

/// Returns the path to do all the TQL stuff in.
/// Makes the folder if not exists
pub fn launcher_folder() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Could not find a data directory. This is a bug!");
    let path = data_dir.join(crate::LAUNCHER_FOLDER_NAME);

    if !path.exists() {
        std::fs::create_dir_all(&path)
            .expect("Could not create the launcher folder");
    }

    path
}

pub fn instances_folder() -> PathBuf {
    launcher_folder().join(INSTANCES_FOLDER_NAME)
}

pub fn os_bin_name() -> String {
    let ending = match std::env::consts::OS {
        "macos" => "Mac OS)",
        "windows" => "Windows).exe",
        "linux" => "Linux)",
        _ => unimplemented!("platform not supported"),
    };

    format!("TerraQuest ({}", ending)
}
