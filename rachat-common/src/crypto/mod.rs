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
    pub fn new() -> Self {
        Self(rand::thread_rng().gen())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
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

    pub async fn load_from_keyring(profile: impl Display) -> Result<Self> {
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
