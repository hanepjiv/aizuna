// -*- mode:rust; coding:utf-8-unix; -*-

//! ask.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/29
//  @date 2018/05/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::io::{stdin, stdout, Write};
// ----------------------------------------------------------------------------
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) fn ask(msg: &str, default: bool) -> Result<bool> {
    stdout().write_all(msg.as_ref())?;
    stdout().write_all(if default { b" [Y/n]: " } else { b" [y/N]: " })?;
    stdout().flush()?;
    let mut line = String::new();
    let _ = stdin().read_line(&mut line)?;
    match line.trim().to_lowercase().as_str() {
        "" => Ok(default),
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => ask(msg, default),
    }
}
