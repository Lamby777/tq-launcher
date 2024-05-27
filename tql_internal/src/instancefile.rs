use crate::{paths, Release};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Stuff the launcher should know about an instance.
///
/// The name is based on the folder name, so that's
/// irrelevant here.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub release: Release,
}

// write to toml file
pub fn write_info_file(instance: &str, info: InstanceInfo) -> Result<()> {
    let path = paths::instance_info_file(instance);
    let toml = toml::to_string(&info)?;
    std::fs::write(path, toml)?;

    Ok(())
}
