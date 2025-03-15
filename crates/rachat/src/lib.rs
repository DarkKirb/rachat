//! The root crate for rachat
use eyre::Result;
use tracing::info;

/// Starts the main rachat application
///
/// # Errors
///
/// This function returns an error if a fatal error occurs during startup or execution.
#[expect(clippy::unused_async, reason = "API futureproofing")]
pub async fn start() -> Result<()> {
    rachat_misc::logging::init()?;

    info!("Starting rachat…");
    info!(
        "Rachat is Free Software, released under the {} license. You can find the source code at {}.",
        env!("CARGO_PKG_LICENSE"),
        env!("CARGO_PKG_REPOSITORY")
    );

    Ok(())
}
