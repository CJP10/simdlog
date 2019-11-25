use crate::apache_access::{avx2::stage1::Structurals, Log};
use std::str::from_utf8_unchecked;

#[macro_export]
macro_rules! check {
    ($parser:ident ,$b:expr) => {
        if $parser.s_index >= $parser.structurals.len() {
            return None;
        }

        if $parser.input[$parser.structurals[$parser.s_index] as usize] != $b {
            return None;
        }

        $parser.s_index += 1;
    };
}

pub struct Stage2<'a> {
    // input string
    input: &'a [u8],
    // index into structurals
    s_index: usize,
    structurals: Vec<u32>,
}

impl<'a> Stage2<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            s_index: 0,
            structurals: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn parse(&mut self) -> Option<Log> {
        let mut log = Log::new();
        self.structurals = Structurals::new(self.input).find();

        check!(self, b' ');
        let end = self.structurals[self.s_index - 1] as usize;
        log.ip = unsafe { from_utf8_unchecked(&self.input[0..end]) };

        check!(self, b'-');
        check!(self, b' ');
        check!(self, b'-');
        check!(self, b' ');
        check!(self, b'[');
        let start = self.structurals[self.s_index - 1] as usize + 1;
        check!(self, b']');
        let end = self.structurals[self.s_index - 1] as usize;
        log.date = unsafe { from_utf8_unchecked(&self.input[start..end]) };

        check!(self, b' ');
        check!(self, b'"');
        let start = self.structurals[self.s_index - 1] as usize + 1;
        check!(self, b'"');
        let end = self.structurals[self.s_index - 1] as usize;
        log.message = unsafe { from_utf8_unchecked(&self.input[start..end]) };

        check!(self, b' ');
        let start = self.structurals[self.s_index - 1] as usize + 1;
        check!(self, b' ');
        let end = self.structurals[self.s_index - 1] as usize;
        log.status = unsafe { from_utf8_unchecked(&self.input[start..end]) };

        let start = self.structurals[self.s_index - 1] as usize + 1;
        check!(self, b' ');
        let end = self.structurals[self.s_index - 1] as usize;
        log.code = unsafe { from_utf8_unchecked(&self.input[start..end]) };

        check!(self, b'"');
        let start = self.structurals[self.s_index - 1] as usize + 1;
        check!(self, b'"');
        let end = self.structurals[self.s_index - 1] as usize;
        log.referer = unsafe { from_utf8_unchecked(&self.input[start..end]) };

        check!(self, b' ');
        check!(self, b'"');
        let start = self.structurals[self.s_index - 1] as usize + 1;
        check!(self, b'"');
        let end = self.structurals[self.s_index - 1] as usize;
        log.user_agent = unsafe { from_utf8_unchecked(&self.input[start..end]) };

        Some(log)
    }
}
