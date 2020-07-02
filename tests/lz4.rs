#![cfg(feature = "lz4")]

use lzzzz::lz4;
use rayon::{iter::ParallelBridge, prelude::*};
use std::cmp;

mod common;
use common::lz4_test_set;

mod compress {
    use super::*;

    #[test]
    fn default() {
        lz4_test_set().par_bridge().for_each(|(src, mode)| {
            let mut comp_buf = vec![0; lz4::max_compressed_size(src.len())];
            let mut decomp_buf = vec![0; src.len()];
            let len = lz4::compress(&src, &mut comp_buf, mode).unwrap().dst_len();
            lz4::decompress(
                &comp_buf[..len],
                &mut decomp_buf,
                lz4::DecompressionMode::Default,
            )
            .unwrap();
            assert_eq!(decomp_buf, src);
        });
    }
}

mod compress_to_vec {
    use super::*;

    #[test]
    fn default() {
        lz4_test_set().par_bridge().for_each(|(src, mode)| {
            let mut comp_buf = Vec::new();
            let mut decomp_buf = vec![0; src.len()];
            lz4::compress_to_vec(&src, &mut comp_buf, mode).unwrap();
            lz4::decompress(&comp_buf, &mut decomp_buf, lz4::DecompressionMode::Default).unwrap();
            assert_eq!(decomp_buf, src);
        });
    }
}

mod decompress {
    use super::*;

    #[test]
    fn partial() {
        lz4_test_set()
            .map(|(src, mode)| (0..20).map(move |n| (src.clone(), mode, 16 << n)))
            .flatten()
            .par_bridge()
            .for_each(|(src, mode, len)| {
                let mut comp_buf = Vec::new();
                let mut decomp_buf = vec![0; cmp::min(src.len(), len)];
                lz4::compress_to_vec(&src, &mut comp_buf, mode).unwrap();
                lz4::decompress(
                    &comp_buf,
                    &mut decomp_buf,
                    lz4::DecompressionMode::Partial {
                        original_size: src.len(),
                    },
                )
                .unwrap();
                assert!(src.starts_with(&decomp_buf));
            });
    }
}