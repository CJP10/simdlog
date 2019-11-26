use crate::avx2::{cmpeq_mask, flatten_bits, load_2x256};
use std::arch::x86_64::*;

pub struct Stage1<'a> {
    src: &'a [u8],
    len: usize,
    inside_quotes: u32,
    inside_braces: u32,
    index: usize,
    structurals: Vec<u32>,
}

impl<'a> Stage1<'a> {
    #[inline]
    pub const fn new(src: &'a [u8]) -> Self {
        Self {
            src,
            len: src.len(),
            inside_quotes: 0,
            inside_braces: 0,
            index: 0,
            structurals: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn find(mut self) -> Vec<u32> {
        loop {
            if self.index >= self.len {
                break;
            }

            let (v1, v2) = unsafe { load_2x256(self.src, self.index) };

            let mask_1 = unsafe { self.structurals_mask(v1) as u64 };
            let mask_2 = unsafe { self.structurals_mask(v2) as u64 };

            flatten_bits(self.index, &mut self.structurals, mask_1 | (mask_2 << 32));

            self.index += 64;
        }

        self.structurals
    }

    #[inline(always)]
    unsafe fn structurals_mask(&mut self, v: __m256i) -> u32 {
        let quote_bits = cmpeq_mask(v, b'"');
        let mut brace_bits = cmpeq_mask(v, b'[') | cmpeq_mask(v, b']');
        let space_bits = cmpeq_mask(v, b' ');

        let mut structurals = quote_bits | brace_bits | space_bits;

        #[allow(overflowing_literals)]
        let mut quote_mask = _mm_cvtsi128_si32(_mm_clmulepi64_si128(
            _mm_set_epi32(0, 0, 0, static_cast_i32!(quote_bits)),
            _mm_set1_epi8(0xFF),
            0,
        )) as u32;

        quote_mask ^= self.inside_quotes;
        self.inside_quotes = static_cast_u32!(static_cast_i32!(quote_mask) >> 31);

        structurals &= !quote_mask;
        structurals |= quote_bits;

        brace_bits &= !quote_mask;

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
