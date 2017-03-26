use std::error;
use std::fmt;
use std::io;
use hyper;
use hyper_native_tls::native_tls;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    TLSError(native_tls::Error),
    ConnectionError(hyper::Error),
    JSONError(json::DecoderError),
    IOError(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TLSError(ref err) => err.fmt(f),
            Error::ConnectionError(ref err) => err.fmt(f),
            Error::JSONError(ref err) => err.fmt(f),
            Error::IOError(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TLSError(ref err) => err.description(),
            Error::ConnectionError(ref err) => err.description(),
            Error::JSONError(ref err) => err.description(),
            Error::IOError(ref err) => err.description(),
        }
    }
}

impl From<native_tls::Error> for Error {
    fn from(err: native_tls::Error) -> Error {
        Error::TLSError(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::ConnectionError(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::JSONError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}
