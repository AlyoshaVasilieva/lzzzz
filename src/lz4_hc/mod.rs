//! LZ4_HC Block Compressor
//!
//! The `lz4_hc` module doesn't provide decompression functionalities.
//! Use the `lz4` module instead.
mod api;

use crate::{LZ4Error, Result};
use api::ExtState;

/// Compression mode specifier
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompressionMode {
    Default,
    Partial,
}

impl Default for CompressionMode {
    fn default() -> Self {
        Self::Default
    }
}

/// Compression level specifier
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompressionLevel {
    /// Custom compression level.
    /// Any value between 1 and 12 is valid.
    Custom(i32),
    /// `Min` is same as `Custom(3)`.
    Min,
    /// `Default` is same as `Custom(9)`.
    Default,
    /// `OptMin` is same as `Custom(10)`.
    OptMin,
    /// `Max` is same as `Custom(12)`.
    Max,
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self::Default
    }
}

impl CompressionLevel {
    fn as_i32(&self) -> i32 {
        match self {
            Self::Custom(level) => *level,
            Self::Min => 3,
            Self::Default => 9,
            Self::OptMin => 10,
            Self::Max => 12,
        }
    }
}

/// Read data from a slice and write compressed data into another slice.
///
/// Ensure that the destination slice have enough capacity.
/// If `dst.len()` is smaller than `lz4::max_compressed_size(src.len())`,
/// this function may fail.
///
/// # Examples
///
/// Compress data with the default compression mode:
///
/// ```
/// use lzzzz::{lz4, lz4_hc};
///
/// let data = "— Да, простите, — повторил он то же слово, которым закончил и весь рассказ.";
/// let mut buf = [0u8; 2048];
///
/// // The slice should have enough space.
/// assert!(buf.len() >= lz4::max_compressed_size(data.len()));
///
/// let len = lz4_hc::compress_to_slice(
///     data.as_bytes(),
///     &mut buf,
///     lz4_hc::CompressionMode::Default,
///     lz4_hc::CompressionLevel::Default,
/// )
/// .unwrap();
/// let compressed = &buf[..len];
///
/// # let mut buf = [0u8; 2048];
/// # let len = lz4::decompress(
/// #     compressed,
/// #     &mut buf[..data.len()],
/// #     lz4::DecompressionMode::Default,
/// # )
/// # .unwrap();
/// # assert_eq!(&buf[..len], data.as_bytes());
/// ```
pub fn compress_to_slice(
    src: &[u8],
    dst: &mut [u8],
    mode: CompressionMode,
    compression_level: CompressionLevel,
) -> Result<usize> {
    let len = EXT_STATE.with(|state| {
        api::compress_ext_state(
            &mut state.borrow_mut(),
            src,
            dst,
            compression_level.as_i32(),
        )
    });
    if len > 0 {
        Ok(len)
    } else {
        Err(LZ4Error::from("Compression failed"))
    }
}

/// Read data from a slice and append compressed data to `Vec<u8>`.
///
/// In this function, `CompressionMode::Partial` has no special meaning and
/// is same as `CompressionMode::Default`.
///
/// [`CompressionMode::Partial`]: /enum.CompressionMode.html#variant.Partial
/// [`CompressionMode::Default`]: /enum.CompressionMode.html#variant.Default
///
/// # Examples
///
/// ### Basic usage
///
/// Compress data into the `Vec<u8>` with the default compression mode/level.
///
/// ```
/// use lzzzz::lz4_hc;
///
/// let data = "So we beat on, boats against the current, borne back ceaselessly into the past.";
/// let mut buf = Vec::new();
///
/// lz4_hc::compress(
///     data.as_bytes(),
///     &mut buf,
///     lz4_hc::CompressionMode::Default,
///     lz4_hc::CompressionLevel::Default,
/// );
/// # let compressed = &buf;
/// # let mut buf = [0u8; 2048];
/// # let len = lzzzz::lz4::decompress(
/// #     compressed,
/// #     &mut buf[..data.len()],
/// #     lzzzz::lz4::DecompressionMode::Default,
/// # )
/// # .unwrap();
/// # assert_eq!(&buf[..len], data.as_bytes());
/// ```
///
/// ### Higher compression level
///
/// ```
/// use lzzzz::lz4_hc;
///
/// let data = "It was not till they had examined the rings that they recognized who it was.";
/// let mut buf = Vec::new();
///
/// lz4_hc::compress(
///     data.as_bytes(),
///     &mut buf,
///     lz4_hc::CompressionMode::Default,
///     lz4_hc::CompressionLevel::Max,
/// );
/// # let compressed = &buf;
/// # let mut buf = [0u8; 2048];
/// # let len = lzzzz::lz4::decompress(
/// #     compressed,
/// #     &mut buf[..data.len()],
/// #     lzzzz::lz4::DecompressionMode::Default,
/// # )
/// # .unwrap();
/// # assert_eq!(&buf[..len], data.as_bytes());
/// ```
pub fn compress(
    src: &[u8],
    dst: &mut Vec<u8>,
    mode: CompressionMode,
    compression_level: CompressionLevel,
) -> Result<()> {
    let orig_len = dst.len();
    dst.reserve(crate::lz4::max_compressed_size(src.len()));
    #[allow(unsafe_code)]
    unsafe {
        dst.set_len(dst.capacity());
    }
    let result = compress_to_slice(
        src,
        &mut dst[orig_len..],
        CompressionMode::Default,
        compression_level,
    );
    dst.resize_with(orig_len + *result.as_ref().unwrap_or(&0), Default::default);
    result.map(|_| ())
}

thread_local!(static EXT_STATE: ExtState = ExtState::new());
