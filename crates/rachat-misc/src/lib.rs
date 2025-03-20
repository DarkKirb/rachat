//! Miscellaneous rachat components
//!
//! These are all things that could be independent of rachat

pub mod id_generator;
pub mod logging;
pub mod paths;

/// The reverse domain name notation of the app, without the app name
pub const QUALIFIER: &str = "rs.chir";

/// The organization name
pub const ORGANIZATION: &str = "Raccoon Productions";

/// Application Name
pub const APPLICATION_NAME: &str = "Rachat";
