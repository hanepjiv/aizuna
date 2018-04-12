// -*- mode:rust; coding:utf-8-unix; -*-

//! responce.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/10
//  @date 2018/04/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::{Error, MessageAelicit};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Responce
#[allow(variant_size_differences)]
#[derive(Debug)]
pub enum Responce {
    /// Error
    Error(Error),
    /// Yield
    Yield,
    /// Message
    Message(MessageAelicit),
}
