// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/27
//  @date 2018/01/07

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub use super::{Error, Result};
pub use self::config::Config;
pub use self::discord::Discord;
pub use self::message::DiscordMessage;
// mod  =======================================================================
pub mod config;
pub mod discord;
pub mod message;
#[cfg(feature = "coroutine")]
pub mod receiver;
