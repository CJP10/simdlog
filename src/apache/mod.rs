mod stage1;
mod stage2;

pub use stage1::*;
pub use stage2::*;

#[derive(Debug, PartialEq)]
pub struct Log<'a> {
    ip: &'a str,
    date: &'a str,
    message: &'a str,
    status: &'a str,
    code: &'a str,
    referer: &'a str,
    user_agent: &'a str,
}

impl<'a> Log<'a> {
    pub const fn new() -> Log<'a> {
        Log {
            ip: "",
            date: "",
            message: "",
            status: "",
            code: "",
            referer: "",
            user_agent: ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_line() {
        let line = r#"83.149.9.216 - - [17/May/2015:10:05:03 +0000] "GET /presentations/logstash-monitorama-2013/images/kibana-search.png HTTP/1.1" 200 203023 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36""#;
        let target = Log {
            ip: "83.149.9.216",
            date: "17/May/2015:10:05:03 +0000",
            message: "GET /presentations/logstash-monitorama-2013/images/kibana-search.png HTTP/1.1",
            status: "200",
            code: "203023",
            referer: "http://semicomplete.com/presentations/logstash-monitorama-2013/",
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36"
        };

        assert_eq!(Stage2::new(line.as_bytes()).parse(), Some(target));
    }

}