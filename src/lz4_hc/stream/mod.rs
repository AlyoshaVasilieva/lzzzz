//! LZ4_HC Streaming Compressor
//!
//! The `lz4_hc_stream` module doesn't provide decompression functionalities.
//! Use the [`lz4_stream`] module instead.
//!
//! [`lz4_stream`]: ../lz4_stream/index.html
//!
//! # Example
//! ```
//! use lzzzz::lz4_hc;
//!
//! let mut stream = lz4_hc::Compressor::new().unwrap();
//!
//! let data = &b"aaaaa"[..];
//! let mut buf = Vec::new();
//!
//! stream.next_to_vec(data, &mut buf, lz4_hc::CompressionMode::Default);
//!
//! # use lzzzz::lz4;
//! # let compressed = &buf;
//! # let mut buf = [0u8; 2048];
//! # let len = lz4::decompress(
//! #     compressed,
//! #     &mut buf[..data.len()],
//! #     lz4::DecompressionMode::Default,
//! # )
//! # .unwrap()
//! # .dst_len();
//! # assert_eq!(&buf[..len], &data[..]);
//! ```

mod api;

use crate::{
    lz4,
    lz4_hc::{CompressionLevel, CompressionMode},
    Buffer, Report, Result,
};
use api::CompressionContext;

/// Streaming compressor
pub struct Compressor<'a> {
    ctx: CompressionContext,
    compression_level: CompressionLevel,
    dict: Buffer<'a>,
    prev: Buffer<'a>,
}

impl<'a> Compressor<'a> {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ctx: CompressionContext::new()?,
            compression_level: CompressionLevel::Default,
            dict: Buffer::new(),
            prev: Buffer::new(),
        })
    }

    #[cfg(feature = "liblz4-experimental")]
    #[cfg_attr(docsrs, doc(cfg(feature = "liblz4-experimental")))]
    pub fn set_compression_level(&mut self, level: CompressionLevel) {
        self.compression_level = level;
        self.ctx.set_compression_level(level.as_i32());
    }

    #[cfg(feature = "liblz4-experimental")]
    #[cfg_attr(docsrs, doc(cfg(feature = "liblz4-experimental")))]
    pub fn set_favor_dec_speed(&mut self, flag: bool) {
        self.ctx.set_favor_dec_speed(flag);
    }

    pub fn reset(&mut self) {
        self.ctx.reset(self.compression_level.as_i32());
    }

    pub fn reset_with_dict<B>(&mut self, dict: B)
    where
        B: Into<Buffer<'a>>,
    {
        let dict = dict.into();
        if dict.is_empty() {
            self.reset();
        } else {
            self.ctx.load_dict(&dict);
        }
        self.dict = dict;
    }

    /// LZ4 Streaming Compressor/Decompressor
    ///
    /// # Example
    /// ```
    /// use lzzzz::{lz4, lz4_hc};
    ///
    /// let mut stream = lz4_hc::Compressor::new().unwrap();
    ///
    /// let data = &b"As soon as they had strength, they arose, joined hands again, and went on."[..];
    /// let mut buf = [0u8; 2048];
    ///
    /// // The slice should have enough space.
    /// assert!(buf.len() >= lz4::max_compressed_size(data.len()));
    ///
    /// let len = stream
    ///     .next(data, &mut buf, lz4_hc::CompressionMode::Default)
    ///     .unwrap()
    ///     .dst_len();
    /// let compressed = &buf[..len];
    ///
    /// # let mut buf = [0u8; 2048];
    /// # let len = lz4::decompress(
    /// #     compressed,
    /// #     &mut buf[..data.len()],
    /// #     lz4::DecompressionMode::Default,
    /// # )
    /// # .unwrap()
    /// # .dst_len();
    /// # assert_eq!(&buf[..len], &data[..]);
    /// ```
    pub fn next<B>(&mut self, src: B, dst: &mut [u8], mode: CompressionMode) -> Result<Report>
    where
        B: Into<Buffer<'a>>,
    {
        let src = src.into();
        let result = match mode {
            CompressionMode::Default => self.ctx.next(&src, dst),
            _ => self.ctx.next_partial(&src, dst),
        };
        self.prev = src;
        result
    }

    pub fn next_to_vec<B>(
        &mut self,
        src: B,
        dst: &mut Vec<u8>,
        mode: CompressionMode,
    ) -> Result<Report>
    where
        B: Into<Buffer<'a>>,
    {
        let src = src.into();
        let orig_len = dst.len();
        dst.reserve(lz4::max_compressed_size(src.len()));
        #[allow(unsafe_code)]
        unsafe {
            dst.set_len(dst.capacity());
        }
        let result = self.next(src, &mut dst[orig_len..], mode);
        dst.resize_with(
            orig_len + result.as_ref().map(|r| r.dst_len()).unwrap_or(0),
            Default::default,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::lz4_hc::{CompressionMode, Compressor};

    #[test]
    fn empty_dst() {
        assert!(Compressor::new()
            .unwrap()
            .next(&b"hello"[..], &mut [], CompressionMode::Default)
            .is_err());
    }
}
