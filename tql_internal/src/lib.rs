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

use anyhow::{bail, Result};
use std::sync::OnceLock;

pub use octocrab::models::repos::Release;

mod paths;
mod reqs;
pub use reqs::{download_release, fetch_releases};

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";

// For the TerraQuest repo, not the launcher!
const TQ_REPO_OWNER: &str = "MaxineHelsel";
const TQ_REPO_NAME: &str = "TerraQuest";

// ----------------------------------------

pub fn create_instance(name: &str, release: Release) -> Result<()> {
    let instances = paths::instances_folder();

    // check if name already used
    if instances.join(name).exists() {
        bail!("instance with that name already exists");
    }

    println!("Creating instance {} on release {:?}", name, &release.name);
    println!("Downloading release...");

    // download the bin into that folder
    download_release(name, &release);
    println!("Downloaded!");

    Ok(())
}
