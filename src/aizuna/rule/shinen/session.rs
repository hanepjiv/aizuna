// -*- mode:rust; coding:utf-8-unix; -*-

//! session.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/14
//  @date 2018/08/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::{Borrow, Cow};
use std::cmp::Ord;
use std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use log::debug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::super::super::super::FormatIndent;
use super::super::super::Session as AizunaSession;
use super::{Deck, Error, Player, PlayerMap, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type DefaultPlayerMap
type DefaultPlayerMap = BTreeMap<Uuid, Uuid>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Session
#[derive(Debug, Clone)]
pub(crate) struct Session {
    /// pile
    pile: Deck,
    /// discard
    discard: Deck,
    /// players
    players: PlayerMap,
    /// default_player
    default_player: DefaultPlayerMap,
}
// ============================================================================
impl ::std::fmt::Display for Session {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_idt(f, 0usize)
    }
}
// ============================================================================
impl FormatIndent for Session {
    fn fmt_idt(
        &self,
        f: &mut ::std::fmt::Formatter,
        idt: usize,
    ) -> ::std::fmt::Result {
        let s0 = <Self as FormatIndent>::make_idt(idt);
        let s1 = <Self as FormatIndent>::make_idt(idt + 2usize);
        writeln!(
            f,
            r##"{s0}ShinEn {{
{s1}pile:         {pile:?},
{s1}discard:      {discard},
{s1}players:"##,
            s0 = s0,
            s1 = s1,
            pile = self.pile.len(),
            discard = self.discard.len(),
        )?;
        self.players.fmt_idt(f, idt + 2usize)?;
        write!(f, r##"\n{s0}}}"##, s0 = s0,)
    }
}
// ============================================================================
impl<'a> AizunaSession<'a> for Session {
    fn as_rule_name(&self) -> &str {
        "shinen"
    }
}
// ============================================================================
impl Session {
    // ========================================================================
    /// fn new
    pub(crate) fn new(pile: Deck) -> Self {
        Session {
            pile,
            discard: Deck::default(),
            players: PlayerMap::default(),
            default_player: DefaultPlayerMap::default(),
        }
    }
    // ========================================================================
    /// fn as_pile
    pub(crate) fn as_pile(&self) -> &Deck {
        &self.pile
    }
    // ========================================================================
    /// fn as_discard
    pub(crate) fn as_discard(&self) -> &Deck {
        &self.discard
    }
    // ------------------------------------------------------------------------
    /// fn as_discard_mut
    pub(crate) fn as_discard_mut(&mut self) -> &mut Deck {
        &mut self.discard
    }
    // ========================================================================
    /// fn as_players
    pub(crate) fn as_players(&self) -> &PlayerMap {
        &self.players
    }
    // ------------------------------------------------------------------------
    /// fn as_players_mut
    pub(crate) fn as_players_mut(&mut self) -> &mut PlayerMap {
        &mut self.players
    }
    // ========================================================================
    /// fn default_player
    pub(crate) fn as_default_player<Q>(&self, user: &Q) -> Option<&Player>
    where
        Q: Ord + ?Sized,
        Uuid: Borrow<Q>,
    {
        if let Some(x) = self.default_player.get(user) {
            self.players.get::<Uuid>(x)
        } else {
            None
        }
    }
    // ------------------------------------------------------------------------
    /// fn default_player_mut
    pub(crate) fn as_default_player_mut<Q>(
        &mut self,
        user: &Q,
    ) -> Option<&mut Player>
    where
        Q: Ord + ?Sized,
        Uuid: Borrow<Q>,
    {
        if let Some(x) = self.default_player.get(user) {
            self.players.get_mut::<Uuid>(x)
        } else {
            None
        }
    }
    // ------------------------------------------------------------------------
    /// fn insert_default_player
    pub(crate) fn insert_default_player(
        &mut self,
        key: Uuid,
        val: Uuid,
    ) -> Option<Uuid> {
        self.default_player.insert(key, val)
    }
    // ========================================================================
    /// fn shuffle
    pub(crate) fn shuffle(&mut self) {
        self.pile.shuffle()
    }
    // ------------------------------------------------------------------------
    /// fn tsukimachi
    pub(crate) fn tsukimachi(&mut self) {
        self.pile.append(&mut self.discard);
        self.pile.shuffle()
    }
    // ------------------------------------------------------------------------
    /// fn draw
    pub(crate) fn draw(&mut self) -> Option<String> {
        self.pile.pop_front()
    }
    // ------------------------------------------------------------------------
    /// fn pick
    pub(crate) fn pick(&mut self, v: &[u8]) -> Option<String> {
        if let Some(x) = self.discard.pick(v) {
            Some(x)
        } else if let Some(x) = self.pile.pick(v) {
            Some(x)
        } else {
            None
        }
    }
    // ------------------------------------------------------------------------
    /// fn discard
    pub(crate) fn discard(&mut self, c: String) {
        self.discard.push_front(c)
    }
    // ------------------------------------------------------------------------
    /// fn totop
    pub(crate) fn totop(&mut self, c: String) {
        self.pile.push_front(c)
    }
}
// ============================================================================
impl Serialize for Session {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize::Session::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for Session {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::serialize::Session::deserialize(deserializer)?
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
    /// struct Session
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct Session<'a> {
        /// serdever
        serdever: i32,
        /// pile
        pile: Option<Cow<'a, Deck>>,
        /// discard
        discard: Option<Cow<'a, Deck>>,
        /// players
        players: Option<Cow<'a, PlayerMap>>,
        /// default_player
        default_player: Option<Cow<'a, DefaultPlayerMap>>,
    }
    // ========================================================================
    impl<'a> Session<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::Session> {
            debug!("Session::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            Ok(super::Session {
                pile: self.pile.map_or(Deck::default(), Cow::into_owned),
                discard: self.discard.map_or(Deck::default(), Cow::into_owned),
                players: self
                    .players
                    .map_or(PlayerMap::default(), Cow::into_owned),
                default_player: self
                    .default_player
                    .map_or(DefaultPlayerMap::default(), Cow::into_owned),
            })
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::Session> for Session<'a> {
        fn from(src: &'a super::Session) -> Self {
            Self {
                serdever: CURRENT,
                pile: if src.pile.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.pile))
                },
                discard: if src.discard.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.discard))
                },
                players: if src.players.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.players))
                },
                default_player: if src.default_player.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(&src.default_player))
                },
            }
        }
    }
}
