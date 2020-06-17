# Lzzzz
Yet another liblz4 binding 😴

[![Build Status](https://dev.azure.com/picoHz/lzzzz/_apis/build/status/lzzzz-CI?branchName=master)](https://dev.azure.com/picoHz/lzzzz/_build/latest?definitionId=2&branchName=master)

- **Designed for Rust:** Lzzzz is a high-level wrapper of liblz4 provides comprehensible API complies with Rust's manner without losing performance and flexibility. You have no concern about memory management and concurrency safety.

- **Various Modes:** `LZ4`, `LZ4_HC`, `LZ4F`, `LZ4 Streaming`, `LZ4_HC Streaming` and `LZ4F Streaming` are supported.

- **Flexible Streaming:** All the compressor/decompressor streams support `Read`, `BufRead` and `Write` operations. 
With `tokio` feature, `AsyncRead`, `AsyncBufRead` and `AsyncWrite` are also supported.

- LZ4 Block Compression/Decompression
- LZ4 Streaming Compression/Decompression (Reader/Writer)
- LZ4_HC Block Compression
- LZ4_HC Streaming Compression (Reader/Writer)
- LZ4F Compression/Decompression
- LZ4F Streaming Compression/Decompression (Reader/Writer)


## Features

- **tokio-io :** Add `AsyncRead`, `AsyncBufRead` and `AsyncWrite` support for streams.
- **use-tls :** (Enabled by default) Use the thread local storage to reuse resources.
- **lz4-use-stack :** (Enabled by default) Use the stack allocation for the LZ4 compressor stream.
Disabling this future may resolve stack overflow errors.