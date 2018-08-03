// -*- mode:rust; coding:utf-8-unix; -*-

//! session_kind.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/04
//  @date 2018/04/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::Cow;
// ----------------------------------------------------------------------------
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// ----------------------------------------------------------------------------
use super::super::FormatIndent;
use super::rule::shinen::Session as ShinEnSession;
use super::{Error, Result, Session};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum SessionKind
#[derive(Debug, Clone)]
pub(crate) enum SessionKind {
    /// Dummy
    Dummy,
    /// ShinEn
    ShinEn(ShinEnSession),
}
// ============================================================================
impl SessionKind {
    // ========================================================================
    /// fn as_shinen
    pub(crate) fn as_shinen(&self) -> Option<&ShinEnSession> {
        match *self {
            SessionKind::ShinEn(ref x) => Some(x),
            _ => None,
        }
    }
    // ------------------------------------------------------------------------
    /// fn as_shinen_mut
    pub(crate) fn as_shinen_mut(&mut self) -> Option<&mut ShinEnSession> {
        match *self {
            SessionKind::ShinEn(ref mut x) => Some(x),
            _ => None,
        }
    }
}
// ============================================================================
impl ::std::fmt::Display for SessionKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_idt(f, 0usize)
    }
}
// ============================================================================
impl FormatIndent for SessionKind {
    fn fmt_idt(
        &self,
        f: &mut ::std::fmt::Formatter,
        idt: usize,
    ) -> ::std::fmt::Result {
        match *self {
            SessionKind::ShinEn(ref x) => x.fmt_idt(f, idt),
            _ => f.write_str("dummy"),
        }
    }
}
// ============================================================================
impl<'a> Session<'a> for SessionKind {
    // ========================================================================
    fn as_rule_name(&self) -> &str {
        match *self {
            SessionKind::ShinEn(ref x) => x.as_rule_name(),
            _ => "dummy",
        }
    }
}
// ============================================================================
impl Serialize for SessionKind {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize::SessionKind::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for SessionKind {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::serialize::SessionKind::deserialize(deserializer)?
            .into()
            .map_err(|e| ::serde::de::Error::custom(format!("{}", e)))
    }
}
// ============================================================================
mod serialize {
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    use super::*;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    const CURRENT: i32 = 0i32;
    const AGE: i32 = 0i32;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct SessionKind
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct SessionKind<'a> {
        /// serdever
        serdever: i32,
        /// rule
        rule: Option<Cow<'a, str>>,
        /// dump
        dump: Option<String>,
    }
    // ========================================================================
    impl<'a> SessionKind<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::SessionKind> {
            debug!("::aizuna::SessionKind::serialize::serialize::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            let rule = self
                .rule
                .ok_or_else(|| {
                    Error::MissingField(String::from(
                        "::aizuna::SessionKind::serialize::rule",
                    ))
                })?.into_owned();
            let dump = self.dump.ok_or_else(|| {
                Error::MissingField(String::from(
                    "::aizuna::SessionKind::serialize::dump",
                ))
            })?;
            match rule.as_str() {
                "shinen" => if let Ok(x) =
                    ::serde_json::from_str::<ShinEnSession>(&dump)
                {
                    Ok(super::SessionKind::ShinEn(x))
                } else {
                    Err(Error::Aizuna(String::from(
                        "::aizuna::SessionKind::serialize: unsupported kind.",
                    )))
                },
                _ => Err(Error::Aizuna(format!(
                    "::aizuna::SessionKind::serialize: unsupported kind. {}",
                    rule
                ))),
            }
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::SessionKind> for SessionKind<'a> {
        fn from(src: &'a super::SessionKind) -> Self {
            debug!("::aizuna::SessionKind::serialize::serialize::from");
            Self {
                serdever: CURRENT,
                rule: Some(From::from(src.as_rule_name())),
                dump: Some(
                    match *src {
                        super::SessionKind::ShinEn(ref x) => {
                            ::serde_json::to_string(x)
                                .expect("\
::aizuna::SessionKind::serialize::from ::serde_json::to_string")
                        }
                        super::SessionKind::Dummy => String::from("dummy"),
                    }
                ),
            }
        }
    }
}
