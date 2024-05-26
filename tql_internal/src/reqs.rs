//!
//! Stuff relating to making network requests, whether
//! that be downloading versions or just fetching info.
//!

use core::panic;
use std::path::Path;

use anyhow::Result;
use octocrab::models::repos::{Asset, Release};
use octocrab::models::{self};

use crate::{paths, TQ_REPO_NAME, TQ_REPO_OWNER};

pub async fn fetch_releases() -> Result<Vec<Release>> {
    let octo = octocrab::instance();
    let mut page = octo
        .repos(TQ_REPO_OWNER, TQ_REPO_NAME)
        .releases()
        .list()
        .per_page(100)
        .send()
        .await?;

    let mut res = vec![];

    loop {
        for release in &page {
            // dbg!(&release.assets);
            res.push(release.clone());
        }

        let next = octo.get_page::<models::repos::Release>(&page.next).await?;
        let Some(next) = next else { break };
        page = next;
    }

    Ok(res)
}

/// Download a specific release binary to an instance folder.
/// If a binary already exists, it will be overwritten.
pub fn download_release(instance_name: &str, release: &Release) -> Result<()> {
    let instances = paths::instances_folder();

    let asset = get_os_asset(&release.assets);

    Ok(())
}

/// filter out the binaries that are not for the current OS
fn get_os_asset(assets: &[Asset]) -> Asset {
    // Get the name of the OS used in release name parentheses
    let os_paren = match std::env::consts::OS {
        "macos" => "Mac OS",
        "windows" => "Windows",
        "linux" => "Linux",
        _ => unimplemented!("platform not supported"),
    };

    let Some(found) = assets.iter().find(|a| a.name.contains(&os_paren)) else {
        let asset_names = assets.iter().map(|a| &a.name).collect::<Vec<_>>();
        panic!(
            "no asset found for OS {:?}. available asset names: {:?}",
            os_paren, asset_names
        );
    };

    found.clone()
}
