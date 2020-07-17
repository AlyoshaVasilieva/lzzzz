#![cfg(feature = "tokio-io")]

use super::AsyncBufReadDecompressor;
use crate::lz4f::{FrameInfo, Result};
use pin_project::pin_project;
use std::{
    borrow::Cow,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, BufReader};

/// The [`AsyncRead`]-based streaming decompressor.
///
/// # Examples
///
/// ```
/// # use std::env;
/// # use std::path::Path;
/// # use lzzzz::{Error, Result};
/// # use assert_fs::prelude::*;
/// # let tmp_dir = assert_fs::TempDir::new().unwrap().into_persistent();
/// # env::set_current_dir(tmp_dir.path()).unwrap();
/// #
/// # let mut buf = Vec::new();
/// # lzzzz::lz4f::compress_to_vec(b"Hello world!", &mut buf, &Default::default()).unwrap();
/// # tmp_dir.child("foo.lz4").write_binary(&buf).unwrap();
/// #
/// # let mut rt = tokio::runtime::Runtime::new().unwrap();
/// # rt.block_on(async {
/// use lzzzz::lz4f::AsyncReadDecompressor;
/// use tokio::{fs::File, prelude::*};
///
/// let mut f = File::open("foo.lz4").await?;
/// let mut r = AsyncReadDecompressor::new(&mut f)?;
///
/// let mut buf = Vec::new();
/// r.read_to_end(&mut buf).await?;
/// # Ok::<(), tokio::io::Error>(())
/// # }).unwrap();
/// # tmp_dir.close().unwrap();
/// ```
///
/// [`AsyncRead`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-io")))]
#[pin_project]
pub struct AsyncReadDecompressor<'a, R: AsyncRead + Unpin> {
    #[pin]
    inner: AsyncBufReadDecompressor<'a, BufReader<R>>,
}

impl<'a, R: AsyncRead + Unpin> AsyncReadDecompressor<'a, R> {
    pub fn new(reader: R) -> Result<Self> {
        Ok(Self {
            inner: AsyncBufReadDecompressor::new(BufReader::new(reader))?,
        })
    }

    pub fn set_dict<D>(&mut self, dict: D)
    where
        D: Into<Cow<'a, [u8]>>,
    {
        self.inner.set_dict(dict);
    }

    pub async fn read_frame_info(&mut self) -> tokio::io::Result<FrameInfo> {
        self.inner.read_frame_info().await
    }
}

impl<'a, R: AsyncRead + Unpin> AsyncRead for AsyncReadDecompressor<'a, R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<tokio::io::Result<usize>> {
        self.project().inner.poll_read(cx, buf)
    }
}
