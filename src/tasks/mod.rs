mod configure_locale;
mod configure_network;
mod create_partitions;
mod install_base;
mod install_bootloader;
mod install_desktop;
mod setup_users;

pub use configure_locale::*;
pub use configure_network::*;
pub use create_partitions::*;
pub use install_base::*;
pub use install_bootloader::*;
pub use install_desktop::*;
pub use setup_users::*;
