// -*- mode:rust; coding:utf-8-unix; -*-

//! command.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/10
//  @date 2018/01/15

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeSet;
// ----------------------------------------------------------------------------
use super::MessageAelicit;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Command
#[allow(variant_size_differences)]
#[derive(Debug)]
pub enum Command {
    /// Quit
    Quit(Option<MessageAelicit>),
    /// Listen
    Listen,
    /// Send
    Send(MessageAelicit, String),
    /// Whisper
    Whisper(BTreeSet<String>, String),
    /// SendWhisperMine
    SendWhisperMine(
        (MessageAelicit, String),
        (BTreeSet<String>, String),
        (String, String),
    ),
}
