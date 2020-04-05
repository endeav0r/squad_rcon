mod chat;
mod player;
mod rcon;
mod squad;
mod squad_rcon;
mod team;

pub use crate::squad_rcon::{SquadRcon, SERVERDATA_CHAT};
pub use chat::Chat;
pub use player::Player;
pub use squad::Squad;
pub use team::Team;

#[derive(Debug)]
pub enum Error {
    AuthenticationFailure,
    Disconnected,
    EmptyPacketBody,
    FromUtf8Error(std::string::FromUtf8Error),
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ProtocolError,
    SquadParsingError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AuthenticationFailure => write!(f, "Authentication Failure"),
            Error::Disconnected => write!(f, "Disconnected"),
            Error::EmptyPacketBody => write!(f, "Empty packet body"),
            Error::FromUtf8Error(from_utf8_error) => write!(f, "{}", from_utf8_error),
            Error::IoError(io_error) => write!(f, "{}", io_error),
            Error::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Error::ProtocolError => write!(f, "Protocol Error"),
            Error::SquadParsingError => write!(f, "Squad Parsing Error"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Error {
        Error::IoError(io_error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(from_utf8_error: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8Error(from_utf8_error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(parse_int_error: std::num::ParseIntError) -> Error {
        Error::ParseIntError(parse_int_error)
    }
}
