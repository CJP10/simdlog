pub mod avx2;

#[cfg(target_feature = "avx2")]
#[inline]
pub fn parse(line: &str) -> Option<Log> {
    let mut stage = avx2::Stage2::new(unsafe { static_cast_slice!(line) });
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
    date: &'a str,
    package: &'a str,
    pid: &'a str,
    client: &'a str,
    message: &'a str,
}

impl<'a> Log<'a> {
    #[inline]
    pub const fn new() -> Log<'a> {
        Log {
            date: "",
            package: "",
            pid: "",
            client: "",
            message: "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn error_line() {
        let line = r#"[Fri Sep 09 10:42:29.902022 2011] [core:error] [pid 35708:tid 4328636416] [client 72.15.99.187] File does not exist: /usr/local/apache2/htdocs/favicon.ico"#;
        let target = Log {
            date: "Fri Sep 09 10:42:29.902022 2011",
            package: "core:error",
            pid: "pid 35708:tid 4328636416",
            client: "client 72.15.99.187",
            message: "File does not exist: /usr/local/apache2/htdocs/favicon.ico",
        };

        assert_eq!(parse(line), Some(target));
    }

    #[test]
    fn common_samples() {
        let mut buf = String::new();
        File::open("samples/apache_error.txt")
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
}
