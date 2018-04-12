// -*- mode:rust; coding:utf-8-unix; -*-

//! fmt_indent.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/12
//  @date 2018/04/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter, Result};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait FormatIndent
pub trait FormatIndent {
    // ========================================================================
    /// fn fmt_idt
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result;
}
// ============================================================================
impl FormatIndent for Uuid {
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result {
        write!(f, "{e:>idt0$}{}", self, e = "", idt0 = idt)
    }
}
// ============================================================================
impl<T> FormatIndent for BTreeSet<T>
where
    T: FormatIndent,
{
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result {
        if self.is_empty() {
            return write!(f, "{e:>idt0$}[]", e = "", idt0 = idt);
        }
        let _ = write!(f, "{e:>idt0$}[", e = "", idt0 = idt)?;
        for i in self {
            let _ = f.write_str("\n")?;
            let _ = i.fmt_idt(f, idt + 2usize)?;
            let _ = f.write_str(",")?;
        }
        write!(f, "\n{e:>idt0$}]", e = "", idt0 = idt)
    }
}
// ============================================================================
impl<K, V> FormatIndent for BTreeMap<K, V>
where
    K: Debug + FormatIndent,
    V: Debug + FormatIndent,
{
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result {
        if self.is_empty() {
            return write!(f, "{e:>idt0$}{{}}", e = "", idt0 = idt);
        }
        let _ = write!(f, "{e:>idt0$}{{", e = "", idt0 = idt)?;
        for (k, v) in self.iter() {
            let _ = write!(
                f,
                "\n{e:>idt1$}(\n",
                e = "",
                idt1 = idt + 2usize,
            )?;
            let _ = k.fmt_idt(f, idt + 4usize)?;
            let _ = f.write_str(",\n")?;
            let _ = v.fmt_idt(f, idt + 4usize)?;
            let _ = write!(
                f,
                ",\n{e:>idt1$}),",
                e = "",
                idt1 = idt + 2usize,
            )?;
        }
        write!(f, "\n{e:>idt0$}}}", e = "", idt0 = idt)
    }
}
