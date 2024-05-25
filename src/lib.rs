use anyhow::Result;
use std::path::PathBuf;
use std::sync::OnceLock;

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";

pub(crate) static SILENT: OnceLock<bool> = OnceLock::new();

macro_rules! log {
    ($($arg:tt)*) => {
        if !SILENT.get().unwrap_or(&false) {
            eprintln!($($arg)*);
        }
    };
}

pub fn run(opts: TqlOptions) -> Result<()> {
    SILENT.set(opts.silent).unwrap();

    let path = launcher_folder();
    dbg!(&path);
    log!("Loading...");

    Ok(())
}

pub struct TqlOptions {
    pub silent: bool,
}

/// Returns the path to do all the TQL stuff in.
/// Makes the folder if not exists
fn launcher_folder() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Could not find a data directory. This is a bug!");
    let path = data_dir.join(LAUNCHER_FOLDER_NAME);

    if !path.exists() {
        std::fs::create_dir_all(&path)
            .expect("Could not create the launcher folder");
    }

    path
}
