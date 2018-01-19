// -*- mode:rust; coding:utf-8-unix; -*-

//! aizuna.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/06
//  @date 2018/01/19

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
extern crate aizuna;
extern crate env_logger;
extern crate getopts;
extern crate log;
// use  =======================================================================
use std::path::PathBuf;
use std::env;
// ----------------------------------------------------------------------------
use aizuna::Config;
// ----------------------------------------------------------------------------
use self::error::Result;
// mod  =======================================================================
mod error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const AIZUNA_VERSTR: &'static str =
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
    let path_root = matches.opt_str("R").map(PathBuf::from).unwrap_or({
        let mut ret = PathBuf::from(env::home_dir()?);
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
