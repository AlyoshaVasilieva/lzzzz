#![cfg(feature = "lz4f")]
//! LZ4 Frame Compressor/Decompressor

mod frame;
mod frame_info;
mod preferences;
mod stream;

pub use frame::*;
pub use frame_info::*;
pub use preferences::*;
pub use stream::*;

pub use stream::{CompressorBuilder, DecompressorBuilder};
