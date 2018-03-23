// -*- mode:rust; coding:utf-8-unix; -*-

//! action.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/18
//  @date 2018/02/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::str::FromStr;
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Attack
    Attack,
    /// Defence
    Defence,
    /// Avoid
    Avoid,
    /// Move
    Move,
    /// Magic
    Magic,
    /// Vision
    Vision,
    /// Despair
    Despair,
    /// Madness
    Madness,
    /// Free
    Free,
    /// Failure
    Failure,
    /// Custum
    Custum(String),
}
// ============================================================================
impl Action {
    // ========================================================================
    /// as_str
    pub fn as_str(&self) -> &str {
        match *self {
            Action::Attack => "攻撃",
            Action::Defence => "防御",
            Action::Avoid => "回避",
            Action::Move => "移動",
            Action::Magic => "魔法",
            Action::Vision => "幻視",
            Action::Despair => "絶望",
            Action::Madness => "狂気",
            Action::Free => "自由",
            Action::Failure => "大失敗",
            Action::Custum(ref src) => src.as_str(),
        }
    }
}
// ============================================================================
impl FromStr for Action {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "攻撃" => Ok(Action::Attack),
            "防御" => Ok(Action::Defence),
            "回避" => Ok(Action::Avoid),
            "移動" => Ok(Action::Move),
            "魔法" => Ok(Action::Magic),
            "幻視" => Ok(Action::Vision),
            "絶望" => Ok(Action::Despair),
            "狂気" => Ok(Action::Madness),
            "自由" => Ok(Action::Free),
            "大失敗" => Ok(Action::Failure),
            x => Ok(Action::Custum(String::from(x))),
        }
    }
}
