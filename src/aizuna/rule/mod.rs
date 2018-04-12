// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/25
//  @date 2018/04/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub(crate) use self::config::Config;
pub(crate) use super::{Behavior, Command, Error, Result, SessionKind};
// mod  =======================================================================
pub(crate) mod config;
pub(crate) mod shinen;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Rule
pub(crate) trait Rule: ::std::fmt::Debug {
    // ========================================================================
    /// fn as_rule_name
    fn as_rule_name(&self) -> &str;
    // ========================================================================
    /// fn run
    fn run(&mut self, behavior: &mut Behavior) -> Result<Command>;
    // ========================================================================
    /// fn new_session_kind
    fn new_session_kind(&mut self) -> Result<SessionKind>;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum RuleImpl
#[derive(Debug)]
pub enum RuleImpl {
    /// ShinEn
    ShinEn(self::shinen::ShinEn),
}
// ============================================================================
impl Rule for RuleImpl {
    // ========================================================================
    fn as_rule_name(&self) -> &str {
        match *self {
            RuleImpl::ShinEn(ref x) => x.as_rule_name(),
        }
    }
    // ========================================================================
    fn run(&mut self, behavior: &mut Behavior) -> Result<Command> {
        match *self {
            RuleImpl::ShinEn(ref mut x) => x.run(behavior),
        }
    }
    // ========================================================================
    fn new_session_kind(&mut self) -> Result<SessionKind> {
        match *self {
            RuleImpl::ShinEn(ref mut x) => x.new_session_kind(),
        }
    }
}
