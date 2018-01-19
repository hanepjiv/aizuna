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
use toml::Value;
use toml::value::*;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Config
#[derive(Debug, Clone)]
pub struct Config {
    /// enable
    pub enable: bool,
    /// connector
    pub connector: String,
    /// config
    pub config: Value,
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
pub mod serialize {
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
    pub struct Config<'a> {
        /// serdever
        serdever: i32,
        /// enable
        enable: Option<bool>,
        /// connector
        connector: Option<Cow<'a, str>>,
        /// config
        config: Option<Cow<'a, Value>>,
    }
    // ========================================================================
    impl<'a> Config<'a> {
        // ====================================================================
        /// into
        pub fn into(self) -> Result<super::Config> {
            debug!("::aizuna::connector::Config::serialize::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            match self.serdever {
                0 => Ok(super::Config {
                    enable: self.enable.unwrap_or(true),
                    connector: self.connector
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
            debug!("::aizuna::connector::Config::serialize::from");
            Self {
                serdever: CURRENT,
                enable: Some(src.enable),
                connector: Some(From::from(src.connector.as_str())),
                config: Some(Cow::Borrowed(&src.config)),
            }
        }
    }
}
