use crate::avx2::{cmpeq_mask, flatten_bits, load_2x256};

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

        loop {
            if self.src_i >= self.src.len() {
                break;
            }

            unsafe {
                let (v1, v2) = load_2x256(self.src, self.src_i, &mut padding);

                let mask_1 = cmpeq_mask(v1, b' ')
                    | cmpeq_mask(v1, b'"')
                    | cmpeq_mask(v1, b'[')
                    | cmpeq_mask(v1, b']');
                let mask_1 = mask_1 as u64;

                let mask_2 = cmpeq_mask(v2, b' ')
                    | cmpeq_mask(v2, b'"')
                    | cmpeq_mask(v2, b'[')
                    | cmpeq_mask(v2, b']');
                let mask_2 = mask_2 as u64;

                flatten_bits(self.src_i, &mut self.structurals, mask_1 | (mask_2 << 32))
            }

            self.src_i += 64;
        }

        self.structurals
    }
}
