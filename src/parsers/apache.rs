use crate::parsers::Parser;

#[derive(Debug, PartialEq)]
pub enum ApacheLog<'a> {
    Common(CommonLog<'a>),
    Combined(CombinedLog<'a>),
    Error(ErrorLog<'a>),
}

pub struct ApacheParser<'a> {
    structurals: &'a [u32],
}

impl<'a> ApacheParser<'a> {
    pub fn new(structurals: &'a [u32]) -> Self {
        Self { structurals }
    }
}

impl<'a> Parser<'a> for ApacheParser<'a> {
    type Log = ApacheLog<'a>;
    type Error = ();

    fn parse(self, src: &'a str) -> Result<Self::Log, Self::Error> {
        if let Ok(log) = ApacheErrorParser::new(&self.structurals).parse(src) {
            return Ok(ApacheLog::Error(log));
        }

        if let Ok(log) = ApacheCombinedParser::new(&self.structurals).parse(src) {
            return Ok(ApacheLog::Combined(log));
        }

        if let Ok(log) = ApacheCommonParser::new(&self.structurals).parse(src) {
            return Ok(ApacheLog::Common(log));
        }

        Err(())
    }
}

#[derive(Debug, PartialEq)]
pub struct CommonLog<'a> {
    ip: &'a str,
    identity: &'a str,
    user: &'a str,
    date: &'a str,
    message: &'a str,
    status: &'a str,
    bytes: &'a str,
}

pub struct ApacheCommonParser<'a> {
    structurals: &'a [u32],
    structurals_i: usize,
}

impl<'a> ApacheCommonParser<'a> {
    pub fn new(structurals: &'a [u32]) -> Self {
        Self {
            structurals,
            structurals_i: 0,
        }
    }
}

impl<'a> Parser<'a> for ApacheCommonParser<'a> {
    type Log = CommonLog<'a>;
    type Error = ();

    fn parse(mut self, src: &'a str) -> Result<Self::Log, Self::Error> {
        let src = unsafe { static_cast_slice!(src) };
        let mut log = CommonLog {
            ip: "",
            identity: "",
            user: "",
            date: "",
            message: "",
            status: "",
            bytes: "",
        };

        let start = 0;
        read!(self, src, b' ');
        let end = index!(self);
        log.ip = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.identity = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.user = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b'[');
        let start = index!(self) + 1;
        read_until!(self, src, b']');
        let end = index!(self);
        log.date = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        bump!(self);
        read!(self, src, b'"');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b'"');
        let end = index!(self);
        log.message = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.status = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        log.bytes = unsafe { static_cast_str!(&src[start..]) };

        Ok(log)
    }
}

#[derive(Debug, PartialEq)]
pub struct CombinedLog<'a> {
    ip: &'a str,
    identity: &'a str,
    user: &'a str,
    date: &'a str,
    message: &'a str,
    status: &'a str,
    bytes: &'a str,
    referer: &'a str,
    user_agent: &'a str,
}

pub struct ApacheCombinedParser<'a> {
    structurals: &'a [u32],
    structurals_i: usize,
}

impl<'a> ApacheCombinedParser<'a> {
    pub fn new(structurals: &'a [u32]) -> Self {
        Self {
            structurals,
            structurals_i: 0,
        }
    }
}

impl<'a> Parser<'a> for ApacheCombinedParser<'a> {
    type Log = CombinedLog<'a>;
    type Error = ();

    fn parse(mut self, src: &'a str) -> Result<Self::Log, Self::Error> {
        let src = unsafe { static_cast_slice!(src) };
        let mut log = CombinedLog {
            ip: "",
            identity: "",
            user: "",
            date: "",
            message: "",
            status: "",
            bytes: "",
            referer: "",
            user_agent: "",
        };

        let start = 0;
        read!(self, src, b' ');
        let end = index!(self);
        log.ip = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.identity = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.user = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b'[');
        let start = index!(self) + 1;
        read_until!(self, src, b']');
        let end = index!(self);
        log.date = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        bump!(self);
        read!(self, src, b'"');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b'"');
        let end = index!(self);
        log.message = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.status = unsafe { static_cast_str!(&src[start..end]) };

        let start = index!(self) + 1;
        bump!(self);
        read!(self, src, b' ');
        let end = index!(self);
        log.bytes = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b'"');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b'"');
        let end = index!(self);
        log.referer = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        bump!(self);
        read!(self, src, b'"');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b'"');
        let end = index!(self);
        log.user_agent = unsafe { static_cast_str!(&src[start..end]) };

        Ok(log)
    }
}

