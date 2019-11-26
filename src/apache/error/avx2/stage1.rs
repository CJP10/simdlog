use crate::avx2::{cmpeq_mask, flatten_bits, load_2x256};
use std::arch::x86_64::*;

pub struct Stage1<'a> {
    src: &'a [u8],
    len: usize,
    inside_braces: u32,
    src_i: usize,
    structurals: Vec<u32>,
}

impl<'a> Stage1<'a> {
    #[inline]
    pub const fn new(src: &'a [u8]) -> Self {
        Self {
            src,
            len: src.len(),
            inside_braces: 0,
            src_i: 0,
            structurals: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn find(mut self) -> Vec<u32> {
        loop {
            if self.src_i >= self.len {
                break;
            }

            let (v1, v2) = unsafe { load_2x256(self.src, self.src_i) };
            let mask_1 = unsafe { self.structurals_mask(v1) as u64 };
            let mask_2 = unsafe { self.structurals_mask(v2) as u64 };

            flatten_bits(self.src_i, &mut self.structurals, mask_1 | (mask_2 << 32));

            self.src_i += 64;
        }

        self.structurals
    }

    #[inline(always)]
    unsafe fn structurals_mask(&mut self, v: __m256i) -> u32 {
        let mut brace_bits = cmpeq_mask(v, b'[') | cmpeq_mask(v, b']');
        let mut structurals = brace_bits;

        #[allow(overflowing_literals)]
        let mut brace_mask = _mm_cvtsi128_si32(_mm_clmulepi64_si128(
            _mm_set_epi32(0, 0, 0, static_cast_i32!(brace_bits)),
            _mm_set1_epi8(0xFF),
            0,
        )) as u32;

        brace_mask ^= self.inside_braces;
        self.inside_braces = static_cast_u32!(static_cast_i32!(brace_mask) >> 31);

        structurals &= !brace_mask;
        structurals |= brace_bits;

        structurals
    }
}
