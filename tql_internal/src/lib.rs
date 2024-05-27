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
use std::fs;
use std::sync::OnceLock;

pub use octocrab::models::repos::Release;

mod paths;
mod reqs;
mod zippy;
pub use reqs::{download_release, fetch_releases};

mod consts {
    pub const LAUNCHER_FOLDER_NAME: &str = "tq-launcher";
    pub const INSTANCES_FOLDER_NAME: &str = "instances";
    pub const INSTANCE_INFO_FILE_NAME: &str = "instance.toml";

    // For the TerraQuest repo, not the launcher!
    pub const TQ_REPO_OWNER: &str = "MaxineHelsel";
    pub const TQ_REPO_NAME: &str = "TerraQuest";
}

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

pub fn play_instance(name: &str) -> Result<(), &'static str> {
    println!("Playing instance {}", name);

    let instance = paths::instance_folder(name);
    if !instance.exists() {
        return Err("instance does not exist");
    }

    let bin = paths::executable(name).canonicalize().unwrap();
    if !bin.exists() {
        return Err("instance has no executable");
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;

        // set executable bit
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755))
            .expect("could not set executable bit");
    }

    // run the bin
    std::process::Command::new(bin)
        .spawn()
        .expect("could not run the instance");

    Ok(())
}
