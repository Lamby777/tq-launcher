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
pub use reqs::fetch_releases;

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";

// For the TerraQuest repo, not the launcher!
const TQ_REPO_OWNER: &str = "MaxineHelsel";
const TQ_REPO_NAME: &str = "TerraQuest";

// ----------------------------------------

pub fn create_instance(name: &str, version: Release) -> Result<()> {
    // check if version exists
    // check if name already used
    // clone the repo into that folder
    let instances = paths::instances_folder();

    println!("Creating instance {} with version {:?}", name, version.name);

    Ok(())
}
