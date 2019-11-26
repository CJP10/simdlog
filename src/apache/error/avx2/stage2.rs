use super::super::Log;
use super::Stage1;

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

        if !(self.structurals.len() == 8) {
            return None;
        }

        check!(self, 0, b'[');
        check!(self, 1, b']');
        let start = self.structurals[0] as usize + 1;
        let end = self.structurals[1] as usize;
        log.date = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 2, b'[');
        check!(self, 3, b']');
        let start = self.structurals[2] as usize + 1;
        let end = self.structurals[3] as usize;
        log.package = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 4, b'[');
        check!(self, 5, b']');
        let start = self.structurals[4] as usize + 1;
        let end = self.structurals[5] as usize;
        log.pid = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        check!(self, 6, b'[');
        check!(self, 7, b']');
        let start = self.structurals[6] as usize + 1;
        let end = self.structurals[7] as usize;
        log.client = unsafe { static_cast_str!(self.src.get_unchecked(start..end)) };

        let start = self.structurals[7] as usize + 2;
        log.message = unsafe { static_cast_str!(self.src.get_unchecked(start..)) };

        Some(log)
    }
}
