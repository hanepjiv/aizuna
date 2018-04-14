// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/28
//  @date 2018/04/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
#[cfg(feature = "coroutine")]
pub(crate) use super::Error;
pub(crate) use super::Result;
// ----------------------------------------------------------------------------
pub(crate) use self::console::Console;
// mod  =======================================================================
mod console;
#[cfg(feature = "coroutine")]
mod receiver;
