// -*- mode:rust; coding:utf-8-unix; -*-

//! aizuna.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/28
//  @date 2018/05/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine-fringe")]
use std::cell::RefCell;
#[cfg(feature = "coroutine-fringe")]
use std::collections::VecDeque;
// ----------------------------------------------------------------------------
use rusty_leveldb::{CompressionType, Options, WriteBatch, DB};
// ----------------------------------------------------------------------------
use super::connector::{Connector, ResRec};
use super::rule::RuleImpl;
use super::{
    Behavior, Command, Config, Dice, Error, Message, MessageAelicit, Responce,
    Result,
};
