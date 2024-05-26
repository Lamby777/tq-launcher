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
#![feature(let_chains)]

use anyhow::{bail, Result};
use std::sync::OnceLock;

pub use octocrab::models::repos::Release;

mod paths;
mod reqs;
mod zippy;
pub use reqs::{download_release, fetch_releases};

const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";
const INSTANCES_FOLDER_NAME: &str = "instances";

// For the TerraQuest repo, not the launcher!
const TQ_REPO_OWNER: &str = "MaxineHelsel";
const TQ_REPO_NAME: &str = "TerraQuest";

// ----------------------------------------

pub async fn create_instance(name: &str, release: Release) -> Result<()> {
    let instances = paths::instances_folder();

    // check if name already used
    if instances.join(name).exists() {
        bail!("instance with that name already exists");
    }

    println!("Creating instance {} on release {:?}", name, &release.name);
    println!("Downloading release...");

    // download the bin into that folder
    download_release(name, &release).await?;
    println!("Downloaded!");

    Ok(())
}

/// Get the names of all valid instances
pub fn instance_names() -> Vec<String> {
    paths::instances_folder()
        .read_dir()
        .expect("could not read instances folder")
        .filter_map(|entry| {
            if let Ok(ref v) = entry
                && v.path().is_dir()
            {
                if let fname @ Some(_) =
                    v.file_name().to_str().map(str::to_string)
                {
                    return fname;
                }

                println!("instance folder name has invalid chars");
            } else {
                println!("could not read instance: {:?}", entry);
            };

            None
        })
        .collect()
}
