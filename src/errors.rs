use failure::Fail;
use std::result::Result as StdResult;

pub type Result<T = ()> = StdResult<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "parse error")]
    ParseError,
    #[fail(display = "stdin error")]
    StdinError,
    #[fail(display = "Send Requset error")]
    SendRequsetError,
    #[fail(display = "Send Requset error")]
    ParseJsonError,
    #[fail(display = "Unsuccessful response error")]
    UnsuccessResponseError,
    #[fail(display = "invalid link error")]
    InvalidLinkError,
    #[fail(display = "Create file error")]
    CreateFileError,
    #[fail(display = "Open file error")]
    OpenFileError,
    #[fail(display = "write to file error")]
    WriteToFileError,
    #[fail(display = "read file error")]
    ReadToStringError,
}
