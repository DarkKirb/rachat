//! Paths used by Rachat
//!
//! This is based on the `directories` crate.

use std::path::PathBuf;

use directories::ProjectDirs;
use eyre::{Context, OptionExt, Result};

/// The directories used by rachat
#[derive(Debug)]
pub struct Directories {
    /// The location where the application configuration is stored.
    config: PathBuf,
    #[cfg(test)]
    /// For tests, this contains the temporary location where the test’s configuration is stored
    _test_dir: Option<tempfile::TempDir>,
}

impl Directories {
    /// Creates an instance of the rachat directories.
    ///
    /// # Errors
    ///
    /// This function returns an error if the user’s home directory cannot be found.
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from(
            crate::QUALIFIER,
            crate::ORGANIZATION,
            crate::APPLICATION_NAME,
        )
        .ok_or_eyre("Expected to find user’s home directory")?;

        Ok(Self {
            config: project_dirs.config_dir().to_path_buf(),
            #[cfg(test)]
            _test_dir: None,
        })
    }

    /// Creates a new rachat directory for testing purposes.
    ///
    /// # Errors
    /// Returns an error
    #[cfg(test)]
    pub fn new_tmpdir() -> Result<Self> {
        let tmp = tempfile::tempdir()?;
        Ok(Self {
            config: tmp.path().join("config"),
            _test_dir: Some(tmp),
        })
    }

    /// Gets the config directory path.
    ///
    /// # Errors
    ///
    /// This function returns an error if the config directory doesn’t exist and cannot be created.
    pub async fn config(&self) -> Result<&PathBuf> {
        tokio::fs::create_dir_all(&self.config)
            .await
            .context("Creating config directory")?;
        Ok(&self.config)
    }

    /// Gets the config directory path.
    ///
    /// # Errors
    ///
    /// This function returns an error if the config directory doesn’t exist and cannot be created.
    pub fn config_sync(&self) -> Result<&PathBuf> {
        std::fs::create_dir_all(&self.config).context("Creating config directory")?;
        Ok(&self.config)
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;
    #[tokio::test]
    async fn check_directory_presence() -> Result<()> {
        let directories = super::Directories::new_tmpdir()?;
        let config = directories.config().await?;
        assert!(config.exists());
        Ok(())
    }
}
