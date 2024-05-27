use crate::paths;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Stuff the launcher should know about an instance.
///
/// The name is based on the folder name, so that's
/// irrelevant here.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub release_id: u64,
    pub release_name: String,
}

impl InstanceInfo {
    // write to toml file
    pub fn write_info(&self, instance: &str) -> Result<()> {
        let path = paths::instance_info_file(instance);
        let toml = toml::to_string(&self)?;
        std::fs::write(path, toml)?;

        Ok(())
    }
}
