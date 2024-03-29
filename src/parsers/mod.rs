pub mod apache;
pub mod syslog;

pub trait Parser<'a> {
    type Log;
    type Error;
    fn parse(self, src: &'a str) -> Result<Self::Log, Self::Error>;
}
