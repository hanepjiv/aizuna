// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2018/04/12

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(variant_size_differences)]
#[derive(Debug)]
pub enum Error {
    /// OptionNone
    OptionNone,
    /// EnvVar
    EnvVar(::std::env::VarError),
    /// IO
    IO(::std::io::Error),
    /// Send
    Send(String),
    /// ParseInt
    ParseInt(::std::num::ParseIntError),
    /// SyncPoison
    SyncPoison(String),
    /// Utf8
    Utf8(::std::str::Utf8Error),
    /// UuidParse
    UuidParse(::uuid::ParseError),
    /// Regex
    Regex(::regex::Error),
    /// GetOpts
    GetOpts(::getopts::Fail),
    /// SerdeJSON
    SerdeJSON(::serde_json::Error),
    /// TOMLSer
    TOMLSer(::toml::ser::Error),
    /// TOMLDe
    TOMLDe(::toml::de::Error),
    /// LevelDB
    LevelDB(::rusty_leveldb::Status),
    /// Discord
    Discord(::discord::Error),
    /// Elicit
    Elicit(::elicit::Error),
    /// SerDeVer
    SerDeVer(i32, i32, i32),
    /// MissingField
    MissingField(String),
    /// Downcast
    Downcast(String),
    /// InvalidType
    InvalidType(String),
    /// InvalidArg
    InvalidArg(String),
    /// NoConfig
    NoConfig,
    /// Connector
    Connector(String),
    /// Aizuna
    Aizuna(String),
    /// AizunaDBVer
    AizunaDBVer(i32, i32, i32),
}
// ============================================================================
impl From<::std::option::NoneError> for Error {
    fn from(_: ::std::option::NoneError) -> Self {
        Error::OptionNone
    }
}
// ----------------------------------------------------------------------------
impl From<::std::env::VarError> for Error {
    fn from(e: ::std::env::VarError) -> Self {
        Error::EnvVar(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IO(e)
    }
}
// ----------------------------------------------------------------------------
impl<T> From<::std::sync::mpsc::SendError<T>> for Error {
    fn from(e: ::std::sync::mpsc::SendError<T>) -> Self {
        Error::Send(format!("{:?}", e))
    }
}
// ----------------------------------------------------------------------------
impl From<::std::num::ParseIntError> for Error {
    fn from(e: ::std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}
// ----------------------------------------------------------------------------
impl<T> From<::std::sync::PoisonError<T>> for Error {
    fn from(e: ::std::sync::PoisonError<T>) -> Self {
        Error::SyncPoison(format!("{:?}", e))
    }
}
// ----------------------------------------------------------------------------
impl From<::std::str::Utf8Error> for Error {
    fn from(e: ::std::str::Utf8Error) -> Self {
        Error::Utf8(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::uuid::ParseError> for Error {
    fn from(e: ::uuid::ParseError) -> Self {
        Error::UuidParse(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::regex::Error> for Error {
    fn from(e: ::regex::Error) -> Self {
        Error::Regex(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::getopts::Fail> for Error {
    fn from(e: ::getopts::Fail) -> Self {
        Error::GetOpts(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::serde_json::Error> for Error {
    fn from(e: ::serde_json::Error) -> Self {
        Error::SerdeJSON(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::toml::ser::Error> for Error {
    fn from(e: ::toml::ser::Error) -> Self {
        Error::TOMLSer(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::toml::de::Error> for Error {
    fn from(e: ::toml::de::Error) -> Self {
        Error::TOMLDe(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::rusty_leveldb::Status> for Error {
    fn from(e: ::rusty_leveldb::Status) -> Self {
        Error::LevelDB(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::discord::Error> for Error {
    fn from(e: ::discord::Error) -> Self {
        Error::Discord(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::elicit::Error> for Error {
    fn from(e: ::elicit::Error) -> Self {
        Error::Elicit(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptionNone => "::std::option::NoneError",
            Error::EnvVar(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::Send(_) => "::aizuna::Error::Send",
            Error::ParseInt(ref e) => e.description(),
            Error::SyncPoison(_) => "::aizuna::Error::SyncPoison",
            Error::Utf8(ref e) => e.description(),
            Error::UuidParse(_) => "::aizuna::Error::UuidParse",
            Error::Regex(ref e) => e.description(),
            Error::GetOpts(ref e) => e.description(),
            Error::SerdeJSON(ref e) => e.description(),
            Error::TOMLSer(ref e) => e.description(),
            Error::TOMLDe(ref e) => e.description(),
            Error::LevelDB(ref e) => e.description(),
            Error::Discord(ref e) => e.description(),
            Error::Elicit(ref e) => e.description(),
            Error::SerDeVer(_, _, _) => "::aizuna::Error::SerDeVer",
            Error::Downcast(_) => "::aizuna::Error::Downcast",
            Error::MissingField(_) => "::aizuna::Error::MissingField",
            Error::InvalidType(_) => "::aizuna::Error::InvalidType",
            Error::InvalidArg(_) => "::aizuna::Error::InvalidArg",
            Error::NoConfig => "::aizuna::Error::NoConfig",
            Error::Connector(_) => "::aizuna::Error::Connector",
            Error::Aizuna(_) => "::aizuna::Error::Aizuna",
            Error::AizunaDBVer(_, _, _) => "::aizuna::Error::AizunaDBVer",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::OptionNone => None,
            Error::EnvVar(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::Send(_) => None,
            Error::ParseInt(ref e) => Some(e),
            Error::SyncPoison(_) => None,
            Error::Utf8(ref e) => Some(e),
            Error::UuidParse(_) => None,
            Error::Regex(ref e) => Some(e),
            Error::GetOpts(ref e) => Some(e),
            Error::SerdeJSON(ref e) => Some(e),
            Error::TOMLSer(ref e) => Some(e),
            Error::TOMLDe(ref e) => Some(e),
            Error::LevelDB(ref e) => Some(e),
            Error::Discord(ref e) => Some(e),
            Error::Elicit(ref e) => Some(e),
            Error::SerDeVer(_, _, _) => None,
            Error::MissingField(_) => None,
            Error::Downcast(_) => None,
            Error::InvalidType(_) => None,
            Error::InvalidArg(_) => None,
            Error::NoConfig => None,
            Error::Connector(_) => None,
            Error::Aizuna(_) => None,
            Error::AizunaDBVer(_, _, _) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
