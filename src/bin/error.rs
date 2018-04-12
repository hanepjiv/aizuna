// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/30
//  @date 2017/12/31

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(variant_size_differences)]
#[derive(Debug)]
pub(crate) enum Error {
    /// OptionNone
    OptionNone,
    /// SetLogger
    SetLogger(::log::SetLoggerError),
    /// GetOpts
    GetOpts(::getopts::Fail),
    /// Aizuna
    Aizuna(::aizuna::Error),
}
// ============================================================================
impl From<::std::option::NoneError> for Error {
    fn from(_: ::std::option::NoneError) -> Self {
        Error::OptionNone
    }
}
// ----------------------------------------------------------------------------
impl From<::log::SetLoggerError> for Error {
    fn from(e: ::log::SetLoggerError) -> Self {
        Error::SetLogger(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::getopts::Fail> for Error {
    fn from(e: ::getopts::Fail) -> Self {
        Error::GetOpts(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::aizuna::Error> for Error {
    fn from(e: ::aizuna::Error) -> Self {
        Error::Aizuna(e)
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
            Error::SetLogger(ref e) => e.description(),
            Error::GetOpts(ref e) => e.description(),
            Error::Aizuna(ref e) => e.description(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::OptionNone => None,
            Error::SetLogger(ref e) => Some(e),
            Error::GetOpts(ref e) => Some(e),
            Error::Aizuna(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
