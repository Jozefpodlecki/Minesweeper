#[cfg(debug_assertions)]
mod error;
mod game;

#[cfg(debug_assertions)]
pub use error::*;
pub use game::*;