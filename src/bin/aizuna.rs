// -*- mode:rust; coding:utf-8-unix; -*-

//! aizuna.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/06
//  @date 2018/04/14

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![feature(try_trait)]
#![deny(
    anonymous_parameters, bare_trait_object, missing_copy_implementations,
    missing_debug_implementations, missing_docs, unused_extern_crates,
    unused_import_braces, unused_qualifications, unused_results,
    variant_size_differences, const_err, deprecated,
    illegal_floating_point_literal_pattern, improper_ctypes,
    incoherent_fundamental_impls, late_bound_lifetime_arguments,
    non_camel_case_types, non_shorthand_field_patterns, non_snake_case,
    non_upper_case_globals, no_mangle_generic_items, overflowing_literals,
    path_statements, patterns_in_fns_without_body, plugin_as_library,
    private_in_public, private_no_mangle_fns, private_no_mangle_statics,
    renamed_and_removed_lints, safe_packed_borrows, stable_features,
    type_alias_bounds, tyvar_behind_raw_pointer, unconditional_recursion,
    unions_with_drop_fields, unknown_lints, unreachable_code,
    unreachable_patterns, unstable_name_collision, unused_allocation,
    unused_assignments, unused_attributes, unused_comparisons,
    unused_doc_comment, unused_features, unused_imports, unused_macros,
    unused_must_use, unused_mut, unused_parens, unused_unsafe,
    unused_variables, while_true, exceeding_bitshifts,
    invalid_type_param_default, legacy_constructor_visibility,
    legacy_directory_ownership, legacy_imports, missing_fragment_specifier,
    mutable_transmutes, no_mangle_const_items,
    parenthesized_params_in_types_and_modules, pub_use_of_private_extern_crate,
    safe_extern_statics, unknown_crate_types
)]
#![warn(dead_code, unreachable_pub, unstable_features)]
#![allow(
    box_pointers, elided_lifetime_in_path, single_use_lifetime, trivial_casts,
    trivial_numeric_casts, unsafe_code
)]
// extern  ====================================================================
extern crate aizuna;
extern crate env_logger;
extern crate getopts;
extern crate log;
// use  =======================================================================
use std::env;
use std::path::PathBuf;
// ----------------------------------------------------------------------------
use aizuna::Config;
// ----------------------------------------------------------------------------
use self::error::Result;
// mod  =======================================================================
mod error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const AIZUNA_VERSTR: &str =
    concat!(module_path!(), " v", env!("CARGO_PKG_VERSION"));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn print_usage(opts: &::getopts::Options) {
    print!(
        "{}{}",
        AIZUNA_VERSTR,
        opts.usage(concat!(
            r##"

Usage:
    "##,
            module_path!(),
            r##" [Options]"##
        ))
    );
}
// ============================================================================
fn app() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = ::getopts::Options::new();
    let _ = opts.optflag("v", "version", "print version")
        .optflag("h", "help", "print this help menu")
        .optopt(
            "R",
            "root",
            "set Aizuna root path. default '${HOME}/.config/aizuna'",
            "PATH",
        );
    let matches = opts.parse(&args[1..])?;
    if matches.opt_present("v") {
        return Ok(println!("{}", AIZUNA_VERSTR));
    }
    if matches.opt_present("h") {
        return Ok(print_usage(&opts));
    }
    let path_root = matches
        .opt_str("R")
        .map(PathBuf::from)
        .unwrap_or({
            let mut ret = env::home_dir()?;
            ret.push(".config");
            ret.push("aizuna");
            ret
        });
    Ok(match Config::new(path_root) {
        Err(aizuna::Error::NoConfig) => (),
        x @ Err(_) => x.map(|_| ())?,
        Ok(x) => x.aizuna()?.drive()?,
    })
}
// ============================================================================
fn main() {
    env_logger::init();
    if let Err(x) = app() {
        eprintln!("{}", x);
    }
}
