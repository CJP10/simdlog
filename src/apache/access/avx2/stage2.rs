use super::Stage1;
use crate::apache::access::Log;

#[macro_export]
macro_rules! check {
    ($parser:ident, $index:expr, $b:expr) => {
        unsafe {
            let s = *$parser.structurals.get_unchecked($index) as usize;
            if *$parser.src.get_unchecked(s) != $b {
                return None;
            }
        }
    };
}

pub struct Stage2<'a> {
    src: &'a [u8],
    structurals: Vec<u32>,
}

impl<'a> Stage2<'a> {

    #[inline]
    pub fn new(src: &'a [u8]) -> Self {
        Self {
            src,
            structurals: Stage1::new(src).find(),
        }
    }

    #[inline]
    pub fn new_with_structurals(src: &'a [u8], structurals: Vec<u32>) -> Self {
        Self {
            src,
            structurals,
        }
    }

    #[inline]
    pub fn parse<'b>(&'a mut self) -> Option<Log<'b>> {
        let mut log = Log::new();

        if !(self.structurals.len() == 10 || self.structurals.len() == 16) {
            return None;
        }

        check!(self, 0, b' ');
        let end = self.structurals[0] as usize;
        log.ip = unsafe { static_cast_str!(self.src.get_unchecked(0..end)) };

        check!(self, 1, b' ');
        let start = self.structurals[0] as usize + 1;
        let end = self.structurals[1] as usize;
        log.identity = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 2, b' ');
        let start = self.structurals[1] as usize + 1;
        let end = self.structurals[2] as usize;
        log.user = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 3, b'[');
        check!(self, 4, b']');
        let start = self.structurals[3] as usize + 1;
        let end = self.structurals[4] as usize;
        log.date = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 5, b' ');
        check!(self, 6, b'"');
        check!(self, 7, b'"');
        let start = self.structurals[6] as usize + 1;
        let end = self.structurals[7] as usize;
        log.message = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 8, b' ');
        check!(self, 9, b' ');
        let start = self.structurals[8] as usize + 1;
        let end = self.structurals[9] as usize;
        log.status = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        if self.structurals.len() == 10 {
            let start = self.structurals[9] as usize + 1;
            log.code = unsafe { static_cast_str!(self.src.get_unchecked(start..)) };
            return Some(log);
        }

        check!(self, 10, b' ');
        let start = self.structurals[9] as usize + 1;
        let end = self.structurals[10] as usize;
        log.code = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 11, b'"');
        check!(self, 12, b'"');
        let start = self.structurals[11] as usize + 1;
        let end = self.structurals[12] as usize;
        log.referer = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 13, b' ');
        check!(self, 14, b'"');
        check!(self, 15, b'"');
        let start = self.structurals[14] as usize + 1;
        let end = self.structurals[15] as usize;
        log.user_agent = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        Some(log)
    }
}
