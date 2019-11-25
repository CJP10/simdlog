pub mod avx2;

#[cfg(target_feature = "avx2")]
#[inline]
pub fn parse(line: &str) -> Option<Log> {
    let mut stage = avx2::Stage2::new(
        unsafe { static_cast_slice!(line) }
    );
    stage.parse()
}

#[cfg(not(target_feature = "avx2"))]
#[inline]
pub fn parse(line: &str) -> Option<Log> {
    //TODO impl a scalar fallback and an sse fallback
    unimplemented!()
}

#[derive(Debug, PartialEq)]
pub struct Log<'a> {
    ip: &'a str,
    identity: &'a str,
    user: &'a str,
    date: &'a str,
    message: &'a str,
    status: &'a str,
    code: &'a str,
    referer: &'a str,
    user_agent: &'a str,
}

impl<'a> Log<'a> {

    #[inline]
    pub const fn new() -> Log<'a> {
        Log {
            ip: "",
            identity: "",
            user: "",
            date: "",
            message: "",
            status: "",
            code: "",
            referer: "",
            user_agent: "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn common_line() {
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326"#;
        let target = Log {
            ip: "127.0.0.1",
            identity: "-",
            user: "frank",
            date: "10/Oct/2000:13:55:36 -0700",
            message: "GET /apache_pb.gif HTTP/1.0",
            status: "200",
            code: "2326",
            referer: "",
            user_agent: "",
        };

        assert_eq!(parse(line), Some(target));
    }

    #[test]
    fn combined_line() {
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08 [en] (Win98; I ;Nav)""#;
        let target = Log {
            ip: "127.0.0.1",
            identity: "-",
            user: "frank",
            date: "10/Oct/2000:13:55:36 -0700",
            message: "GET /apache_pb.gif HTTP/1.0",
            status: "200",
            code: "2326",
            referer: "http://www.example.com/start.html",
            user_agent: "Mozilla/4.08 [en] (Win98; I ;Nav)",
        };

        assert_eq!(parse(line), Some(target));
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
            if parse(line).is_none() {
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
            if parse(line).is_none() {
                panic!("failed to parse: {}", line);
            }
        }
    }

    #[test]
    fn empty_quotes() {
        let line = r#"83.149.9.216 - - [] "" 200 203023 "" """#;
        let target = Log {
            ip: "83.149.9.216",
            identity: "-",
            user: "-",
            date: "",
            message: "",
            status: "200",
            code: "203023",
            referer: "",
            user_agent: "",
        };

        assert_eq!(parse(line), Some(target));
    }
}
