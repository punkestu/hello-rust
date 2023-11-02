use core::fmt;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct Error {
    pub message: &'static str,
    pub kind: ErrorKind,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub const ERR_NAME_IS_USED: Error = Error {
    kind: ErrorKind::AlreadyExists,
    message: "name is used",
};
