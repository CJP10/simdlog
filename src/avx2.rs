use std::arch::x86_64::*;

#[inline(always)]
pub unsafe fn cmpeq_mask(v: __m256i, b: u8) -> u32 {
    static_cast_u32!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(
        v,
        _mm256_set1_epi8(static_cast_i8!(b))
    )))
}

#[inline(always)]
pub unsafe fn flatten_bits(index: usize, structurals: &mut Vec<u32>, mut bits: u64) {
    let cnt: usize = bits.count_ones() as usize;
    let mut l = structurals.len();
    let idx_64_v = _mm256_set_epi32(
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
        static_cast_i32!(index as u32),
    );

    structurals.reserve(64);
    structurals.set_len(l + cnt);

    while bits != 0 {
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
        _mm256_storeu_si256(structurals.as_mut_ptr().add(l) as *mut __m256i, v);

        l += 8;
    }
}

#[inline(always)]
pub unsafe fn load_2x256(src: &[u8], index: usize, padding: &mut [u8]) -> (__m256i, __m256i) {
    let len = src.len();

    if len >= index + 64 {
        (
            _mm256_loadu_si256(src.as_ptr().add(index) as *const __m256i),
            _mm256_loadu_si256(src.as_ptr().add(index + 32) as *const __m256i),
        )
    } else {
        padding
            .get_unchecked_mut(..len - index)
            .clone_from_slice(src.get_unchecked(index..));
        (
            _mm256_loadu_si256(padding.as_ptr() as *const __m256i),
            _mm256_loadu_si256(padding.as_ptr().add(32) as *const __m256i),
        )
    }
}

#[inline(always)]
#[allow(overflowing_literals)]
pub unsafe fn fill_mask(mask: u32) -> u32 {
    static_cast_u32!(_mm_cvtsi128_si32(_mm_clmulepi64_si128(
        _mm_set_epi32(0, 0, 0, static_cast_i32!(mask)),
        _mm_set1_epi8(0xFF),
        0,
    )))
}

#[inline(always)]
pub unsafe fn lookup(v: __m256i, table_lo: __m256i, table_hi: __m256i) -> u32 {
    let low = _mm256_shuffle_epi8(table_lo, v);
    let high = _mm256_shuffle_epi8(
        table_hi,
        _mm256_and_si256(_mm256_srli_epi32(v, 4), _mm256_set1_epi8(0x7f)),
    );
    let mask = _mm256_and_si256(low, high);
    static_cast_u32!(_mm256_movemask_epi8(_mm256_cmpgt_epi8(
        mask,
        _mm256_set1_epi8(0)
    )))
}
