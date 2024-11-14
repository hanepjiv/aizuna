// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2024/11/14

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![feature(try_trait)]
// ----------------------------------------------------------------------------

// extern  ====================================================================

// use  =======================================================================
pub use self::{
    aizuna::{Aizuna, Config},
    error::{Error, Result},
    format_indent::FormatIndent,
};
// mod  =======================================================================
mod aizuna;
mod ask;
mod error;
mod format_indent;
mod uuid_set;
