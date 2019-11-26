use std::arch::x86_64::*;

#[inline(always)]
pub unsafe fn cmpeq_mask(v: __m256i, b: u8) -> u32 {
    static_cast_u32!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(
        v,
        _mm256_set1_epi8(static_cast_i8!(b))
    )))
}
