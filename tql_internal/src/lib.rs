//!
//! Launcher for TerraQuest. Pretty much runs git commands
//! or moves files all under the hood, but with a fancy
//! user interface for players.
//!
//! Don't expect much yet, it's still in development.
//!
//! \- &Cherry, 2024
//!
#![allow(unused)]

use anyhow::Result;
use std::sync::OnceLock;

pub use octocrab::models::repos::Release;

mod paths;
mod reqs;
pub use reqs::fetch_versions;

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";

// For the TerraQuest repo, not the launcher!
const TQ_REPO_OWNER: &str = "MaxineHelsel";
const TQ_REPO_NAME: &str = "TerraQuest";

// pub static SILENT: OnceLock<bool> = OnceLock::new();
//
// macro_rules! log {
//     ($($arg:tt)*) => {
//         if !SILENT.get().unwrap_or(&false) {
//             eprintln!($($arg)*);
//         }
//     };
// }
//
// pub struct TqlOptions {
//     pub silent: bool,
// }
