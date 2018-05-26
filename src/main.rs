// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/06
//  @date 2018/05/27

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![feature(try_trait)]
// rustc 1.28.0-nightly (990d8aa74 2018-05-25)  -------------------------------
#![deny(
    absolute_paths_not_starting_with_crate, anonymous_parameters,
    bare_trait_objects, box_pointers, missing_copy_implementations,
    missing_debug_implementations, missing_docs, trivial_casts,
    trivial_numeric_casts, unnecessary_extern_crates, unreachable_pub,
    unsafe_code, unused_extern_crates, unused_import_braces, unused_labels,
    unused_lifetimes, unused_qualifications, unused_results,
    variant_size_differences, dead_code, deprecated,
    duplicate_associated_type_bindings, illegal_floating_point_literal_pattern,
    improper_ctypes, late_bound_lifetime_arguments, non_camel_case_types,
    non_shorthand_field_patterns, non_snake_case, non_upper_case_globals,
    no_mangle_generic_items, overflowing_literals, path_statements,
    patterns_in_fns_without_body, plugin_as_library, private_in_public,
    private_no_mangle_fns, private_no_mangle_statics, safe_packed_borrows,
    stable_features, trivial_bounds, type_alias_bounds,
    tyvar_behind_raw_pointer, unconditional_recursion, unions_with_drop_fields,
    unknown_lints, unreachable_code, unreachable_patterns,
    unstable_name_collisions, unused_allocation, unused_assignments,
    unused_attributes, unused_comparisons, unused_doc_comments,
    unused_features, unused_imports, unused_macros, unused_must_use,
    unused_mut, unused_parens, unused_unsafe, unused_variables, while_true,
    const_err, exceeding_bitshifts, incoherent_fundamental_impls,
    invalid_type_param_default, legacy_constructor_visibility,
    legacy_directory_ownership, missing_fragment_specifier, mutable_transmutes,
    no_mangle_const_items, parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate, safe_extern_statics, unknown_crate_types
)]
#![warn(
    dead_code, unreachable_pub, renamed_and_removed_lints, unstable_features
)]
#![allow(
    box_pointers, elided_lifetimes_in_paths, single_use_lifetimes,
    trivial_casts, trivial_numeric_casts, unsafe_code
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
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const AIZUNA_VERSTR: &str =
    concat!(module_path!(), " v", env!("CARGO_PKG_VERSION"));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(variant_size_differences)]
#[derive(Debug)]
pub(crate) enum Error {
    /// OptionNone
    OptionNone,
    /// SetLogger
    SetLogger(::log::SetLoggerError),
    /// GetOpts
    GetOpts(::getopts::Fail),
    /// Aizuna
    Aizuna(::aizuna::Error),
}
// ============================================================================
impl From<::std::option::NoneError> for Error {
    fn from(_: ::std::option::NoneError) -> Self {
        Error::OptionNone
    }
}
// ----------------------------------------------------------------------------
impl From<::log::SetLoggerError> for Error {
    fn from(e: ::log::SetLoggerError) -> Self {
        Error::SetLogger(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::getopts::Fail> for Error {
    fn from(e: ::getopts::Fail) -> Self {
        Error::GetOpts(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::aizuna::Error> for Error {
    fn from(e: ::aizuna::Error) -> Self {
        Error::Aizuna(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptionNone => "::std::option::NoneError",
            Error::SetLogger(ref e) => e.description(),
            Error::GetOpts(ref e) => e.description(),
            Error::Aizuna(ref e) => e.description(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::OptionNone => None,
            Error::SetLogger(ref e) => Some(e),
            Error::GetOpts(ref e) => Some(e),
            Error::Aizuna(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
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
fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let mut opts = ::getopts::Options::new();
    let _ = opts
        .optflag("v", "version", "print version")
        .optflag("h", "help", "print this help menu")
        .optopt(
            "R",
            "root",
            "set Aizuna root path. default '${HOME}/.config/aizuna'",
            "PATH",
        );
    let matches = opts.parse(&args[1..])?;
    if matches.opt_present("v") {
        println!("{}", AIZUNA_VERSTR);
        return Ok(());
    }
    if matches.opt_present("h") {
        print_usage(&opts);
        return Ok(());
    }
    let path_root = matches.opt_str("R").map(PathBuf::from).unwrap_or({
        let mut ret = env::home_dir()?;
        ret.push(".config");
        ret.push("aizuna");
        ret
    });
    match Config::new(path_root) {
        Err(aizuna::Error::NoConfig) => (),
        x @ Err(_) => x.map(|_| ())?,
        Ok(x) => x.aizuna()?.drive()?,
    }
    Ok(())
}
