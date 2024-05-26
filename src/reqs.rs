//!
//! Stuff relating to making network requests, whether
//! that be downloading versions or just fetching info.
//!

use anyhow::Result;
use octocrab::models;

use crate::{TQ_REPO_NAME, TQ_REPO_OWNER};

pub async fn fetch_versions() -> Result<()> {
    let octo = octocrab::instance();
    let mut page = octo
        .repos(TQ_REPO_OWNER, TQ_REPO_NAME)
        .releases()
        .list()
        .per_page(100)
        .send()
        .await?;

    loop {
        for release in &page {
            println!("{}", release.name.as_deref().unwrap_or("Unnamed"));
        }

        let next = octo.get_page::<models::repos::Release>(&page.next).await?;
        let Some(next) = next else { break };
        page = next;
    }

    Ok(())
}

// pub fn download_version() -> Result<()> {
//     let path = paths::launcher_folder();
//     dbg!(&path);
//     log!("Downloading...");
//     Ok(())
// }
