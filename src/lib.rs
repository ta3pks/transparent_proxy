mod flags;
#[cfg(feature = "socks5")]
pub mod socks;
pub use flags::{Flags, TargetType};
#[cfg(feature = "http")]
pub mod http;
