use crate::apache_access::{avx2::stage1::Stage1, Log};
use std::str::from_utf8_unchecked;

#[macro_export]
macro_rules! check {
    ($parser:ident, $index:expr, $b:expr) => {
        unsafe {
            let s = *$parser.structurals.get_unchecked($index) as usize;
            if *$parser.input.get_unchecked(s)!= $b {
                return None;
            }
        }
    };
}

pub struct Stage2<'a> {
    // input string
    input: &'a [u8],
    structurals: Vec<u32>,
}

impl<'a> Stage2<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            structurals: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn parse(&mut self) -> Option<Log> {
        let mut log = Log::new();
        self.structurals = Stage1::new(self.input).find();
        if self.structurals.len() != 16 {
            return None;
        }

        check!(self, 0, b' ');
        let end = self.structurals[0] as usize;
        log.ip = unsafe { from_utf8_unchecked(self.input.get_unchecked(0..end)) };

        check!(self, 1, b' ');
        let start = self.structurals[0] as usize + 1;
        let end = self.structurals[1] as usize;
        log.identity = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 2, b' ');
        let start = self.structurals[1] as usize + 1;
        let end = self.structurals[2] as usize;
        log.user = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 3, b'[');
        check!(self, 4, b']');
        let start = self.structurals[3] as usize + 1;
        let end = self.structurals[4] as usize;
        log.date = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 5, b' ');
        check!(self, 6, b'"');
        check!(self, 7, b'"');
        let start = self.structurals[6] as usize + 1;
        let end = self.structurals[7] as usize;
        log.message = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 8, b' ');
        check!(self, 9, b' ');
        let start = self.structurals[8] as usize + 1;
        let end = self.structurals[9] as usize;
        log.status = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 10, b' ');
        let start = self.structurals[9] as usize + 1;
        let end = self.structurals[10] as usize;
        log.code = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 11, b'"');
        check!(self, 12, b'"');
        let start = self.structurals[11] as usize + 1;
        let end = self.structurals[12] as usize;
        log.referer = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        check!(self, 13, b' ');
        check!(self, 14, b'"');
        check!(self, 15, b'"');
        let start = self.structurals[14] as usize + 1;
        let end = self.structurals[15] as usize;
        log.user_agent = unsafe { from_utf8_unchecked(self.input.get_unchecked(start..end)) };

        Some(log)
    }
}
