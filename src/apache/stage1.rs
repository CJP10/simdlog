use std::arch::x86_64::*;

#[macro_export]
macro_rules! static_cast_i8 {
    ($v:expr) => {
        std::mem::transmute::<_, i8>($v)
    };
}

#[macro_export]
macro_rules! static_cast_i32 {
    ($v:expr) => {
        std::mem::transmute::<_, i32>($v)
    };
}

#[macro_export]
macro_rules! static_cast_u32 {
    ($v:expr) => {
        std::mem::transmute::<_, u32>($v)
    };
}

pub struct Structurals<'a> {
    input: &'a [u8],
    len: usize,
    inside_quotes: u32,
    inside_braces: u32,
    index: usize,
    structurals: Vec<u32>,
}

impl<'a> Structurals<'a> {
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
        let mut padding = [0u8; 32];

        loop {
            if self.index >= self.len {
                break;
            }

            let v = if self.len >= self.index + 32 {
                unsafe { _mm256_loadu_si256(self.input.as_ptr().add(self.index) as *const __m256i) }
            } else {
                unsafe {
                    padding
                        .get_unchecked_mut(..self.len - self.index)
                        .clone_from_slice(self.input.get_unchecked(self.index..));
                    _mm256_loadu_si256(padding.as_ptr() as *const __m256i)
                }
            };

            let mask = unsafe { self.structurals_mask(v) };
            self.flatten_bits(mask);

            self.index += 32;
        }

        self.structurals
    }

    #[inline(always)]
    unsafe fn structurals_mask(&mut self, v: __m256i) -> u32 {
        // " 0x22 value = 2
        // - 0x2d value = 1
        // SPACE 0x20 value = 1
        // [ 0x5b value = 4
        // ] 0x5d value = 4

        #[rustfmt::skip]
            let high_mask = _mm256_setr_epi8(
            //  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
            0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        );

        #[rustfmt::skip]
            let low_mask = _mm256_setr_epi8(
            //  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
            1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 5, 0, 0,
            1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 5, 0, 0,
        );

        let low = _mm256_shuffle_epi8(low_mask, v);
        let high = _mm256_shuffle_epi8(
            high_mask,
            _mm256_and_si256(_mm256_srli_epi32(v, 4), _mm256_set1_epi8(0x7f)),
        );

        let lookup_mask = _mm256_and_si256(low, high);
        let mut structurals = static_cast_u32!(_mm256_movemask_epi8(_mm256_cmpgt_epi8(
            lookup_mask,
            _mm256_set1_epi8(0)
        )));

        let quote_bits = _mm256_movemask_epi8(_mm256_cmpeq_epi8(
            v,
            _mm256_set1_epi8(static_cast_i8!(b'"')),
        ));

        #[allow(overflowing_literals)]
        let mut quote_mask = _mm_cvtsi128_si32(_mm_clmulepi64_si128(
            _mm_set_epi32(0, 0, 0, static_cast_i32!(quote_bits)),
            _mm_set1_epi8(0xFF),
            0,
        )) as u32;

        quote_mask ^= self.inside_quotes;
        self.inside_quotes = static_cast_u32!(static_cast_i32!(quote_mask) >> 31);

        structurals &= !quote_mask;
        structurals |= static_cast_u32!(quote_bits);

        let brace_bits = _mm256_movemask_epi8(_mm256_cmpgt_epi8(
            _mm256_and_si256(lookup_mask, _mm256_set1_epi8(4)),
            _mm256_set1_epi8(0),
        ));

        #[allow(overflowing_literals)]
        let mut brace_mask = _mm_cvtsi128_si32(_mm_clmulepi64_si128(
            _mm_set_epi32(0, 0, 0, static_cast_i32!(brace_bits)),
            _mm_set1_epi8(0xFF),
            0,
        )) as u32;

        brace_mask ^= self.inside_braces;
        self.inside_braces = static_cast_u32!(static_cast_i32!(brace_mask) >> 31);

        structurals &= !brace_mask;
        structurals |= static_cast_u32!(brace_bits);

        structurals
    }

    #[inline(always)]
    fn flatten_bits(&mut self, mut bits: u32) {
        let cnt: usize = bits.count_ones() as usize;
        let mut l = self.structurals.len();
        let idx_32_v = unsafe { _mm256_set1_epi32(static_cast_i32!(self.index as u32)) };

        self.structurals.reserve(32);
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
                let v: __m256i = _mm256_add_epi32(idx_32_v, v);
                _mm256_storeu_si256(self.structurals.as_mut_ptr().add(l) as *mut __m256i, v);
            }
            l += 8;
        }
    }
}
