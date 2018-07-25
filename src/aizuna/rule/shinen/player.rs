// -*- mode:rust; coding:utf-8-unix; -*-

//! player.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/14
//  @date 2018/05/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::super::super::super::FormatIndent;
use super::{CardMap, Hand};
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum PlayerType
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub(crate) enum PlayerType {
    /// Player
    Player,
    /// GameMaster
    GameMaster,
}
// ============================================================================
impl AsRef<str> for PlayerType {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerType::Player => "Player",
            PlayerType::GameMaster => "GameMaster",
        }
    }
}
// ============================================================================
impl ::std::str::FromStr for PlayerType {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        match src.as_bytes() {
            b"player" | b"Player" => Ok(PlayerType::Player),
            b"gm" | b"GM" | b"gamemaster" | b"GameMaster" | b"master"
            | b"Master" => Ok(PlayerType::GameMaster),
            _ => Err(Error::Aizuna(format!(
                "PlayerType::from_str: failed {}",
                src
            ))),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Player
#[derive(Debug, Clone, Eq)]
pub(crate) struct Player {
    /// uuid
    uuid: Uuid,
    /// user_uuid
    user_uuid: Uuid,
    /// name
    name: String,
    /// player_type
    player_type: PlayerType,
    /// hand
    hand: Hand,
}
// ============================================================================
impl ::std::fmt::Display for Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_idt(f, 0usize)
    }
}
// ============================================================================
impl FormatIndent for Player {
    fn fmt_idt(
        &self,
        f: &mut ::std::fmt::Formatter,
        idt: usize,
    ) -> ::std::fmt::Result {
        let s0 = <Self as FormatIndent>::make_idt(idt);
        let s1 = <Self as FormatIndent>::make_idt(idt + 2usize);
        write!(
            f,
            r##"{s0}Player {{
{s1}uuid:         {uuid},
{s1}user_uuid:    {user_uuid},
{s1}name:         {name},
{s1}player_type:  {player_type:?},
{s1}hand:         {hand}
{s0}}}"##,
            s0 = s0,
            s1 = s1,
            uuid = self.uuid,
            user_uuid = self.user_uuid,
            name = self.name,
            player_type = self.player_type,
            hand = self.hand.len(),
        )
    }
}
// ============================================================================
impl ::std::cmp::PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.uuid.eq(&other.uuid)
    }
}
// ----------------------------------------------------------------------------
impl ::std::cmp::PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}
// ----------------------------------------------------------------------------
impl ::std::cmp::Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uuid.cmp(&other.uuid)
    }
}
// ============================================================================
impl Borrow<Uuid> for Player {
    fn borrow(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<Uuid> for Player {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl Player {
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        uuid: impl Into<Uuid>,
        user_uuid: impl Into<Uuid>,
        name: impl Into<String>,
        player_type: PlayerType,
    ) -> Self {
        Player {
            uuid: uuid.into(),
            user_uuid: user_uuid.into(),
            name: name.into(),
            player_type,
            hand: Hand::default(),
        }
    }
    // ========================================================================
    /// fn as_uuid
    pub(crate) fn as_uuid(&self) -> &Uuid {
        &self.uuid
    }
    // ========================================================================
    /// fn as_user_uuid
    pub(crate) fn as_user_uuid(&self) -> &Uuid {
        &self.user_uuid
    }
    // ------------------------------------------------------------------------
    /// fn set_user_uuid
    pub(crate) fn set_user_uuid(&mut self, uuid: Uuid) -> &mut Self {
        self.user_uuid = uuid;
        self
    }
    // ========================================================================
    /// fn as_name
    pub(crate) fn as_name(&self) -> &str {
        &self.name
    }
    // ------------------------------------------------------------------------
    /// fn set_name
    pub(crate) fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }
    // ========================================================================
    /// fn as_player_type
    pub(crate) fn as_player_type(&self) -> &PlayerType {
        &self.player_type
    }
    // ------------------------------------------------------------------------
    /// fn set_player_type
    pub(crate) fn set_player_type(
        &mut self,
        player_type: PlayerType,
    ) -> &mut Self {
        self.player_type = player_type;
        self
    }
    // ========================================================================
    /// fn as_hand
    pub(crate) fn as_hand(&self) -> &Hand {
        &self.hand
    }
    // ------------------------------------------------------------------------
    /// fn as_hand_mut
    pub(crate) fn as_hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }
    // ------------------------------------------------------------------------
    /// fn hand_to_string
    pub(crate) fn hand_to_string(&self, cards: &CardMap) -> Option<String> {
        self.hand.to_string(cards)
    }
}
// ============================================================================
impl Serialize for Player {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize::Player::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::serialize::Player::deserialize(deserializer)?
            .into()
            .map_err(|e| ::serde::de::Error::custom(format!("{}", e)))
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// mod serialize
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
    /// struct Player
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct Player<'a> {
        /// serdever
        serdever: i32,
        /// uuid
        uuid: Option<Cow<'a, Uuid>>,
        /// user_uuid
        user_uuid: Option<Cow<'a, Uuid>>,
        /// name
        name: Option<Cow<'a, str>>,
        /// player_type
        player_type: Option<Cow<'a, PlayerType>>,
        /// hand
        hand: Option<Cow<'a, Hand>>,
    }
    // ========================================================================
    impl<'a> Player<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::Player> {
            debug!("::shinen::Player::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            Ok(super::Player {
                uuid: self
                    .uuid
                    .ok_or_else(|| {
                        Error::MissingField(String::from(
                            "::shinen::Player::serialize::uuid",
                        ))
                    })?
                    .into_owned(),
                user_uuid: self
                    .user_uuid
                    .ok_or_else(|| {
                        Error::MissingField(String::from(
                            "::shinen::Player::serialize::user_uuid",
                        ))
                    })?
                    .into_owned(),
                name: self.name.map_or(String::default(), Cow::into_owned),
                player_type: self
                    .player_type
                    .ok_or_else(|| {
                        Error::MissingField(String::from(
                            "::shinen::Player::serialize::player_type",
                        ))
                    })?
                    .into_owned(),
                hand: self.hand.map_or(Hand::default(), Cow::into_owned),
            })
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::Player> for Player<'a> {
        fn from(src: &'a super::Player) -> Self {
            debug!("::shinen::Player::from");
            Self {
                serdever: CURRENT,
                uuid: Some(Cow::Borrowed(&src.uuid)),
                user_uuid: Some(Cow::Borrowed(&src.user_uuid)),
                name: if src.name.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.name))
                },
                player_type: Some(Cow::Borrowed(&src.player_type)),
                hand: if src.hand.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.hand))
                },
            }
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type PlayerMap
pub(crate) type PlayerMap = BTreeMap<Uuid, Player>;
