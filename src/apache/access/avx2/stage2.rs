use super::Stage1;
use crate::apache::access::Log;

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
        Self { src, structurals }
    }

    #[inline]
    pub fn parse<'b>(&'a mut self) -> Option<Log<'b>> {
        let mut log = Log::new();

        if !(self.structurals.len() == 10 || self.structurals.len() == 16) {
            return None;
        }

        unsafe {
            check!(self, 0, b' ');
            let end = self.structurals[0]as usize;
            log.ip = get!(self.src, 0, end);

            check!(self, 1, b' ');
            let start = self.structurals[0] as usize + 1;
            let end = self.structurals[1] as usize;
            log.identity = get!(self.src, start, end);

            check!(self, 2, b' ');
            let start = self.structurals[1] as usize + 1;
            let end = self.structurals[2] as usize;
            log.user = get!(self.src, start, end);

            check!(self, 3, b'[');
            check!(self, 4, b']');
            let start = self.structurals[3] as usize + 1;
            let end = self.structurals[4] as usize;
            log.date = get!(self.src, start, end);

            check!(self, 5, b' ');
            check!(self, 6, b'"');
            check!(self, 7, b'"');
            let start = self.structurals[6] as usize + 1;
            let end = self.structurals[7] as usize;
            log.message = get!(self.src, start, end);

            check!(self, 8, b' ');
            check!(self, 9, b' ');
            let start = self.structurals[8] as usize + 1;
            let end = self.structurals[9] as usize;
            log.status = get!(self.src, start, end);

            if self.structurals.len() == 10 {
                let start = self.structurals[9] as usize + 1;
                log.code = get!(self.src, start);
                return Some(log);
            }

            check!(self, 10, b' ');
            let start = self.structurals[9] as usize + 1;
            let end = self.structurals[10] as usize;
            log.code = get!(self.src, start, end);

            check!(self, 11, b'"');
            check!(self, 12, b'"');
            let start = self.structurals[11] as usize + 1;
            let end = self.structurals[12] as usize;
            log.referer = get!(self.src, start, end);

            check!(self, 13, b' ');
            check!(self, 14, b'"');
            check!(self, 15, b'"');
            let start = self.structurals[14] as usize + 1;
            let end = self.structurals[15] as usize;
            log.user_agent = get!(self.src, start, end);

            Some(log)
        }
    }
}
