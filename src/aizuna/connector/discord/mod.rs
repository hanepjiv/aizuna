// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/27
//  @date 2018/04/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub(crate) use self::config::Config;
pub(crate) use self::discord::Discord;
pub(crate) use self::message::DiscordMessage;
pub(crate) use super::{Error, Result};
// mod  =======================================================================
mod config;
mod discord;
mod message;
#[cfg(feature = "coroutine")]
mod receiver;
