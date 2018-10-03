// -*- mode:rust; coding:utf-8-unix; -*-

//! dice.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/01
//  @date 2018/09/19

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Dice
#[derive(Debug)]
pub(crate) struct Dice {
    /// regex
    regex: Regex,
}
// ============================================================================
impl Dice {
    // ========================================================================
    /// fn roll
    pub(crate) fn roll(mut n: i32, m: i32) -> (i32, i32, Vec<i32>, i64) {
        let mut v = Vec::default();
        if n < 1 || m < 1 {
            return (n, m, v, 0);
        }
        if 99 < n {
            n = 99;
        }
        let mut ret = 0i64;
        let between = Uniform::from(1..(m + 1));
        let mut rng = ::rand::thread_rng();
        for i in 0..n {
            v.push(between.sample(&mut rng));
            ret += i64::from(v[i as usize]);
        }
        (n, m, v, ret)
    }
    // ========================================================================
    /// fn new
    pub(crate) fn new() -> Result<Self> {
        Ok(Dice {
            regex: Regex::new(r"^(\d*)d(\d*)$")?,
        })
    }
    // ========================================================================
    /// fn parse
    pub(crate) fn parse(
        &self,
        src: impl AsRef<str>,
    ) -> Result<(i32, i32, Vec<i32>, i64)> {
        let caps = self.regex.captures(src.as_ref()).ok_or_else(|| {
            Error::InvalidArg(format!("Dice::parse: {}: caps", src.as_ref()))
        })?;
        Ok(Dice::roll(
            caps.get(1)
                .map(|x| x.as_str().parse().unwrap_or(1))
                .unwrap_or(1),
            caps.get(2)
                .map(|x| x.as_str().parse().unwrap_or(6))
                .unwrap_or(6),
        ))
    }
}
