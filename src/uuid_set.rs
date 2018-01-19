// -*- mode:rust; coding:utf-8-unix; -*-

//! uuidset.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/12
//  @date 2018/01/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeSet;
use uuid::Uuid;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Iter
pub type Iter<'a> = ::std::collections::btree_set::Iter<'a, Uuid>;
// ============================================================================
/// type UuidSet
pub type UuidSet = BTreeSet<Uuid>;
