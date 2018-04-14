// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2018/04/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
// ----------------------------------------------------------------------------
pub(crate) use super::{Command, Error, Responce, Result};
// ----------------------------------------------------------------------------
pub(crate) use self::config::Config;
pub(crate) use self::console::Console;
pub(crate) use self::discord::Discord;
#[cfg(feature = "coroutine")]
use self::receiver::{ReceiverImpl, Recv};
// mod  =======================================================================
mod config;
mod console;
mod discord;
#[cfg(feature = "coroutine")]
mod receiver;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
// Fringe
// ============================================================================
#[cfg(feature = "coroutine-fringe")]
/// type Generator
pub(crate) type Generator =
    ::fringe::generator::Generator<Command, Responce, ::fringe::OsStack>;
// ============================================================================
#[cfg(feature = "coroutine-fringe")]
/// Yielder
pub(crate) type Yielder = ::fringe::generator::Yielder<Command, Responce>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
// Thread
// ============================================================================
/// type CmdSen
pub(crate) type CmdSen = Sender<Command>;
// ----------------------------------------------------------------------------
/*
/// type CmdRec
pub(crate) type CmdRec = Receiver<Command>;
*/
// ============================================================================
/// type ResSen
pub(crate) type ResSen = Sender<(Responce, Option<CmdSen>)>;
// ----------------------------------------------------------------------------
/// type ResRec
pub(crate) type ResRec = Receiver<(Responce, Option<CmdSen>)>;
// ============================================================================
/// trait Connector
pub trait Connector: Debug {
    // ========================================================================
    #[cfg(feature = "coroutine-fringe")]
    /// fn gen
    fn gen(&self, stack: ::fringe::OsStack) -> Result<Generator>;
    // ========================================================================
    /// fn spawn
    fn spawn(&self, res_sen: ResSen) -> Result<JoinHandle<Result<()>>>;
}
