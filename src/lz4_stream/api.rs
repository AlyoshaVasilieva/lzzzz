#![allow(unsafe_code)]

use crate::{binding, binding::LZ4Stream, Error, Result};

use libc::{c_void, size_t};
use std::{
    mem::{size_of, MaybeUninit},
    ptr::NonNull,
};

enum Stream {
    #[cfg(feature = "lz4-use-stack")]
    Stack(LZ4Stream),
    Heap(NonNull<LZ4Stream>),
}

pub struct CompressionContext {
    stream: Stream,
}

impl CompressionContext {
    pub fn new() -> Result<Self> {
        let mut stream = MaybeUninit::<LZ4Stream>::zeroed();

        #[cfg(feature = "lz4-use-stack")]
        unsafe {
            let ptr = binding::LZ4_initStream(
                stream.as_mut_ptr() as *mut c_void,
                size_of::<LZ4Stream>() as size_t,
            );
            if !ptr.is_null() {
                return Ok(Self {
                    stream: Stream::Stack(stream.assume_init()),
                });
            }
        }

        unsafe {
            let ptr = NonNull::new(binding::LZ4_createStream());
            ptr.ok_or(Error::Generic).map(|stream| Self {
                stream: Stream::Heap(stream),
            })
        }
    }
}

impl Drop for CompressionContext {
    fn drop(&mut self) {
        if let Stream::Heap(mut ptr) = self.stream {
            unsafe {
                binding::LZ4_freeStream(ptr.as_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compression_context() {
        println!("{}", super::CompressionContext::new().is_ok());
    }
}
