use anyhow::Result;
pub use octocrab::models::repos::Release;
use std::sync::OnceLock;

mod paths;
mod reqs;
pub use reqs::fetch_versions;

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";

// For the TerraQuest repo, not the launcher!
const TQ_REPO_OWNER: &str = "MaxineHelsel";
const TQ_REPO_NAME: &str = "TerraQuest";

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

pub struct TqlOptions {
    pub silent: bool,
}
