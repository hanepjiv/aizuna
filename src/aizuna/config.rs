// -*- mode:rust; coding:utf-8-unix; -*-

//! config.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/28
//  @date 2018/04/28

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
// ----------------------------------------------------------------------------
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// ----------------------------------------------------------------------------
use super::super::ask::ask;
use super::connector::{Config as ConnectorConfig, Connector, Console, Discord};
use super::rule::shinen::ShinEn;
use super::rule::{Config as RuleConfig, RuleImpl};
use super::{Aizuna, Driver, Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const CONFIG_FILE: &str = "config.toml";
const CONFIG_DEFAULT: &str = concat!(
    r##"# -*- mode:toml; coding:utf-8-unix; -*-
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
# "##,
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION"),
    r##"
serdever                = 0
greeting                = ""##,
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION"),
    r##""
driver                  = "Thread"
fringe_stack_size       = 1048576
path_db                 = "./db"
prefix                  = ","
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
[connectors.console]
serdever                = 0
enable                  = true
connector               = "console"
# =============================================================================
[connectors.discord-00]
serdever                = 0
enable                  = false
connector               = "discord"
[connectors.discord-00.config]
serdever                = 0
token                   = "DISCORD_BOT_TOKEN"
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
[rules.shinen]
serdever                = 0
enable                  = false
prefix                  = ","
[rules.shinen.config]
serdever                = 0
root                    = "SHINEN_ROOT_PATH"
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
[admin]
console                 = [".*"]
discord-00              = ["DISCORD_USER_ID_00", "DISCORD_USER_ID_01"]
"##
);
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Config
#[derive(Debug, Clone)]
pub struct Config {
    /// path_root
    path_root: PathBuf,
    /// path_config
    path_config: PathBuf,
    /// greeting
    greeting: String,
    /// driver
    driver: Driver,
    /// fringe_stack_size
    fringe_stack_size: usize,
    /// path_db
    path_db: PathBuf,
    /// prefix
    prefix: String,
    /// connectors
    connectors: BTreeMap<String, ConnectorConfig>,
    /// rules
    rules: BTreeMap<String, RuleConfig>,
    /// admin
    admin: BTreeMap<String, Vec<Regex>>,
}
// ============================================================================
impl Config {
    // ========================================================================
    /// fn as_greeting
    pub(crate) fn as_greeting(&self) -> &str {
        &self.greeting
    }
    // ========================================================================
    /// fn as_driver
    pub(crate) fn as_driver(&self) -> &Driver {
        &self.driver
    }
    // ========================================================================
    /// fn as_fringe_stack_size
    pub(crate) fn as_fringe_stack_size(&self) -> usize {
        self.fringe_stack_size
    }
    // ========================================================================
    /// fn as_prefix
    pub(crate) fn as_prefix(&self) -> &str {
        self.prefix.as_str()
    }
    // ========================================================================
    /// fn as_path_db
    pub(crate) fn as_path_db(&self) -> &Path {
        &self.path_db
    }
    // ========================================================================
    /// fn as_admin
    pub(crate) fn as_admin(&self) -> &BTreeMap<String, Vec<Regex>> {
        &self.admin
    }
    // ========================================================================
    /// new
    pub fn new<P>(path_root: P) -> Result<Config>
    where
        P: AsRef<Path>,
    {
        info!("Config::new");
        let mut path_root = PathBuf::from(path_root.as_ref());
        path_root = if path_root.is_absolute() {
            path_root
        } else {
            let mut current_dir = ::std::env::current_dir()?;
            current_dir.push(path_root);
            current_dir
        };
        let mut path_config = path_root.clone();
        path_config.push(CONFIG_FILE);
        if !path_config.exists() {
            if ask(
                format!("create config file? {:?}", path_config).as_str(),
                true,
            )? {
                ::std::fs::create_dir_all(path_root.as_path())?;
                let _ = File::create(path_config.as_path()).and_then(
                    |mut f| Ok(f.write(CONFIG_DEFAULT.as_bytes())?),
                )?;
            } else {
                return Err(Error::NoConfig);
            }
        }
        path_root = path_root.canonicalize()?;
        path_config = path_config.canonicalize()?;
        if !path_root.is_dir() {
            return Err(Error::InvalidArg(String::from(
                "input root path is not directory.",
            )));
        }
        let mut config = ::toml::from_str::<Config>(
            File::open(&path_config)
                .and_then(|mut f| {
                    let mut input = String::new();
                    let _ = f.read_to_string(&mut input)?;
                    Ok(input)
                })?
                .as_str(),
        )?;
        if config.path_db.is_relative() {
            let mut path_db = path_root.clone();
            path_db.push(config.path_db);
            config.path_db = path_db;
        }
        Ok(Config {
            path_root,
            path_config,
            ..config
        })
    }
    // ========================================================================
    /// aizuna
    pub fn aizuna(self) -> Result<Aizuna> {
        info!(
            "::aizuna::Config::aizuna: {:?}",
            self.path_config
        );
        let mut connectors = Vec::<Box<dyn Connector>>::default();
        for (k0, v0) in &self.connectors {
            if !v0.enable {
                continue;
            }
            match v0.connector.as_str() {
                "console" => {
                    connectors.push(Box::new(Console::new(k0)));
                }
                "discord" => {
                    connectors
                        .push(Box::new(Discord::new(k0, v0.config.clone())?));
                }
                _ => {
                    return Err(Error::InvalidArg(format!(
                        "::aizuna::Config::aizuna: unknown connector: {}",
                        v0.connector
                    )));
                }
            }
        }
        let mut rules = BTreeMap::<String, RuleImpl>::default();
        for (k0, v0) in &self.rules {
            if !v0.enable {
                continue;
            }
            match k0.as_str() {
                "shinen" => {
                    let _ = rules.insert(
                        v0.prefix.clone(),
                        RuleImpl::ShinEn(ShinEn::new(
                            self.path_root.as_path(),
                            v0.config.clone(),
                        )?),
                    );
                }
                _ => {
                    return Err(Error::InvalidArg(format!(
                        "::aizuna::Config::aizuna: unknown rule: {}",
                        k0
                    )));
                }
            }
        }
        Aizuna::new(self, connectors, rules)
    }
}
// ============================================================================
impl Serialize for Config {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize::Config::from(self).serialize(serializer)
    }
}
// ============================================================================
impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::serialize::Config::deserialize(deserializer)?
            .into()
            .map_err(|e| ::serde::de::Error::custom(format!("{}", e)))
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// serialize
mod serialize {
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    use super::*;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    const CURRENT: i32 = 0i32;
    const AGE: i32 = 0i32;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct Config
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct Config<'a> {
        /// serdever
        serdever: i32,
        /// greeting
        greeting: Option<Cow<'a, str>>,
        /// driver
        driver: Option<Cow<'a, Driver>>,
        /// fringe_stack_size
        fringe_stack_size: Option<usize>,
        /// path_db
        path_db: Option<Cow<'a, str>>,
        /// prefix
        prefix: Option<Cow<'a, str>>,
        /// connectors
        connectors: Option<Cow<'a, BTreeMap<String, ConnectorConfig>>>,
        /// rules
        rules: Option<Cow<'a, BTreeMap<String, RuleConfig>>>,
        /// admin
        admin: Option<BTreeMap<String, Vec<Cow<'a, str>>>>,
    }
    // ========================================================================
    impl<'a> Config<'a> {
        // ====================================================================
        /// into
        pub(crate) fn into(self) -> Result<super::Config> {
            debug!("::aizuna::Config::serialize::into");
            if self.serdever < (CURRENT - AGE) || CURRENT < self.serdever {
                return Err(Error::SerDeVer(self.serdever, CURRENT, AGE));
            }
            match self.serdever {
                0 => Ok(super::Config {
                    path_root: PathBuf::default(),
                    path_config: PathBuf::default(),
                    greeting: self.greeting.map_or(
                        String::from(concat!(
                            env!("CARGO_PKG_NAME"),
                            " v",
                            env!("CARGO_PKG_VERSION"),
                        )),
                        String::from,
                    ),
                    driver: self.driver
                        .map_or(Driver::Thread, Cow::into_owned),
                    fringe_stack_size: self.fringe_stack_size
                        .unwrap_or(1024 * 1024),
                    path_db: self.path_db
                        .map(Cow::into_owned)
                        .map_or(PathBuf::from("./db"), PathBuf::from),
                    prefix: self.prefix
                        .map_or(String::from(","), String::from),
                    connectors: self.connectors
                        .map(Cow::into_owned)
                        .unwrap_or_default(),
                    rules: self.rules
                        .map(Cow::into_owned)
                        .unwrap_or_default(),
                    admin: if let Some(x) = self.admin {
                        let mut ret = BTreeMap::default();
                        for (k, v) in x {
                            let mut vs = Vec::default();
                            for i in v {
                                vs.push(Regex::new(&i.into_owned())?);
                            }
                            let _ = ret.insert(k, vs);
                        }
                        ret
                    } else {
                        BTreeMap::default()
                    },
                }),
                _ => Err(Error::SerDeVer(self.serdever, CURRENT, AGE)),
            }
        }
    }
    // ========================================================================
    impl<'a> From<&'a super::Config> for Config<'a> {
        fn from(src: &'a super::Config) -> Self {
            debug!("::aizuna::Config::serialize::from");
            Self {
                serdever: CURRENT,
                greeting: Some(From::from(src.greeting.as_str())),
                driver: Some(Cow::Borrowed(&src.driver)),
                fringe_stack_size: Some(src.fringe_stack_size),
                path_db: Some(src.path_db.as_os_str().to_string_lossy()),
                prefix: Some(From::from(src.prefix.as_str())),
                connectors: Some(Cow::Borrowed(&src.connectors)),
                rules: Some(Cow::Borrowed(&src.rules)),
                admin: if src.admin.is_empty() {
                    None
                } else {
                    let mut ret = BTreeMap::default();
                    for (k, v) in &src.admin {
                        let mut vs = Vec::default();
                        for i in v.iter() {
                            vs.push(From::from(i.as_str()));
                        }
                        let _ = ret.insert(k.clone(), vs);
                    }
                    Some(ret)
                },
            }
        }
    }
}
