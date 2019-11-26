use crate::avx2::cmpeq_mask;
use std::arch::x86_64::*;

pub struct Stage1<'a> {
    input: &'a [u8],
    len: usize,
    inside_quotes: u32,
    inside_braces: u32,
    index: usize,
    structurals: Vec<u32>,
}

impl<'a> Stage1<'a> {
    #[inline]
    pub const fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            len: input.len(),
            inside_quotes: 0,
            inside_braces: 0,
            index: 0,
            structurals: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn find(mut self) -> Vec<u32> {
        let mut padding = [0u8; 64];

        loop {
            if self.index >= self.len {
                break;
            }

            let (v1, v2) = if self.len >= self.index + 64 {
                unsafe {
                    (
                        _mm256_loadu_si256(self.input.as_ptr().add(self.index) as *const __m256i),
                        _mm256_loadu_si256(
                            self.input.as_ptr().add(self.index + 32) as *const __m256i
                        ),
                    )
                }
            } else {
                unsafe {
                    padding
                        .get_unchecked_mut(..self.len - self.index)
                        .clone_from_slice(self.input.get_unchecked(self.index..));
                    (
                        _mm256_loadu_si256(padding.as_ptr() as *const __m256i),
                        _mm256_loadu_si256(padding.as_ptr().add(32) as *const __m256i),
                    )
                }
            };

            let mask_1 = unsafe { self.structurals_mask(v1) as u64 };
            let mask_2 = unsafe { self.structurals_mask(v2) as u64 };

            self.flatten_bits(mask_1 | (mask_2 << 32));

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

    #[inline(always)]
    fn flatten_bits(&mut self, mut bits: u64) {
        let cnt: usize = bits.count_ones() as usize;
        let mut l = self.structurals.len();
        let idx_64_v = unsafe {
            _mm256_set_epi32(
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
                static_cast_i32!(self.index as u32),
            )
        };

        self.structurals.reserve(64);
        unsafe {
            self.structurals.set_len(l + cnt);
        }

        while bits != 0 {
            unsafe {
                let v0 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v1 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v2 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v3 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v4 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v5 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v6 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);
                let v7 = bits.trailing_zeros() as i32;
                bits &= bits.wrapping_sub(1);

                let v: __m256i = _mm256_set_epi32(v7, v6, v5, v4, v3, v2, v1, v0);
                let v: __m256i = _mm256_add_epi64(idx_64_v, v);
                _mm256_storeu_si256(self.structurals.as_mut_ptr().add(l) as *mut __m256i, v);
            }
            l += 8;
        }
    }
}
