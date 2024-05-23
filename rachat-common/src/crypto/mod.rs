//! Root cryptography module
use std::{
    fmt::{Debug, Display},
    path::Path,
};

use anyhow::Result;
use keyring::Entry;
use rand::{distributions::Alphanumeric, CryptoRng, Rng, SeedableRng};
use secrecy::{ExposeSecret, Secret, Zeroize};

use self::mutable_file::MutableFile;

pub mod mutable_file;

/// 256 bit key derivation key. This is used as the IKM of a KDF.
#[derive(Clone, Debug)]
pub struct KDFSecretKey(Secret<[u8; 32]>);

impl KDFSecretKey {
    /// Generates a random new 256 key.
    ///
    /// This is intended to be the root key for the key hierarchy.
    #[must_use]
    pub fn new() -> Self {
        let mut key = rand::thread_rng().r#gen();
        Self::from_bytes(&mut key)
    }

    /// Creates a new secret key from 32 bytes
    #[must_use]
    fn from_bytes(bytes: &mut [u8; 32]) -> Self {
        let res = Self(Secret::new(*bytes));
        bytes.zeroize();
        res
    }

    /// Generates a KDF child key.
    ///
    /// The purpose must be unique for each different subkey.
    ///
    /// The purpose may not include any secret information, including other keys.
    #[must_use]
    pub fn generate_kdf_subkey(&self, purpose: impl Display) -> Self {
        let context = format!("rs.chir.rachat.crypto: {purpose}");
        let mut blake_key = blake3::derive_key(&context, self.0.expose_secret());
        Self::from_bytes(&mut blake_key)
    }

    /// Generates a seeded CSPRNG with specified purpose.
    /// `
    /// From the same root key and subkey, it will generate the same CSPRNG every time.`
    #[must_use]
    pub fn subkey_rng(&self, purpose: impl Display) -> impl CryptoRng + Rng {
        let subkey = self.generate_kdf_subkey(purpose);
        rand_chacha::ChaChaRng::from_seed(*subkey.0.expose_secret())
    }

    /// Generates an alphanumeric passphrase with specified purpose.
    #[must_use]
    pub fn subkey_passphrase(&self, purpose: impl Display) -> Secret<String> {
        let secret = self
            .subkey_rng(purpose)
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        Secret::new(secret)
    }

    /// Attempts to load the root key from the keyring from a specific profile.
    ///
    /// If it doesn’t exist, it will generate a new one and store it in the keyring.
    ///
    /// # Errors
    /// This function will return an error if accessing the keyring fails.
    ///
    /// Reasons for that may include:
    ///
    /// - The keyring does not exist.
    /// - The keyring is closed
    /// - The user has rejected access to the keyring.
    /// - There is some sort of IO error preventing the keyring from working.
    pub async fn load_from_keyring(profile: impl Display + Send) -> Result<Self> {
        let profile = format!("{profile}");
        let mut secret_json = tokio::task::spawn_blocking(move || -> Result<String> {
            let entry = Entry::new("rs.chir.rachat", &format!("{profile}-key"))?;
            match entry.get_password() {
                Ok(entry) => Ok(entry),
                Err(keyring::Error::NoEntry) => {
                    let secret = Self::new();
                    let secret_json = serde_json::to_string(secret.0.expose_secret())?;
                    entry.set_password(&secret_json)?;
                    Ok(secret_json)
                }
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        })
        .await??;

        let mut key = serde_json::from_str(&secret_json)?;
        secret_json.zeroize();
        Ok(Self::from_bytes(&mut key))
    }

    /// Returns a handle to a mutable data file
    ///
    /// This data file will be encrypted on disk
    pub fn open_mutable_file(
        &self,
        data_path: impl AsRef<Path>,
        subdir: impl AsRef<Path>,
    ) -> MutableFile {
        let subdir = subdir.as_ref();
        let subdir_key_id = crate::utils::path_to_bytes(subdir)
            .into_iter()
            .map(|c| c as char)
            .collect::<String>();
        let context = format!("rs.chir.rachat.crypto.file: {subdir_key_id}");
        let mut blake_key = blake3::derive_key(&context, self.0.expose_secret());
        let res = MutableFile {
            path: data_path.as_ref().join(subdir_key_id),
            secret_key: chacha20poly1305::Key::from(blake_key),
        };
        blake_key.zeroize();
        res
    }
}

impl Default for KDFSecretKey {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    #[test]
    fn test_passphrase_stability() {
        let mut rk = [0u8; 32];
        let rk = super::KDFSecretKey::from_bytes(&mut rk);
        assert_eq!(
            rk.subkey_passphrase("test").expose_secret(),
            "MH0ldlHJ0EyUjkxmOYfUutnktw7lTdYD"
        );
    }
}
