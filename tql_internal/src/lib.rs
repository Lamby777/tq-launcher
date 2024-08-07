//!
//! Launcher for TerraQuest. Pretty much runs git commands
//! or moves files all under the hood, but with a fancy
//! user interface for players.
//!
//! Don't expect much yet, it's still in development.
//!
//! \- &Cherry, 2024
//!
#![feature(let_chains)]

use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fs;

pub use octocrab::models::repos::Release;

mod instancefile;
mod paths;
mod reqs;
mod zippy;
pub use instancefile::InstanceInfo;
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

pub fn alter_instance(name: &str, flags: &str) -> Result<(), &'static str> {
    let mut instance = InstanceInfo::from_name(name).unwrap();

    println!("new flags: {}", flags);
    instance.boot_flags = shlex::split(flags).ok_or("could not parse flags")?;

    // TODO rename existing folder before writing data if name changed
    instance.write_info(name).unwrap();
    Ok(())
}

pub async fn create_instance(name: &str, release: Release) -> Result<()> {
    println!("Creating instance {} on release {:?}", name, &release.name);
    let instances = paths::instances_folder();

    // check if name already used
    if instances.join(name).exists() {
        bail!("Instance with that name already exists!");
    }

    println!("Downloading release...");
    // download the bin into that folder
    download_release(name, &release).await?;

    println!("Downloaded! Writing instance info file...");

    let release_id = *release.id;
    let instance = InstanceInfo {
        release_id,
        boot_flags: vec![],
    };
    instance.write_info(name)?;

    Ok(())
}

pub fn delete_instance(name: &str) -> Result<(), &'static str> {
    println!("Deleting instance {}", name);

    let instance = paths::instance_folder(name);
    if !instance.exists() {
        return Err("instance does not exist");
    }

    fs::remove_dir_all(&instance).expect("could not remove instance folder");
    Ok(())
}

/// Get the names of all valid instances
pub fn instance_map() -> HashMap<String, InstanceInfo> {
    paths::instances_folder()
        .read_dir()
        .expect("could not read instances folder")
        .filter_map(|entry| {
            let Ok(ref v) = entry else {
                eprintln!("failed to read instance folder {:?}", entry);
                return None;
            };

            // skip folders silently
            if !v.path().is_dir() {
                return None;
            }

            let Some(fname) = v.file_name().to_str().map(str::to_string) else {
                eprintln!("instance folder name has invalid chars");
                return None;
            };

            let info = InstanceInfo::from_name(&fname).ok()?;
            Some((fname, info))
        })
        .collect()
}

pub fn play_instance(name: &str) -> Result<(), &'static str> {
    println!("Playing instance {}", name);

    let instance = paths::instance_folder(name);
    if !instance.exists() {
        return Err("instance does not exist");
    }

    let Ok(bin) = paths::executable(name).canonicalize() else {
        return Err("instance has no executable");
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;

        // set executable bit
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755))
            .expect("could not set executable bit");
    }

    // run the bin
    std::process::Command::new(bin)
        .arg("windowed")
        .spawn()
        .expect("could not run the instance");

    Ok(())
}
