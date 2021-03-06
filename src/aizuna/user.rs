// -*- mode:rust; coding:utf-8-unix; -*-

//! user.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/25
//  @date 2018/08/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{borrow::Cow, collections::BTreeSet};
// ----------------------------------------------------------------------------
use log::debug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct User
#[derive(Debug, Clone)]
pub(crate) struct User<'a> {
    /// uuid
    uuid: Uuid,
    /// connector_id
    connector_id: Cow<'a, str>,
    /// author_id
    author_id: Cow<'a, str>,
    /// author_name
    author_name: Cow<'a, str>,
    /// alias
    alias: BTreeSet<String>,
    /// admin
    admin: bool,
}
// ============================================================================
impl<'a> User<'a> {
    // ========================================================================
    /// fn make_id
    pub(crate) fn make_id(
        connector_id: impl AsRef<str>,
        author_id: impl AsRef<str>,
    ) -> String {
        let mut id = String::from(connector_id.as_ref());
        id.push(':');
        id += author_id.as_ref();
        id
    }
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        uuid: impl Into<Uuid>,
        connector_id: &'a str,
        author_id: &'a str,
        author_name: &'a str,
        admin: bool,
    ) -> Self {
        User {
            uuid: uuid.into(),
            alias: BTreeSet::default(),
            connector_id: Cow::Borrowed(connector_id),
            author_id: Cow::Borrowed(author_id),
            author_name: Cow::Borrowed(author_name),
            admin,
        }
    }
    // ========================================================================
    /// fn as_uuid
    pub(crate) fn as_uuid(&self) -> &Uuid {
        &self.uuid
    }
    // ========================================================================
    /// fn as_connector_id
    pub(crate) fn as_connector_id(&self) -> &str {
        &self.connector_id
    }
    // ========================================================================
    /// fn as_author_id
    pub(crate) fn as_author_id(&self) -> &str {
        &self.author_id
    }
    // ========================================================================
    /// fn as_author_name
    pub(crate) fn as_author_name(&self) -> &str {
        &self.author_name
    }
    // ------------------------------------------------------------------------
    /// fn set_author_name
    pub(crate) fn set_author_name(
        &mut self,
        author_name: &'a impl AsRef<str>,
    ) {
        self.author_name = Cow::Borrowed(author_name.as_ref());
    }
    // ========================================================================
    /// fn as_alias
    pub(crate) fn as_alias(&self) -> &BTreeSet<String> {
        &self.alias
    }
    // ========================================================================
    /// fn as_admin
    pub(crate) fn as_admin(&self) -> &bool {
        &self.admin
    }
    // ------------------------------------------------------------------------
    /// fn set_admin
    pub(crate) fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }
}
// ============================================================================
impl<'a> Serialize for User<'a> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        debug!("User::serialize");
        self::serialize::User::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'a, 'de> Deserialize<'de> for User<'a> {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        debug!("User::deserialize");
        self::serialize::User::deserialize(deserializer)?
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
    /// struct User
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct User<'a> {
        /// serdever
        serdever: i32,
        /// uuid
        uuid: Option<Cow<'a, Uuid>>,
        /// connector_id
        connector_id: Option<Cow<'a, str>>,
        /// author_id
        author_id: Option<Cow<'a, str>>,
        /// author_name
        author_name: Option<Cow<'a, str>>,
        /// alias
        alias: Option<Cow<'a, BTreeSet<String>>>,
    }
    // ========================================================================
    impl<'a> User<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::User<'a>> {
            debug!("User::serialize::into");
            if (self.serdever < (CURRENT - AGE)) || (CURRENT < self.serdever) {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            let connector_id = self.connector_id.ok_or_else(|| {
                Error::MissingField(String::from(
                    "::aizuna::User::serialize::connector?id",
                ))
            })?;
            let author_id = self.author_id.ok_or_else(|| {
                Error::MissingField(String::from(
                    "::aizuna::User::serialize::author_id",
                ))
            })?;
            let author_name = self.author_name.ok_or_else(|| {
                Error::MissingField(String::from(
                    "::aizuna::User::serialize::author_name",
                ))
            })?;
            Ok(super::User {
                uuid: self
                    .uuid
                    .ok_or_else(|| {
                        Error::MissingField(String::from(
                            "::aizuna::User::serialize::uuid",
                        ))
                    })?
                    .into_owned(),
                connector_id,
                author_id,
                author_name,
                alias: self.alias.map(Cow::into_owned).unwrap_or_default(),
                admin: false,
            })
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::User<'a>> for User<'a> {
        fn from(src: &'a super::User<'a>) -> Self {
            debug!("User::serialize::from");
            Self {
                serdever: CURRENT,
                uuid: Some(Cow::Borrowed(&src.uuid)),
                connector_id: Some(src.connector_id.clone()),
                author_id: Some(src.author_id.clone()),
                author_name: Some(src.author_name.clone()),
                alias: Some(Cow::Borrowed(&src.alias)),
            }
        }
    }
}
