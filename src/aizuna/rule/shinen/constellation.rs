// -*- mode:rust; coding:utf-8-unix; -*-

//! constellation.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/14
//  @date 2018/02/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::str::FromStr;
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Constellation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Constellation {
    /// All
    All,
    /// Kokken
    Kokken,
    /// Yokujin
    Yokujin,
    /// Yubiwa
    Yubiwa,
    /// Sensya
    Sensya,
    /// Kayoibi
    Kayoibi,
    /// Noduti
    Noduti,
    /// Seiryuu
    Seiryuu,
    /// Genjya
    Genjya,
    /// Kaiou
    Kaiou,
    /// Makibito
    Makibito,
    /// Kokyou
    Kokyou,
    /// Fuuko
    Fuuko,
    /// Hatigenkin
    Hatigenkin,
    /// Custum
    Custum(String),
}
// ============================================================================
impl Constellation {
    // ========================================================================
    /// as_str
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            Constellation::All => "全て",
            Constellation::Kokken => "黒剣",
            Constellation::Yokujin => "翼人",
            Constellation::Yubiwa => "指輪",
            Constellation::Sensya => "戦車",
            Constellation::Kayoibi => "通火",
            Constellation::Noduti => "野槌",
            Constellation::Seiryuu => "青龍",
            Constellation::Genjya => "原蛇",
            Constellation::Kaiou => "海王",
            Constellation::Makibito => "牧人",
            Constellation::Kokyou => "古鏡",
            Constellation::Fuuko => "風虎",
            Constellation::Hatigenkin => "八弦琴",
            Constellation::Custum(ref src) => src.as_str(),
        }
    }
    // ========================================================================
    /// as_ruby
    pub(crate) fn as_ruby(&self) -> &'static str {
        match *self {
            Constellation::All => "",
            Constellation::Kokken => "ソダール",
            Constellation::Yokujin => "ティオール",
            Constellation::Yubiwa => "ギャルレイ",
            Constellation::Sensya => "サイベル",
            Constellation::Kayoibi => "ヒュオヌス",
            Constellation::Noduti => "アヌルフ",
            Constellation::Seiryuu => "ラーヴリュ",
            Constellation::Genjya => "エセス",
            Constellation::Kaiou => "アーエィス",
            Constellation::Makibito => "エンティン",
            Constellation::Kokyou => "テルティス",
            Constellation::Fuuko => "ヴァーリン",
            Constellation::Hatigenkin => "パーパイル",
            Constellation::Custum(_) => "",
        }
    }
}
// ============================================================================
impl FromStr for Constellation {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "全て" => Ok(Constellation::All),
            "黒剣" => Ok(Constellation::Kokken),
            "翼人" => Ok(Constellation::Yokujin),
            "指輪" => Ok(Constellation::Yubiwa),
            "戦車" => Ok(Constellation::Sensya),
            "通火" => Ok(Constellation::Kayoibi),
            "野槌" => Ok(Constellation::Noduti),
            "青龍" => Ok(Constellation::Seiryuu),
            "原蛇" => Ok(Constellation::Genjya),
            "海王" => Ok(Constellation::Kaiou),
            "牧人" => Ok(Constellation::Makibito),
            "古鏡" => Ok(Constellation::Kokyou),
            "風虎" => Ok(Constellation::Fuuko),
            "八弦琴" => Ok(Constellation::Hatigenkin),
            x => Ok(Constellation::Custum(String::from(x))),
        }
    }
}
