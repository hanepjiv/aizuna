// -*- mode:rust; coding:utf-8-unix; -*-

//! color.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/14
//  @date 2018/08/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::str::FromStr;
// ----------------------------------------------------------------------------
use serde_derive::{Deserialize, Serialize};
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Color
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Color {
    /// All
    All,
    /// White
    White,
    /// Black
    Black,
    /// Yellow
    Yellow,
    /// Red
    Red,
    /// Blue
    Blue,
    /// Green
    Green,
    /// Purple
    Purple,
    /// Custum
    Custum(String),
}
// ============================================================================
impl Color {
    // ========================================================================
    /// as_str
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            Color::All => "全て",
            Color::White => "白",
            Color::Black => "黒",
            Color::Yellow => "黄",
            Color::Red => "赤",
            Color::Blue => "青",
            Color::Green => "緑",
            Color::Purple => "紫",
            Color::Custum(ref src) => src.as_str(),
        }
    }
}
// ============================================================================
impl FromStr for Color {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "全て" => Ok(Color::All),
            "白" => Ok(Color::White),
            "黒" => Ok(Color::Black),
            "黄" => Ok(Color::Yellow),
            "赤" => Ok(Color::Red),
            "青" => Ok(Color::Blue),
            "緑" => Ok(Color::Green),
            "紫" => Ok(Color::Purple),
            x => Ok(Color::Custum(String::from(x))),
        }
    }
}
