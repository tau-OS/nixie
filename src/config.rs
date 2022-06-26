// Edit build.rs to modify these variables

pub const APP_ID:  &'static str = env!("APP_ID");
pub const PROFILE:  &'static str = env!("APP_PROFILE");
pub const RESOURCES_FILE: &'static str = env!("RESOURCES_FILE");
pub const VERSION: &'static str = concat!(env!("CARGO_PKG_VERSION"), env!("APP_SUFFIX"));
