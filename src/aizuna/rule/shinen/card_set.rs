// -*- mode:rust; coding:utf-8-unix; -*-

//! card_set.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/19
//  @date 2017/12/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::str::FromStr;
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum CardSet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardSet {
    /// Basic
    Basic,
    /// Crimson
    Crimson,
    /// Custum
    Custum(String),
}
// ============================================================================
impl CardSet {
    // ========================================================================
    /// as_str
    pub fn as_str(&self) -> &str {
        match *self {
            CardSet::Basic => "基本",
            CardSet::Crimson => {
                "血のごとく赤き 〜夢魔の占い札〜"
            }
            CardSet::Custum(ref src) => src.as_str(),
        }
    }
}
// ============================================================================
impl FromStr for CardSet {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "基本" => Ok(CardSet::Basic),
            "血のごとく赤き 〜夢魔の占い札〜" => {
                Ok(CardSet::Crimson)
            }
            x @ _ => Ok(CardSet::Custum(String::from(x))),
        }
    }
}
