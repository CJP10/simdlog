use crate::avx2::{cmpeq_mask, flatten_bits, load_2x256, lookup};
use std::arch::x86_64::*;

pub struct Stage1<'a> {
    src: &'a [u8],
    src_i: usize,
    structurals: Vec<u32>,
}

impl<'a> Stage1<'a> {
    pub fn new(src: &str) -> Self {
        Self {
            src: unsafe { static_cast_slice!(src) },
            src_i: 0,
            structurals: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<u32> {
        let mut padding = [0u8; 64];

        // Table Values
        // " 0x22 = 1
        // SPACE 0x20 = 1
        // [ 0x5b = 2
        // ] 0x5d = 2
        // all other values = 0

        #[rustfmt::skip]
        let table_hi = unsafe {
            _mm256_setr_epi8(
            //  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
                0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            )
        };

        #[rustfmt::skip]
        let table_lo = unsafe {
            _mm256_setr_epi8(
            //  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
                1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0,
                1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0,
            )
        };

        loop {
            if self.src_i >= self.src.len() {
                break;
            }

            unsafe {
                let (v1, v2) = load_2x256(self.src, self.src_i, &mut padding);

                let mask_1 = lookup(v1, table_lo, table_hi) as u64;
                let mask_2 = lookup(v2, table_lo, table_hi) as u64;

                flatten_bits(self.src_i, &mut self.structurals, mask_1 | (mask_2 << 32))
            }

            self.src_i += 64;
        }

        self.structurals
    }
}
