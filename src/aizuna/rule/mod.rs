// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/25
//  @date 2018/01/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub use super::{Behavior, Command, Error, Message, Result, SessionKind};
pub use self::config::Config;
// mod  =======================================================================
pub mod config;
pub mod shinen;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Rule
pub trait Rule: ::std::fmt::Debug {
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
