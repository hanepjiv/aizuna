// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2019/04/30

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![feature(try_trait)]
// ----------------------------------------------------------------------------
// rustc 1.36.0-nightly (00859e3e6 2019-04-29)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    question_mark_macro_sep,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    ambiguous_associated_items,
    deprecated,
    exported_private_dependencies,
    illegal_floating_point_literal_pattern,
    ill_formed_attribute_input,
    improper_ctypes,
    intra_doc_link_resolution_failure,
    irrefutable_let_patterns,
    late_bound_lifetime_arguments,
    mutable_borrow_reservation_conflict,
    nested_impl_trait,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unnameable_test_items,
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
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types
)]
#![warn(
    dead_code,
    renamed_and_removed_lints,
    unreachable_pub,
    unstable_features
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
