#![cfg(feature = "lz4f")]
//! LZ4 frame format

mod api;
mod binding;
mod dictionary;
mod frame;
mod frame_info;
mod preferences;
mod stream;

pub use dictionary::*;
pub use frame::*;
pub use frame_info::*;
pub use preferences::*;
pub use stream::*;

pub use stream::{CompressorBuilder, DecompressorBuilder};
