// -*- mode:rust; coding:utf-8-unix; -*-

//! config.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/05
//  @date 2018/01/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::Cow;
// ----------------------------------------------------------------------------
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use toml::value::*;
use toml::Value;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Config
#[derive(Debug, Clone)]
pub(crate) struct Config {
    /// enable
    pub(crate) enable: bool,
    /// prefix
    pub(crate) prefix: String,
    /// config
    pub(crate) config: Value,
}
// ============================================================================
impl Config {}
// ============================================================================
impl Serialize for Config {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize::Config::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::serialize::Config::deserialize(deserializer)?
            .into()
            .map_err(|e| ::serde::de::Error::custom(format!("{}", e)))
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// serialize
pub(crate) mod serialize {
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    use super::*;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    const CURRENT: i32 = 0i32;
    const AGE: i32 = 0i32;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct Config
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct Config<'a> {
        /// serdever
        serdever: i32,
        /// enable
        enable: Option<bool>,
        /// prefix
        prefix: Option<Cow<'a, str>>,
        /// config
        config: Option<Cow<'a, Value>>,
    }
    // ========================================================================
    impl<'a> Config<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::Config> {
            debug!("::aizuna::prefix::Config::serialize::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            match self.serdever {
                0 => Ok(super::Config {
                    enable: self.enable.unwrap_or(true),
                    prefix: self
                        .prefix
                        .map_or(String::from(","), String::from),
                    config: self.config.map_or(
                        Value::Table(Table::default()),
                        Cow::into_owned,
                    ),
                }),
                _ => Err(Error::SerDeVer(self.serdever, CURRENT, AGE)),
            }
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::Config> for Config<'a> {
        fn from(src: &'a super::Config) -> Self {
            debug!("::aizuna::prefix::Config::serialize::from");
            Self {
                serdever: CURRENT,
                enable: Some(src.enable),
                prefix: Some(From::from(src.prefix.as_str())),
                config: Some(Cow::Borrowed(&src.config)),
            }
        }
    }
}
