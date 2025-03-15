use eyre::Result;

/// Starts the main rachat application
pub async fn start() -> Result<()> {
    rachat_misc::logging::init()?;
    Ok(())
}
