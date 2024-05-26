use std::path::PathBuf;

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
