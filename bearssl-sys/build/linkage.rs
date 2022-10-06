#[cfg(not(feature = "bundled"))]
mod dynamichost;

#[cfg(feature = "bundled")]
mod bundled;

#[cfg(feature = "bundled")]
pub use bundled::{configure, HEADER_PATH};

#[cfg(not(feature = "bundled"))]
pub use dynamichost::{configure, HEADER_PATH};