#[derive(Debug, PartialEq)]
pub struct ErrorLog<'a> {
    date: &'a str,
    package: &'a str,
    pid: &'a str,
    client: &'a str,
    message: &'a str,
}

pub struct ApacheErrorParser<'a> {
    structurals: &'a [u32],
    structurals_i: usize,
}

impl<'a> ApacheErrorParser<'a> {
    pub fn new(structurals: &'a [u32]) -> Self {
        Self {
            structurals,
            structurals_i: 0,
        }
    }
}

impl<'a> Parser<'a> for ApacheErrorParser<'a> {
    type Log = ErrorLog<'a>;
    type Error = ();

    fn parse(mut self, src: &'a str) -> Result<Self::Log, Self::Error> {
        let src = unsafe { static_cast_slice!(src) };
        let mut log = ErrorLog {
            date: "",
            package: "",
            pid: "",
            client: "",
            message: "",
        };

        read!(self, src, b'[');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b']');
        let end = index!(self);
        log.date = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        bump!(self);

        read!(self, src, b'[');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b']');
        let end = index!(self);
        log.package = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        bump!(self);

        read!(self, src, b'[');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b']');
        let end = index!(self);
        log.pid = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        bump!(self);

        read!(self, src, b'[');
        let start = index!(self) + 1;
        bump!(self);
        read_until!(self, src, b']');
        let end = index!(self);
        log.client = unsafe { static_cast_str!(&src[start..end]) };

        bump!(self);
        read!(self, src, b' ');
        let start = index!(self) + 1;

        log.message = unsafe { static_cast_str!(&src[start..]) };

        Ok(log)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage1::Stage1;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn common_line() {
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326"#;
        let target = CommonLog {
            ip: "127.0.0.1",
            identity: "-",
            user: "frank",
            date: "10/Oct/2000:13:55:36 -0700",
            message: "GET /apache_pb.gif HTTP/1.0",
            status: "200",
            bytes: "2326",
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(
            ApacheCommonParser::new(&structurals).parse(line),
            Ok(target)
        );
    }

    #[test]
    fn combined_line() {
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08 [en] (Win98; I ;Nav)""#;
        let target = CombinedLog {
            ip: "127.0.0.1",
            identity: "-",
            user: "frank",
            date: "10/Oct/2000:13:55:36 -0700",
            message: "GET /apache_pb.gif HTTP/1.0",
            status: "200",
            bytes: "2326",
            referer: "http://www.example.com/start.html",
            user_agent: "Mozilla/4.08 [en] (Win98; I ;Nav)",
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(
            ApacheCombinedParser::new(&structurals).parse(line),
            Ok(target)
        );
    }

    #[test]
    fn error_line() {
        let line = r#"[Fri Sep 09 10:42:29.902022 2011] [core:error] [pid 35708:tid 4328636416] [client 72.15.99.187] File does not exist: /usr/local/apache2/htdocs/favicon.ico"#;
        let target = ErrorLog {
            date: "Fri Sep 09 10:42:29.902022 2011",
            package: "core:error",
            pid: "pid 35708:tid 4328636416",
            client: "client 72.15.99.187",
            message: "File does not exist: /usr/local/apache2/htdocs/favicon.ico",
        };

        let structurals = Stage1::new(line).parse();
        //TODO assert_eq on structurals

        assert_eq!(ApacheErrorParser::new(&structurals).parse(line), Ok(target));
    }

    #[test]
    fn all_line() {
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326"#;
        let target = CommonLog {
            ip: "127.0.0.1",
            identity: "-",
            user: "frank",
            date: "10/Oct/2000:13:55:36 -0700",
            message: "GET /apache_pb.gif HTTP/1.0",
            status: "200",
            bytes: "2326",
        };
        assert_eq!(
            ApacheParser::new(&Stage1::new(line).parse()).parse(line),
            Ok(ApacheLog::Common(target))
        );

        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08 [en] (Win98; I ;Nav)""#;
        let target = CombinedLog {
            ip: "127.0.0.1",
            identity: "-",
            user: "frank",
            date: "10/Oct/2000:13:55:36 -0700",
            message: "GET /apache_pb.gif HTTP/1.0",
            status: "200",
            bytes: "2326",
            referer: "http://www.example.com/start.html",
            user_agent: "Mozilla/4.08 [en] (Win98; I ;Nav)",
        };
        assert_eq!(
            ApacheParser::new(&Stage1::new(line).parse()).parse(line),
            Ok(ApacheLog::Combined(target))
        );

        let line = r#"[Fri Sep 09 10:42:29.902022 2011] [core:error] [pid 35708:tid 4328636416] [client 72.15.99.187] File does not exist: /usr/local/apache2/htdocs/favicon.ico"#;
        let target = ErrorLog {
            date: "Fri Sep 09 10:42:29.902022 2011",
            package: "core:error",
            pid: "pid 35708:tid 4328636416",
            client: "client 72.15.99.187",
            message: "File does not exist: /usr/local/apache2/htdocs/favicon.ico",
        };
        assert_eq!(
            ApacheParser::new(&Stage1::new(line).parse()).parse(line),
            Ok(ApacheLog::Error(target))
        );
    }

    #[test]
    fn common_samples() {
        let mut buf = String::new();
        File::open("samples/apache_common.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let lines: Vec<&str> = buf.lines().collect();

        for line in lines {
            if ApacheCommonParser::new(&Stage1::new(line).parse())
                .parse(line)
                .is_err()
            {
                panic!("failed to parse: {}", line);
            }
        }
    }

    #[test]
    fn combined_samples() {
        let mut buf = String::new();
        File::open("samples/apache_combined.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let lines: Vec<&str> = buf.lines().collect();

        for line in lines {
            if ApacheCombinedParser::new(&Stage1::new(line).parse())
                .parse(line)
                .is_err()
            {
                panic!("failed to parse: {}", line);
            }
        }
    }

    #[test]
    fn error_samples() {
        let mut buf = String::new();
        File::open("samples/apache_error.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let lines: Vec<&str> = buf.lines().collect();

        for line in lines {
            if ApacheErrorParser::new(&Stage1::new(line).parse())
                .parse(line)
                .is_err()
            {
                panic!("failed to parse: {}", line);
            }
        }
    }

    #[test]
    fn all_samples() {
        let mut buf = String::new();
        File::open("samples/apache_common.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let lines: Vec<&str> = buf.lines().collect();

        for line in lines {
            let structurals = Stage1::new(line).parse();
            let log = ApacheParser::new(&structurals).parse(line);
            if log.is_err() {
                panic!("failed to parse: {}", line);
            }
            match log {
                Ok(ApacheLog::Common(_)) => {}
                _ =>  panic!("failed to parse: {:?}", log.unwrap()),
            }
        }

        let mut buf = String::new();
        File::open("samples/apache_combined.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let lines: Vec<&str> = buf.lines().collect();

        for line in lines {
            let structurals = Stage1::new(line).parse();
            let log = ApacheParser::new(&structurals).parse(line);
            if log.is_err() {
                panic!("failed to parse: {}", line);
            }
            match log {
                Ok(ApacheLog::Combined(_)) => {}
                _ =>  panic!("failed to parse: {:?}", log.unwrap()),
            }
        }

        let mut buf = String::new();
        File::open("samples/apache_error.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let lines: Vec<&str> = buf.lines().collect();

        for line in lines {
            let structurals = Stage1::new(line).parse();
            let log = ApacheParser::new(&structurals).parse(line);
            if log.is_err() {
                panic!("failed to parse: {}", line);
            }
            match log {
                Ok(ApacheLog::Error(_)) => {}
                _ =>  panic!("failed to parse: {:?}", log.unwrap()),
            }
        }
    }
}
