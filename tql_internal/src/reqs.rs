//!
//! Stuff relating to making network requests, whether
//! that be downloading versions or just fetching info.
//!

use std::io::Cursor;

use anyhow::Result;
use octocrab::models;

use crate::consts::*;
use crate::{paths, Release};

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
pub async fn download_release(
    instance_name: &str,
    release: &Release,
) -> Result<()> {
    let asset = release.assets.first().expect("no assets found");
    let url = asset.browser_download_url.clone();

    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;

    let instance_folder = paths::instances_folder().join(instance_name);
    crate::zippy::extract_zip(&mut Cursor::new(content), &instance_folder)?;

    // std::io::copy(&mut content, &mut dest)?;
    Ok(())
}
