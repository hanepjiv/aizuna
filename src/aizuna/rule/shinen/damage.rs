// -*- mode:rust; coding:utf-8-unix; -*-

//! damage.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/19
//  @date 2018/03/03

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::str::FromStr;
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Damage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Damage {
    /// Impact
    Impact,
    /// Staging
    Staging,
    /// Arms
    Arms,
    /// Body
    Body,
    /// Legs
    Legs,
    /// Penetration
    Penetration,
    /// Armor
    Armor,
    /// Weapon
    Weapon,
    /// Head
    Head,
    /// Whole
    Whole,
    /// Custum
    Custum(String),
}
// ============================================================================
impl Damage {
    // ========================================================================
    /// as_str
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            Damage::Impact => "衝撃",
            Damage::Staging => "演出",
            Damage::Arms => "両腕",
            Damage::Body => "胴体",
            Damage::Legs => "脚部",
            Damage::Penetration => "貫通",
            Damage::Armor => "防具",
            Damage::Weapon => "武器",
            Damage::Head => "頭部",
            Damage::Whole => "全体",
            Damage::Custum(ref src) => src.as_str(),
        }
    }
}
// ============================================================================
impl FromStr for Damage {
    type Err = Error;
    // ========================================================================
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "衝撃" => Ok(Damage::Impact),
            "演出" => Ok(Damage::Staging),
            "両腕" => Ok(Damage::Arms),
            "胴体" => Ok(Damage::Body),
            "脚部" => Ok(Damage::Legs),
            "貫通" => Ok(Damage::Penetration),
            "防具" => Ok(Damage::Armor),
            "武器" => Ok(Damage::Weapon),
            "頭部" => Ok(Damage::Head),
            "全体" => Ok(Damage::Whole),
            x => Ok(Damage::Custum(String::from(x))),
        }
    }
}
