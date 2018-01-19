// -*- mode:rust; coding:utf-8-unix; -*-

//! session.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/04
//  @date 2018/01/19

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::{Borrow, Cow};
use std::iter::{Chain, FromIterator};
// ----------------------------------------------------------------------------
use chrono::prelude::{DateTime, Local, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{Error, Result, SessionKind};
use super::super::FormatIndent;
use super::super::uuid_set::UuidSet;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Flags: u32 {
    const OPEN  = 0b00000001u32 <<  0;
    }
}
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::OPEN
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Session
pub trait Session<'a>: ::std::fmt::Debug + Serialize + Deserialize<'a> {
    // ========================================================================
    /// fn as_rule_name
    fn as_rule_name(&self) -> &str;
}
// ============================================================================
/// macro_rules! session_delegate
#[macro_export]
macro_rules! session_delegate {
    ($field:ident)          => {
        // ====================================================================
        fn as_rule_name(&self) -> &str { self.$field.as_rule_name() }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct SessionImpl
#[derive(Debug, Clone)]
pub struct SessionImpl {
    /// uuid
    uuid: Uuid,
    /// owners
    owners: UuidSet,
    /// member
    member: UuidSet,
    /// utc
    utc: DateTime<Utc>,
    /// title
    title: String,
    /// flags
    flags: Flags,
    /// kind
    kind: SessionKind,
}
// ============================================================================
impl<'a> Session<'a> for SessionImpl {
    session_delegate!(kind);
}
// ============================================================================
impl AsRef<Uuid> for SessionImpl {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl Borrow<Uuid> for SessionImpl {
    fn borrow(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl ::std::fmt::Display for SessionImpl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_idt(f, 0usize)
    }
}
// ============================================================================
impl FormatIndent for SessionImpl {
    fn fmt_idt(
        &self,
        f: &mut ::std::fmt::Formatter,
        idt: usize,
    ) -> ::std::fmt::Result {
        let _ = write!(
            f,
            r##"{e:>idt0$}Session {{
{e:>idt1$}uuid:         {uuid},
{e:>idt1$}create:       {create},
{e:>idt1$}title:        {title},
{e:>idt1$}owners:
"##,
            e = "",
            idt0 = idt,
            idt1 = idt + 2usize,
            uuid = self.uuid,
            create = self.with_local(),
            title = self.title,
        )?;
        let _ = self.owners.fmt_idt(f, idt + 2usize)?;
        let _ = write!(
            f,
            r##"
{e:>idt1$}member:
"##,
            e = "",
            idt1 = idt + 2usize,
        )?;
        let _ = self.member.fmt_idt(f, idt + 2usize)?;
        write!(
            f,
            r##"
{e:>idt1$}open:         {open},
{e:>idt1$}kind:         {kind},
{e:>idt0$}}}"##,
            e = "",
            idt0 = idt,
            idt1 = idt + 2usize,
            open = self.is_open(),
            kind = self.as_rule_name(),
        )
    }
}
// ============================================================================
impl SessionImpl {
    // ========================================================================
    /// fn new
    pub fn new<U, IO>(uuid: U, owners: IO, kind: SessionKind) -> Self
    where
        Uuid: From<U>,
        IO: IntoIterator<Item = Uuid>,
    {
        SessionImpl {
            uuid: Uuid::from(uuid),
            utc: Utc::now(),
            owners: UuidSet::from_iter(owners.into_iter()),
            member: UuidSet::default(),
            title: String::from("title"),
            flags: Flags::default(),
            kind: kind,
        }
    }
    // ========================================================================
    /// fn as_uuid
    pub fn as_uuid(&self) -> &Uuid {
        &self.uuid
    }
    // ========================================================================
    /// fn as_owners
    pub fn as_owners(&self) -> &UuidSet {
        &self.owners
    }
    // ------------------------------------------------------------------------
    /// fn as_owners_mut
    pub fn as_owners_mut(&mut self) -> &mut UuidSet {
        &mut self.owners
    }
    // ========================================================================
    /// fn as_member
    pub fn as_member(&self) -> &UuidSet {
        &self.member
    }
    // ------------------------------------------------------------------------
    /// fn as_member_mut
    pub fn as_member_mut(&mut self) -> &mut UuidSet {
        &mut self.member
    }
    // ========================================================================
    /// fn owners_member_iter
    pub fn owners_member_iter(
        &self,
    ) -> Chain<super::super::uuid_set::Iter, super::super::uuid_set::Iter>
    {
        self.owners.iter().chain(self.member.iter())
    }
    // ------------------------------------------------------------------------
    /// fn owners_member_contains
    pub fn owners_member_contains<Q>(&self, x: &Q) -> bool
    where
        Uuid: Borrow<Q>,
        Q: Ord,
    {
        self.owners.contains(x) || self.member.contains(x)
    }
    // ========================================================================
    /// fn as_utc
    pub fn as_utc(&self) -> &DateTime<Utc> {
        &self.utc
    }
    // ------------------------------------------------------------------------
    /// fn with_local
    pub fn with_local(&self) -> DateTime<Local> {
        self.utc.with_timezone(&Local)
    }
    // ========================================================================
    /// fn as_title
    pub fn as_title(&self) -> &str {
        self.title.as_str()
    }
    // ------------------------------------------------------------------------
    /// fn set_title
    pub fn set_title<S>(&mut self, title: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.title = String::from(title.as_ref());
        self
    }
    // ========================================================================
    /// fn is_open
    pub fn is_open(&self) -> bool {
        self.flags.contains(Flags::OPEN)
    }
    // ------------------------------------------------------------------------
    /// fn open
    pub fn open(&mut self) -> &mut Self {
        self.flags.insert(Flags::OPEN);
        self
    }
    // ------------------------------------------------------------------------
    /// fn close
    pub fn close(&mut self) -> &mut Self {
        self.flags.remove(Flags::OPEN);
        self
    }
    // ========================================================================
    /// fn as_kind
    pub fn as_kind(&self) -> &SessionKind {
        &self.kind
    }
    // ------------------------------------------------------------------------
    /// fn as_kind_mut
    pub fn as_kind_mut(&mut self) -> &mut SessionKind {
        &mut self.kind
    }
}
// ============================================================================
impl Serialize for SessionImpl {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        debug!("SessionImpl::serialize");
        self::serialize::SessionImpl::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for SessionImpl {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        debug!("SessionImpl::deserialize");
        self::serialize::SessionImpl::deserialize(deserializer)?
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
    /// struct SessionImpl
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SessionImpl<'a> {
        /// serdever
        serdever: i32,
        /// uuid
        uuid: Option<Cow<'a, Uuid>>,
        /// utc
        utc: Option<Cow<'a, DateTime<Utc>>>,
        /// owners
        owners: Option<Cow<'a, UuidSet>>,
        /// member
        member: Option<Cow<'a, UuidSet>>,
        /// title
        title: Option<Cow<'a, str>>,
        /// flags
        flags: Option<Flags>,
        /// kind
        kind: Option<Cow<'a, SessionKind>>,
    }
    // ========================================================================
    impl<'a> SessionImpl<'a> {
        // ====================================================================
        /// into
        pub fn into(self) -> Result<super::SessionImpl> {
            debug!("SessionImpl::serialize::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            Ok(super::SessionImpl {
                uuid: self.uuid
                    .ok_or(Error::MissingField(String::from(
                        "SessionImpl::serialize::uuid",
                    )))?
                    .into_owned(),
                utc: self.utc.map_or(Utc::now(), Cow::into_owned),
                owners: self.owners
                    .map_or(UuidSet::default(), Cow::into_owned),
                member: self.member
                    .map_or(UuidSet::default(), Cow::into_owned),
                title: self.title
                    .map_or(String::from("title"), Cow::into_owned),
                flags: self.flags.unwrap_or(Flags::default()),
                kind: self.kind
                    .ok_or(Error::MissingField(String::from(
                        "SessionImpl::serialize::kind",
                    )))?
                    .into_owned(),
            })
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::SessionImpl> for SessionImpl<'a> {
        fn from(src: &'a super::SessionImpl) -> Self {
            debug!("SessionImpl::serialize::from");
            Self {
                serdever: CURRENT,
                uuid: Some(Cow::Borrowed(&src.uuid)),
                utc: Some(Cow::Borrowed(src.as_utc())),
                owners: if src.owners.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.owners))
                },
                member: if src.member.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.member))
                },
                title: Some(Cow::Borrowed(src.as_title())),
                flags: Some(src.flags),
                kind: Some(Cow::Borrowed(src.as_kind())),
            }
        }
    }
}
