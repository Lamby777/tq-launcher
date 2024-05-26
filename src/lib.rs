use anyhow::Result;
use std::sync::OnceLock;

mod paths;

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";

pub static SILENT: OnceLock<bool> = OnceLock::new();

macro_rules! log {
    ($($arg:tt)*) => {
        if !SILENT.get().unwrap_or(&false) {
            eprintln!($($arg)*);
        }
    };
}

pub fn run_game(opts: TqlOptions) -> Result<()> {
    SILENT.set(opts.silent).unwrap();

    let path = paths::launcher_folder();
    dbg!(&path);
    log!("Loading...");

    Ok(())
}

pub fn download_version(opts: TqlOptions) -> Result<()> {
    SILENT.set(opts.silent).unwrap();

    let path = paths::launcher_folder();
    dbg!(&path);
    log!("Downloading...");
    Ok(())
}

pub struct TqlOptions {
    pub silent: bool,
}
