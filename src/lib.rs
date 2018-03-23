// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2018/03/09

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![feature(try_trait)]
#![deny(anonymous_parameters, box_pointers, missing_copy_implementations,
        missing_debug_implementations, missing_docs, trivial_casts,
        trivial_numeric_casts, unsafe_code, unused_extern_crates,
        unused_import_braces, unused_qualifications, unused_results,
        variant_size_differences, const_err, dead_code, deprecated,
        illegal_floating_point_literal_pattern, improper_ctypes,
        late_bound_lifetime_arguments, non_camel_case_types,
        non_shorthand_field_patterns, non_snake_case, non_upper_case_globals,
        no_mangle_generic_items, overflowing_literals, path_statements,
        patterns_in_fns_without_body, plugin_as_library, private_in_public,
        private_no_mangle_fns, private_no_mangle_statics,
        renamed_and_removed_lints, stable_features, unconditional_recursion,
        unions_with_drop_fields, unknown_lints, unreachable_code,
        unreachable_patterns, unused_allocation, unused_assignments,
        unused_attributes, unused_comparisons, unused_doc_comment,
        unused_features, unused_imports, unused_macros, unused_must_use,
        unused_mut, unused_parens, unused_unsafe, unused_variables,
        while_true)]
#![warn(dead_code, unstable_features)]
#![allow(box_pointers, unsafe_code, trivial_casts, trivial_numeric_casts)]
// extern  ====================================================================
#[macro_use]
extern crate bitflags;
extern crate chrono;
extern crate discord;
#[cfg(feature = "coroutine-fringe")]
extern crate fringe;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate rand;
extern crate regex;
extern crate rusty_leveldb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate uuid;
// ----------------------------------------------------------------------------
#[macro_use]
extern crate elicit;
// use  =======================================================================
pub use self::aizuna::{Aizuna, Config};
pub use self::error::{Error, Result};
pub use self::format_indent::FormatIndent;
// mod  =======================================================================
mod ask;
pub mod aizuna;
pub mod error;
mod format_indent;
pub mod uuid_set;
