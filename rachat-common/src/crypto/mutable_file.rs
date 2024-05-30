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
use miette::Diagnostic;
use rand::thread_rng;
use std::path::PathBuf;
use thiserror::Error;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Error, Diagnostic, Debug)]
/// Errors that can occur during mutable file access.
pub enum MutableFileError {
    #[error("IO Error during mutable file access.")]
    #[diagnostic(code(rachat_common::crypto::mutable_file::io_error))]
    /// The file failed to read or write
    IoError(#[from] tokio::io::Error),
    #[error("Cryptographic error. The file may have been corrupted.")]
    #[diagnostic(code(rachat_common::crypto::mutable_file::crypto_error))]
    /// The file could not have been decrypted. This could be because there was corruption or malicious tampering.
    CryptoError(#[from] chacha20poly1305::aead::Error),
}

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
    pub async fn write(&self, data: impl AsRef<[u8]> + Send) -> Result<(), MutableFileError> {
        if let Some(path) = self.path.parent() {
            fs::create_dir_all(path).await?;
        }
        let data = data.as_ref();

        let cipher = XChaCha20Poly1305::new(&self.secret_key);
        let nonce = XChaCha20Poly1305::generate_nonce(thread_rng());
        let payload = cipher.encrypt(&nonce, data)?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&self.path)
            .await?;

        file.write_all(&nonce).await?;
        file.write_all(&payload).await?;

        Ok(())
    }

    /// Reads data from the file
    ///
    /// # Errors
    /// This function will return an error if reading from the file fails.
    pub async fn read(&self) -> Result<Option<Vec<u8>>, MutableFileError> {
        let mut file = match fs::OpenOptions::new().read(true).open(&self.path).await {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(None);
                }
                return Err(e.into());
            }
        };
        let cipher = XChaCha20Poly1305::new(&self.secret_key);
        let mut nonce = XNonce::default();
        file.read_exact(&mut nonce).await?;
        let mut payload = Vec::new();
        file.read_to_end(&mut payload).await?;
        let payload = Payload {
            aad: &[],
            msg: &payload,
        };
        Ok(Some(cipher.decrypt(&nonce, payload)?))
    }
}
