// -*- mode:rust; coding:utf-8-unix; -*-

//! story.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/19
//  @date 2018/02/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::str::FromStr;
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Story
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Story {
    /// Portent
    Portent,
    /// Origin
    Origin,
    /// Desire
    Desire,
    /// Assention
    Assention,
    /// Mythology
    Mythology,
    /// Words
    Words,
    /// Reminiscence
    Reminiscence,
    /// Wisdom
    Wisdom,
    /// Legend
    Legend,
    /// Declaration
    Declaration,
    /// Past
    Past,
    /// Temptation
    Temptation,
    /// Madness
    Madness,
    /// Destiny
    Destiny,
    /// Understanding
    Understanding,
    /// Nightmare
    Nightmare,
    /// Custum
    Custum(String),
}
// ============================================================================
impl Story {
    // ========================================================================
    /// as_str
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            Story::Portent => "前兆",
            Story::Origin => "出自",
            Story::Desire => "欲望",
            Story::Assention => "主張",
            Story::Mythology => "神話",
            Story::Words => "台詞",
            Story::Reminiscence => "追想",
            Story::Wisdom => "英知",
            Story::Legend => "伝説",
            Story::Declaration => "宣言",
            Story::Past => "過去",
            Story::Temptation => "誘惑",
            Story::Madness => "狂気",
            Story::Destiny => "運命",
            Story::Understanding => "理解",
            Story::Nightmare => "悪夢",
            Story::Custum(ref src) => src.as_str(),
        }
    }
}
// ============================================================================
impl FromStr for Story {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "前兆" => Ok(Story::Portent),
            "出自" => Ok(Story::Origin),
            "欲望" => Ok(Story::Desire),
            "主張" => Ok(Story::Assention),
            "神話" => Ok(Story::Mythology),
            "台詞" => Ok(Story::Words),
            "追想" => Ok(Story::Reminiscence),
            "英知" => Ok(Story::Wisdom),
            "伝説" => Ok(Story::Legend),
            "宣言" => Ok(Story::Declaration),
            "過去" => Ok(Story::Past),
            "誘惑" => Ok(Story::Temptation),
            "狂気" => Ok(Story::Madness),
            "運命" => Ok(Story::Destiny),
            "理解" => Ok(Story::Understanding),
            "悪夢" => Ok(Story::Nightmare),
            x => Ok(Story::Custum(String::from(x))),
        }
    }
}
