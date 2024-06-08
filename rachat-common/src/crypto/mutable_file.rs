//! Encrypted mutable files
//!
//! Mutable files are encrypted using xchacha20-poly1305, with a key generated from the root key.
//!
//! The encrypted file starts with a 24 byte nonce, followed by the encrypted data and then the 16 byte authentication tag.
//!
//! Every write to the file will generate a new nonce, to prevent finding out the difference between two consecutive writes.
//!
//!

use chacha20poly1305::{
    aead::{Aead, Payload},
    AeadCore, KeyInit, XChaCha20Poly1305, XNonce,
};
use eyre::{Context, Result};
use rand::thread_rng;
use std::path::PathBuf;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

/// Reference to a mutable data file
#[derive(Clone, Debug)]
pub struct MutableFile {
    /// Path to the file
    pub(super) path: PathBuf,
    /// The encryption key for the file
    pub(super) secret_key: chacha20poly1305::Key,
}

impl MutableFile {
    /// Writes data to the file, overwriting any existing data.
    ///
    /// # Errors
    /// This function will return an error if writing to the file fails.
    pub async fn write(&self, data: impl AsRef<[u8]> + Send) -> Result<()> {
        if let Some(path) = self.path.parent() {
            fs::create_dir_all(path).await.with_context(|| {
                format!(
                    "Creating parent directory of {} ({})",
                    self.path.display(),
                    path.display()
                )
            })?;
        }
        let data = data.as_ref();

        let cipher = XChaCha20Poly1305::new(&self.secret_key);
        let nonce = XChaCha20Poly1305::generate_nonce(thread_rng());
        let payload = cipher
            .encrypt(&nonce, data)
            .with_context(|| format!("Encrypting data for {}", self.path.display()))?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&self.path)
            .await
            .with_context(|| format!("Creating and opening file {}", self.path.display()))?;

        file.write_all(&nonce)
            .await
            .with_context(|| format!("writing nonce for {}", self.path.display()))?;
        file.write_all(&payload)
            .await
            .with_context(|| format!("writing ciphertext for {}", self.path.display()))?;

        Ok(())
    }

    /// Reads data from the file
    ///
    /// # Errors
    /// This function will return an error if reading from the file fails.
    pub async fn read(&self) -> Result<Option<Vec<u8>>> {
        let mut file = match fs::OpenOptions::new().read(true).open(&self.path).await {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(None);
                }
                Err(e).with_context(|| format!("Opening file {}", self.path.display()))?;
                unreachable!();
            }
        };
        let cipher = XChaCha20Poly1305::new(&self.secret_key);
        let mut nonce = XNonce::default();
        file.read_exact(&mut nonce)
            .await
            .with_context(|| format!("Reading nonce of file {}", self.path.display()))?;
        let mut payload = Vec::new();
        file.read_to_end(&mut payload)
            .await
            .with_context(|| format!("Reading ciphertext of file {}", self.path.display()))?;
        let payload = Payload {
            aad: &[],
            msg: &payload,
        };
        Ok(Some(cipher.decrypt(&nonce, payload).with_context(
            || format!("Decryption of file {}", self.path.display()),
        )?))
    }

    /// Deletes the file if it exists
    ///
    /// # Errors
    /// This function will return an error if deleting the file fails.
    pub(crate) async fn delete(&self) -> Result<()> {
        match fs::remove_file(&self.path).await {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(());
                }
                Err(e)
                    .with_context(|| format!("Deleting encrypted file {}.", self.path.display()))?;
                Ok(())
            }
        }
    }
}
