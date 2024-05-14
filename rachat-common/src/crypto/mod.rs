use std::fmt::{Debug, Display};

use anyhow::Result;
use keyring::Entry;
use rand::{distributions::Alphanumeric, CryptoRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RootKey([u8; 32]);

impl Debug for RootKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RootKey").field(&"opaque").finish()
    }
}

impl RootKey {
    #[must_use]
    pub fn new() -> Self {
        Self(rand::thread_rng().r#gen())
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn generate_subkey(&self, purpose: impl Display) -> [u8; 32] {
        let context = format!("rs.chir.rachat.crypto: {purpose}");
        blake3::derive_key(&context, &self.0)
    }

    pub fn subkey_rng(&self, purpose: impl Display) -> impl CryptoRng + Rng {
        let subkey = self.generate_subkey(purpose);
        rand_chacha::ChaChaRng::from_seed(subkey)
    }

    pub fn subkey_passphrase(&self, purpose: impl Display) -> String {
        self.subkey_rng(purpose)
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
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
        let secret_json = tokio::task::spawn_blocking(move || -> Result<String> {
            let entry = Entry::new("rs.chir.rachat", &format!("{profile}-key"))?;
            match entry.get_password() {
                Ok(entry) => Ok(entry),
                Err(keyring::Error::NoEntry) => {
                    let secret = Self::new();
                    let secret_json = serde_json::to_string(&secret)?;
                    entry.set_password(&secret_json)?;
                    Ok(secret_json)
                }
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        })
        .await??;

        Ok(serde_json::from_str(&secret_json)?)
    }
}

impl From<[u8; 32]> for RootKey {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Default for RootKey {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_passphrase_stability() {
        let rk = super::RootKey::from([0u8; 32]);
        assert_eq!(
            rk.subkey_passphrase("test"),
            "MH0ldlHJ0EyUjkxmOYfUutnktw7lTdYD"
        );
    }
}
