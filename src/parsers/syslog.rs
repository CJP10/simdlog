use crate::parsers::Parser;

#[derive(Debug, PartialEq)]
pub struct RFC5424Log<'a> {
    prefix: &'a str,
    timestamp: &'a str,
    hostname: &'a str,
    app_name: &'a str,
    proc_id: &'a str,
    msg_id: &'a str,
    structured_data: &'a str,
    message: &'a str,
}

pub struct SyslogRFC5424Parser<'a> {
    structurals: &'a [u32],
    structurals_i: usize,
}

impl<'a> SyslogRFC5424Parser<'a> {
    pub fn new(structurals: &'a [u32]) -> Self {
        Self {
            structurals,
            structurals_i: 0,
        }
    }

    #[inline(always)]
    pub fn get(&self, src: &[u8]) -> Option<u8> {
        self.structurals
            .get(self.structurals_i)
            .map(|i| src[*i as usize])
    }
}

impl<'a> Parser<'a> for SyslogRFC5424Parser<'a> {
    type Log = RFC5424Log<'a>;
    type Error = ();

    fn parse(mut self, src: &'a str) -> Result<Self::Log, Self::Error> {
        let src = unsafe { static_cast_slice!(src) };
        let mut log = RFC5424Log {
            prefix: "",
            timestamp: "",
            hostname: "",
            app_name: "",
            proc_id: "",
            msg_id: "",
            structured_data: "",
            message: "",
        };

        let start = 0;
        read!(self, src, b' ');
        let end = index!(self);
        log.prefix = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.timestamp = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.hostname = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.app_name = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.proc_id = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.msg_id = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        // handle lines with no message and only structured data
        match self.get(src) {
            // empty structured data
            Some(b' ') => {
                let end = index!(self);
                log.structured_data = unsafe { static_cast_str!(&src[start..end]) };
            }
            Some(b'[') => loop {
                read!(self, src, b'[');
                read_until!(self, src, b']');
                bump!(self);
                match self.get(&src) {
                    Some(b' ') => {
                        let end = index!(self);
                        log.structured_data = unsafe { static_cast_str!(&src[start..end]) };
                        break;
                    }
                    None => {
                        log.structured_data = unsafe { static_cast_str!(&src[start..]) };
                        return Ok(log);
                    }
                    _ => {}
                }
            },
            Some(_) => return Err(()),
            None => {
                log.structured_data = unsafe { static_cast_str!(&src[start..]) };
                return Ok(log);
            }
        }

        let start = index!(self) + 1;
        log.message = unsafe { static_cast_str!(&src[start..]) };

        Ok(log)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage1::Stage1;

    #[test]
    fn rfc5424_simple1() {
        let line = "<34>1 2003-10-11T22:14:15.003Z mymachine.example.com su - ID47 - BOM'su root' failed for lonvick on /dev/pts/8";
        let log = RFC5424Log {
            prefix: "<34>1",
            timestamp: "2003-10-11T22:14:15.003Z",
            hostname: "mymachine.example.com",
            app_name: "su",
            proc_id: "-",
            msg_id: "ID47",
            structured_data: "-",
            message: "BOM'su root' failed for lonvick on /dev/pts/8",
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(SyslogRFC5424Parser::new(&structurals).parse(line), Ok(log))
    }

    #[test]
    fn rfc5424_simple2() {
        let line = "<165>1 2003-08-24T05:14:15.000003-07:00 192.0.2.1 myproc 8710 - - %% It's time to make the do-nuts.";
        let log = RFC5424Log {
            prefix: "<165>1",
            timestamp: "2003-08-24T05:14:15.000003-07:00",
            hostname: "192.0.2.1",
            app_name: "myproc",
            proc_id: "8710",
            msg_id: "-",
            structured_data: "-",
            message: "%% It's time to make the do-nuts.",
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(SyslogRFC5424Parser::new(&structurals).parse(line), Ok(log))
    }

    #[test]
    fn rfc5424_strcutured1() {
        let line = r#"<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource= "Application" eventID="1011"] BOMAn application event log entry..."#;
        let log = RFC5424Log {
            prefix: "<165>1",
            timestamp: "2003-10-11T22:14:15.003Z",
            hostname: "mymachine.example.com",
            app_name: "evntslog",
            proc_id: "-",
            msg_id: "ID47",
            structured_data:
                r#"[exampleSDID@32473 iut="3" eventSource= "Application" eventID="1011"]"#,
            message: "BOMAn application event log entry...",
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(SyslogRFC5424Parser::new(&structurals).parse(line), Ok(log))
    }

    #[test]
    fn rfc5424_strcutured2() {
        let line = r#"<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource= "Application" eventID="1011"][examplePriority@32473 class="high"]"#;
        let log = RFC5424Log {
            prefix: "<165>1",
            timestamp: "2003-10-11T22:14:15.003Z",
            hostname: "mymachine.example.com",
            app_name: "evntslog",
            proc_id: "-",
            msg_id: "ID47",
            structured_data: r#"[exampleSDID@32473 iut="3" eventSource= "Application" eventID="1011"][examplePriority@32473 class="high"]"#,
            message: ""
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(SyslogRFC5424Parser::new(&structurals).parse(line), Ok(log))
    }
}
