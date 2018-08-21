// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2018/08/22

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![feature(try_trait)]
// ----------------------------------------------------------------------------
// rustc 1.30.0-nightly (33b923fd4 2018-08-18)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    async_idents,
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    question_mark_macro_sep,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    dead_code,
    deprecated,
    duplicate_associated_type_bindings,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    intra_doc_link_resolution_failure,
    late_bound_lifetime_arguments,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    proc_macro_derive_resolution_fallback,
    renamed_and_removed_lints,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unnameable_test_functions,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true,
    const_err,
    duplicate_macro_exports,
    exceeding_bitshifts,
    incoherent_fundamental_impls,
    invalid_type_param_default,
    irrefutable_let_patterns,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types,
)]
#![warn(
    dead_code,
    renamed_and_removed_lints,
    unreachable_pub,
    unstable_features,
)]
#![allow(
    box_pointers,
    elided_lifetimes_in_paths,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code
)]
// extern  ====================================================================
extern crate bitflags;
extern crate elicit;
extern crate log;
extern crate serde_derive;
// ----------------------------------------------------------------------------
extern crate chrono;
extern crate discord;
extern crate getopts;
extern crate rand;
extern crate regex;
extern crate rusty_leveldb;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate uuid;
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine_fringe")]
extern crate fringe;
// use  =======================================================================
pub use self::{
    aizuna::{Aizuna, Config},
    error::{Error, Result},
    format_indent::FormatIndent,
};
// mod  =======================================================================
mod aizuna;
mod ask;
mod error;
mod format_indent;
mod uuid_set;
