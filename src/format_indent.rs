// -*- mode:rust; coding:utf-8-unix; -*-

//! fmt_indent.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/12
//  @date 2018/05/27

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
    /// fn make_idt
    fn make_idt(idt: usize) -> String {
        [' '].iter().cycle().take(idt).collect()
    }
    // ========================================================================
    /// fn fmt_idt
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result;
}
// ============================================================================
impl FormatIndent for Uuid {
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result {
        let s0 = <Self as FormatIndent>::make_idt(idt);
        write!(f, "{s0}{}", self, s0 = s0)
    }
}
// ============================================================================
impl<T> FormatIndent for BTreeSet<T>
where
    T: FormatIndent,
{
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result {
        let s0 = <Self as FormatIndent>::make_idt(idt);
        if self.is_empty() {
            return write!(f, "{s0}[]", s0 = s0);
        }
        write!(f, "{s0}[", s0 = s0)?;
        for i in self {
            f.write_str("\n")?;
            i.fmt_idt(f, idt + 2usize)?;
            f.write_str(",")?;
        }
        write!(f, "\n{s0}]", s0 = s0)
    }
}
// ============================================================================
impl<K, V> FormatIndent for BTreeMap<K, V>
where
    K: Debug + FormatIndent,
    V: Debug + FormatIndent,
{
    fn fmt_idt(&self, f: &mut Formatter, idt: usize) -> Result {
        let s0 = <Self as FormatIndent>::make_idt(idt);

        if self.is_empty() {
            return write!(f, "{s0}{{}}", s0 = s0);
        }

        let s1 = <Self as FormatIndent>::make_idt(idt + 2usize);

        write!(f, "{s0}{{", s0 = s0)?;
        for (k, v) in self.iter() {
            writeln!(f, "\n{s1}(", s1 = s1)?;
            k.fmt_idt(f, idt + 4usize)?;
            f.write_str(",\n")?;
            v.fmt_idt(f, idt + 4usize)?;
            write!(f, ",\n{s1}),", s1 = s1)?;
        }
        write!(f, "\n{s0}}}", s0 = s0)
    }
}
