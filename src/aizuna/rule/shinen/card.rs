// -*- mode:rust; coding:utf-8-unix; -*-

//! card.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/14
//  @date 2018/05/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::Cow;
use std::collections::vec_deque;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::File;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::vec::Vec;
// ----------------------------------------------------------------------------
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// ----------------------------------------------------------------------------
use super::{
    Action, CardSet, Color, Constellation, Damage, Error, Result, Story,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Card
#[derive(Debug, Clone)]
pub(crate) struct Card {
    /// name
    name: String,
    /// card_set
    card_set: CardSet,
    /// color
    color: Option<Color>,
    /// constellation
    constellation: Option<Constellation>,
    /// value
    value: Option<i32>,
    /// desc
    desc: String,
    /// story
    story: Vec<Story>,
    /// story_desc
    story_desc: String,
    /// action
    action: Vec<Action>,
    /// damage
    damage: Option<Damage>,
    /// damage_desc
    damage_desc: String,
    /// destiny
    destiny: Option<i32>,
}
// ============================================================================
impl Card {
    // ========================================================================
    /// as_name
    pub(crate) fn as_name(&self) -> &str {
        &self.name
    }
    // ========================================================================
    /// as_color
    pub(crate) fn as_color(&self) -> &Option<Color> {
        &self.color
    }
    // ========================================================================
    /// as_desc
    pub(crate) fn as_desc(&self) -> &str {
        self.desc.as_str()
    }
    // ========================================================================
    /// as_story_desc
    pub(crate) fn as_story_desc(&self) -> &str {
        self.story_desc.as_str()
    }
    // ========================================================================
    /// as_damage_desc
    pub(crate) fn as_damage_desc(&self) -> &str {
        self.damage_desc.as_str()
    }
    // ========================================================================
    /// as_destiny
    pub(crate) fn as_destiny(&self) -> &Option<i32> {
        &self.destiny
    }
    // ========================================================================
    /// fn pretty
    pub(crate) fn pretty(&self) -> String {
        format!(
            "{name} / {card_set} / 色： {color} / 星座： {constellation} / \
             数値: {value} / 叙述: {desc} / 語り部: {story:?} {story_desc} / \
             アクション: {action:?} / ダメージ: {damage} {damage_desc} / \
             運命: {destiny}",
            name = self.name,
            card_set = self.card_set.as_str(),
            color = self.color
                .clone()
                .map(|x| String::from(x.as_str()))
                .unwrap_or_else(|| "-".to_string()),
            constellation = self.constellation
                .clone()
                .map(|x| String::from(x.as_str()))
                .unwrap_or_else(|| "-".to_string()),
            value = self.value.map(|x| x.to_string())
                .unwrap_or_else(|| "-".to_string()),
            desc = self.desc,
            story = self.story
                .iter()
                .map(|x| String::from(x.as_str()))
                .collect::<Vec<String>>(),
            story_desc = self.story_desc,
            action = self.action
                .iter()
                .map(|x| String::from(x.as_str()))
                .collect::<Vec<String>>(),
            damage = self.damage
                .clone()
                .map(|x| String::from(x.as_str()))
                .unwrap_or_else(|| "-".to_string()),
            damage_desc = self.damage_desc,
            destiny = self.destiny
                .map(|x| x.to_string())
                .unwrap_or_else(|| "-".to_string()),
        )
    }
}
// ============================================================================
impl Serialize for Card {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize::Card::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::serialize::Card::deserialize(deserializer)?
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
    /// struct Card
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct Card<'a> {
        /// serdever
        serdever: i32,
        /// name
        name: Option<Cow<'a, str>>,
        /// card_set
        card_set: Option<Cow<'a, str>>,
        /// color
        color: Option<Cow<'a, str>>,
        /// constellation
        constellation: Option<Cow<'a, str>>,
        /// value
        value: Option<i32>,
        /// desc
        desc: Option<Cow<'a, str>>,
        /// story
        story: Option<Vec<Cow<'a, str>>>,
        /// story_desc
        story_desc: Option<Cow<'a, str>>,
        /// action
        action: Option<Vec<Cow<'a, str>>>,
        /// damage
        damage: Option<Cow<'a, str>>,
        /// damage_desc
        damage_desc: Option<Cow<'a, str>>,
        /// destiny
        destiny: Option<i32>,
    }
    // ========================================================================
    impl<'a> Card<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::Card> {
            debug!("::shinen::Card::serialize::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            match self.serdever {
                0 => Ok(super::Card {
                    name: self
                        .name
                        .ok_or_else(|| {
                            Error::MissingField(String::from(
                                "::shinen::Card::serialize::name",
                            ))
                        })?
                        .into_owned(),
                    card_set: self
                        .card_set
                        .ok_or_else(|| {
                            Error::MissingField(String::from(
                                "::shinen::Card::serialize::card_set",
                            ))
                        })?
                        .parse()?,
                    color: if let Some(x) = self.color {
                        Some(x.parse()?)
                    } else {
                        None
                    },
                    constellation: if let Some(x) = self.constellation {
                        Some(x.parse()?)
                    } else {
                        None
                    },
                    value: self.value,
                    desc: self.desc.map_or(String::default(), Cow::into_owned),
                    story: if let Some(x) = self.story {
                        let mut ret = Vec::<Story>::default();
                        for i in x {
                            ret.push(i.parse()?);
                        }
                        ret
                    } else {
                        Vec::<Story>::default()
                    },
                    story_desc: self
                        .story_desc
                        .map_or(String::default(), Cow::into_owned),
                    action: if let Some(x) = self.action {
                        let mut ret = Vec::<Action>::default();
                        for i in x {
                            ret.push(i.parse()?);
                        }
                        ret
                    } else {
                        Vec::<Action>::default()
                    },
                    damage: if let Some(x) = self.damage {
                        Some(x.parse()?)
                    } else {
                        None
                    },
                    damage_desc: self
                        .damage_desc
                        .map_or(String::default(), Cow::into_owned),
                    destiny: self.destiny,
                }),
                _ => Err(Error::SerDeVer(self.serdever, CURRENT, AGE)),
            }
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::Card> for Card<'a> {
        fn from(src: &'a super::Card) -> Self {
            Self {
                serdever: CURRENT,
                name: if src.name.is_empty() {
                    None
                } else {
                    Some(From::from(src.name.as_str()))
                },
                card_set: Some(From::from(src.card_set.as_str())),
                color: if let Some(ref x) = src.color {
                    Some(Cow::Borrowed(x.as_str()))
                } else {
                    None
                },
                constellation: if let Some(ref x) = src.constellation {
                    Some(Cow::Borrowed(x.as_str()))
                } else {
                    None
                },
                value: src.value,
                desc: if src.desc.is_empty() {
                    None
                } else {
                    Some(From::from(src.desc.as_str()))
                },
                story: if src.story.is_empty() {
                    None
                } else {
                    let mut ret = Vec::default();
                    for i in &src.story {
                        ret.push(From::from(i.as_str()));
                    }
                    Some(ret)
                },
                story_desc: if src.story_desc.is_empty() {
                    None
                } else {
                    Some(From::from(src.story_desc.as_str()))
                },
                action: if src.action.is_empty() {
                    None
                } else {
                    let mut ret = Vec::default();
                    for x in &src.action {
                        ret.push(From::from(x.as_str()));
                    }
                    Some(ret)
                },
                damage: if let Some(ref x) = src.damage {
                    Some(Cow::Borrowed(x.as_str()))
                } else {
                    None
                },
                damage_desc: if src.damage_desc.is_empty() {
                    None
                } else {
                    Some(From::from(src.damage_desc.as_str()))
                },
                destiny: src.destiny,
            }
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type CardMap
pub(crate) type CardMap = BTreeMap<String, Card>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const PATH_CARDS: &[&str] = &["cards/basic.", "cards/crimson."];
const EXTS_CARDS: &str = "toml";
const EXTS_NEW_CARDS: &str = "new.toml";
// ============================================================================
/// import_cards
pub(crate) fn import_cards<'a, S>(
    cards: &'a mut CardMap,
    root: &S,
) -> Result<&'a CardMap>
where
    S: Debug + AsRef<OsStr> + ?Sized,
{
    for path in PATH_CARDS {
        let mut p = Path::new(root).to_path_buf();
        p.push(path);
        let _ = p.set_extension(EXTS_CARDS);
        info!("shinen::card::import: {:?}", p);
        let mut cardset = ::toml::from_str::<CardMap>(
            File::open(p)
                .and_then(|mut f| {
                    use std::io::Read;
                    let mut input = String::new();
                    let _ = f.read_to_string(&mut input)?;
                    Ok(input)
                })?
                .as_str(),
        )?;
        if cfg!(debug_assertions) {
            // export
            let mut destiny_map = BTreeMap::<i32, Card>::default();
            for v in cardset.values() {
                let _ = destiny_map.insert(
                    v.as_destiny().unwrap_or_else(|| ::std::i32::MAX),
                    v.clone(),
                );
            }
            let mut p = Path::new(root).to_path_buf();
            p.push(path);
            let _ = p.set_extension(EXTS_NEW_CARDS);
            info!("shinen::card::export: {:?}", p);
            File::create(p).and_then(|mut f| {
                use std::io::{
                    Error as IOError, ErrorKind as IOErrorKind, Write,
                };
                for v in destiny_map.values() {
                    let _ = f
                        .write(format!("[\"{}\"]\n", v.as_name()).as_bytes())?;
                    let _ = f.write(
                        ::toml::to_string(&v)
                            .map_err(|e| {
                                IOError::new(
                                    IOErrorKind::Other,
                                    format!("toml::to_string: {:?}", e),
                                )
                            })?
                            .as_bytes(),
                    )?;
                    let _ = f.write(b"\n")?;
                }
                Ok(())
            })?;
        }
        cards.append(&mut cardset);
    }
    Ok(cards)
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Deck
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Deck(VecDeque<String>);
// ============================================================================
impl From<Vec<String>> for Deck {
    fn from(v: Vec<String>) -> Self {
        Deck(VecDeque::<String>::from(v))
    }
}
// ============================================================================
impl FromIterator<String> for Deck {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        Deck(VecDeque::<String>::from_iter(iter))
    }
}
// ============================================================================
impl Index<usize> for Deck {
    type Output = String;
    fn index(&self, idx: usize) -> &Self::Output {
        self.0.index(idx)
    }
}
// ----------------------------------------------------------------------------
impl IndexMut<usize> for Deck {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.0.index_mut(idx)
    }
}
// ============================================================================
impl Deck {
    // ========================================================================
    /// to_string
    pub(crate) fn to_string(&self, cards: &CardMap) -> Option<String> {
        let mut ret = format!("{} cards", self.0.len());
        for (i, v) in self.0.iter().enumerate() {
            if let Some(x) = cards.get(v) {
                ret += format!("\n========\n{:>4}. {}", i, x.pretty()).as_str()
            } else {
                return None;
            }
        }
        Some(ret)
    }
    // ========================================================================
    /// shuffle
    pub(crate) fn shuffle(&mut self) {
        let mut x = self.0.iter().cloned().collect::<Vec<String>>();
        use rand::{thread_rng, Rng};
        thread_rng().shuffle(&mut x[..]);
        self.0 = VecDeque::<String>::from(x);
    }
    // ========================================================================
    /// find
    pub(crate) fn find(&mut self, v: &[u8]) -> Option<(usize, &String)> {
        self.0.iter().enumerate().find(|&(_, x)| x.as_bytes() == v)
    }
    // ========================================================================
    /// pick
    pub(crate) fn pick(&mut self, v: &[u8]) -> Option<String> {
        if let Some(i) = if let Some((i, _x)) = self.find(v) {
            Some(i)
        } else {
            None
        } {
            self.remove(i)
        } else {
            None
        }
    }
    // VecDeque  //////////////////////////////////////////////////////////////
    // ========================================================================
    /// fn is_empty
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    // ========================================================================
    /// fn len
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
    // ========================================================================
    /// fn get
    pub(crate) fn get(&self, n: usize) -> Option<&String> {
        self.0.get(n)
    }
    // ------------------------------------------------------------------------
    /// fn get_mut
    pub(crate) fn get_mut(&mut self, n: usize) -> Option<&mut String> {
        self.0.get_mut(n)
    }
    // ========================================================================
    /// fn pop_front
    pub(crate) fn pop_front(&mut self) -> Option<String> {
        self.0.pop_front()
    }
    // ------------------------------------------------------------------------
    /// fn pop_back
    pub(crate) fn pop_back(&mut self) -> Option<String> {
        self.0.pop_back()
    }
    // ========================================================================
    /// fn push_front
    pub(crate) fn push_front(&mut self, v: String) {
        self.0.push_front(v)
    }
    // ------------------------------------------------------------------------
    /// fn push_back
    pub(crate) fn push_back(&mut self, v: String) {
        self.0.push_back(v)
    }
    // ========================================================================
    /// fn remove
    pub(crate) fn remove(&mut self, n: usize) -> Option<String> {
        self.0.remove(n)
    }
    // ========================================================================
    /// fn append
    pub(crate) fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }
    // ========================================================================
    /// fn iter
    pub(crate) fn iter(&self) -> vec_deque::Iter<String> {
        self.0.iter()
    }
    // ------------------------------------------------------------------------
    /// fn iter_mut
    pub(crate) fn iter_mut(&mut self) -> vec_deque::IterMut<String> {
        self.0.iter_mut()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Hand
pub(crate) type Hand = Deck;
