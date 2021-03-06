// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/16
//  @date 2018/08/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    collections::BTreeMap,
    iter::{FromIterator, IntoIterator},
    path::PathBuf,
    result::Result as StdResult,
    sync::mpsc::RecvTimeoutError,
    time::Duration,
};
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine_fringe")]
use std::{cell::RefCell, collections::VecDeque};
// ----------------------------------------------------------------------------
use log::{debug, info};
use rusty_leveldb::{CompressionType, Options, WriteBatch, DB};
use serde_derive::{Deserialize, Serialize};
// ----------------------------------------------------------------------------
pub(crate) use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::config::Config;
pub(crate) use self::{
    behavior::Behavior,
    command::Command,
    dice::Dice,
    message::{Message, MessageAelicit, MessageEAFS, MessageEAFSField},
    responce::Responce,
    session::{Session, SessionImpl},
    session_kind::SessionKind,
    user::User,
};
use self::{
    connector::{Connector, ResRec},
    rule::RuleImpl,
};
// mod  =======================================================================
mod behavior;
mod command;
mod config;
mod connector;
mod dice;

mod message;
mod responce;
mod rule;
mod session;
mod session_kind;
mod user;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Driver
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum Driver {
    /// Thread
    Thread,
    /// Fringe
    Fringe,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const AIZUNA_DB_VERSION_KEY: &[u8] = b"aizuna-db-version";
const AIZUNA_DB_CURRENT: i32 = 0i32;
const AIZUNA_DB_AGE: i32 = 0i32;
// ============================================================================
/// struct Aizuna
pub struct Aizuna {
    /// config
    config: Config,
    /// connectors
    connectors: Vec<Box<dyn Connector>>,
    /// rules
    rules: BTreeMap<String, RuleImpl>,
    /// dice
    dice: Dice,
    /// db
    db: DB,
}
// ============================================================================
impl ::std::fmt::Debug for Aizuna {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            r##"Aizuna {{
        config: {:?},
        connectors: {:?},
        rules: {:?},
        dice: {:?}
 }}"##,
            self.config, self.connectors, self.rules, self.dice
        )
    }
}
// ============================================================================
impl Aizuna {
    // ========================================================================
    /// fn new
    pub fn new(
        config: Config,
        connectors: impl IntoIterator<Item = Box<dyn Connector>>,
        rules: impl IntoIterator<Item = (String, RuleImpl)>,
    ) -> Result<Self> {
        info!("Aizuna::new");
        let connectors = Vec::from_iter(connectors);
        if connectors.is_empty() {
            return Err(Error::InvalidArg(String::from(
                "input connectors is empty.",
            )));
        }
        let path_db = PathBuf::from(config.as_path_db());
        let path_db = path_db.to_str().ok_or_else(|| {
            Error::Aizuna(format!(
                "failed get path_db: {:?}",
                config.as_path_db()
            ))
        })?;
        Ok(Aizuna {
            config,
            connectors,
            rules: rules.into_iter().collect::<BTreeMap<_, _>>(),
            dice: Dice::new()?,
            db: Aizuna::new_db(path_db)?,
        })
    }
    // ========================================================================
    /// fn new_db
    fn new_db(path_db: &str) -> Result<DB> {
        let mut opt = Options::default();
        opt.compression_type = CompressionType::CompressionSnappy;
        let mut db = match DB::open(path_db, opt) {
            Ok(x) => x,
            Err(x) => return Err(Error::LevelDB(x)),
        };
        if let Some(ref x) = db.get(AIZUNA_DB_VERSION_KEY) {
            Aizuna::db_migrate(db, ::serde_json::from_slice::<i32>(x)?)
        } else {
            let mut batch = WriteBatch::new();
            batch.put(
                AIZUNA_DB_VERSION_KEY,
                &::serde_json::to_vec(&AIZUNA_DB_CURRENT)?,
            );
            db.write(batch, false)?;
            Ok(db)
        }
    }
    // ------------------------------------------------------------------------
    /// fn db_migrate
    fn db_migrate(db: DB, version: i32) -> Result<DB> {
        if version < (AIZUNA_DB_CURRENT - AIZUNA_DB_AGE)
            || AIZUNA_DB_CURRENT < version
        {
            return Err(Error::AizunaDBVer(
                version,
                AIZUNA_DB_CURRENT,
                AIZUNA_DB_AGE,
            ));
        }
        match version {
            0 => Ok(db),
            _ => Err(Error::AizunaDBVer(
                version,
                AIZUNA_DB_CURRENT,
                AIZUNA_DB_AGE,
            )),
        }
    }
    // ========================================================================
    /// fn on_message
    fn on_message(
        &mut self,
        message: &MessageAelicit,
    ) -> Result<Option<Command>> {
        message.with(move |msg: &dyn Message| -> Result<Option<Command>> {
            Behavior::on_msg(
                &self.config,
                &mut self.rules,
                &self.dice,
                &mut self.db,
                msg,
            )
        })
    }
    // ========================================================================
    #[cfg(feature = "coroutine_fringe")]
    /// fn gen
    pub(crate) fn gen(mut self, stack_size: usize) -> Result<()> {
        info!("Aizuna: Fringe");
        let mut gens = VecDeque::default();
        for x in self.connectors.iter() {
            gens.push_back(RefCell::new(
                x.gen(::fringe::OsStack::new(stack_size)?)?,
            ));
        }
        while let Some(con) = gens.pop_front() {
            let mut res = con.borrow_mut().resume(Command::Listen);
            debug!("{:?} / {:?}", con, res);
            while let Some(r) = res {
                match r {
                    Responce::Error(x) => {
                        eprintln!("Aizuna::run: Error: {:?}", x);
                        res = con.borrow_mut().resume(Command::Quit(None));
                    }
                    Responce::Yield => {
                        gens.push_back(con);
                        break;
                    }
                    Responce::Message(ref message) => {
                        match self.on_message(message) {
                            Err(x) => {
                                eprintln!("Aizuna::on_message: Error {:?}", x);
                                res = con.borrow_mut().resume(Command::Send(
                                    message.clone(),
                                    String::from("Inner error occured."),
                                ))
                            }
                            Ok(None) => {
                                gens.push_back(con); // Yield
                                break;
                            }
                            Ok(Some(cmd)) => {
                                res = con.borrow_mut().resume(cmd);
                            }
                        }
                    }
                }
                ::std::thread::yield_now();
            }
            debug!("Aizuna: DB.flush");
            if let Err(x) = self.db.flush() {
                eprintln!("Aizuna::spawn: DB.flush: {:?}", x);
            }
        }
        info!("Aizuna: Stop");
        Ok(())
    }
    // ========================================================================
    /// fn spawn
    pub(crate) fn spawn(&mut self) -> Result<()> {
        info!("Aizuna: Thread");
        let (res_rec, handles) = {
            // block for res_sen scope.
            let (res_sen, res_rec) = ::std::sync::mpsc::channel();
            let mut handles = Vec::default();
            for x in &self.connectors {
                handles.push(x.spawn(res_sen.clone())?);
            }
            (res_rec, handles)
        };
        // ====================================================================
        enum RecvErr {
            Disconnected,
            Timeout,
            Quit,
            SendError(::std::sync::mpsc::SendError<Command>),
        }
        // --------------------------------------------------------------------
        impl From<::std::sync::mpsc::SendError<Command>> for RecvErr {
            fn from(e: ::std::sync::mpsc::SendError<Command>) -> Self {
                RecvErr::SendError(e)
            }
        }
        // ====================================================================
        fn recv(
            aizuna: &mut Aizuna,
            res_rec: &ResRec,
        ) -> StdResult<(), RecvErr> {
            debug!("Aizuna: Recv");
            match res_rec.recv_timeout(Duration::from_millis(2000)) {
                Err(RecvTimeoutError::Disconnected) => {
                    Err(RecvErr::Disconnected)
                }
                Err(RecvTimeoutError::Timeout) => Err(RecvErr::Timeout),
                Ok((_, None)) => Err(RecvErr::Quit),
                Ok((Responce::Error(ref x), Some(ref cmd_sen))) => {
                    eprintln!("Aizuna::spawn: {:?}", x);
                    cmd_sen.send(Command::Quit(None))?;
                    Ok(())
                }
                Ok((Responce::Yield, Some(ref cmd_sen))) => {
                    debug!("Responce::Yield");
                    cmd_sen.send(Command::Listen)?;
                    Ok(())
                }
                Ok((Responce::Message(ref message), Some(ref cmd_sen))) => {
                    debug!("Responce::Message({:?})", message);
                    match aizuna.on_message(message) {
                        Err(x) => {
                            eprintln!("Aizuna::on_message: Error {:?}", x);
                            cmd_sen.send(Command::Send(
                                message.clone(),
                                String::from("Inner error occured."),
                            ))?;
                            Ok(())
                        }
                        Ok(None) => {
                            cmd_sen.send(Command::Listen)?;
                            Ok(())
                        }
                        Ok(Some(cmd)) => {
                            cmd_sen.send(cmd)?;
                            Ok(())
                        }
                    }
                }
            }
        }
        loop {
            match recv(self, &res_rec) {
                Err(RecvErr::Disconnected) => break,
                Err(RecvErr::Timeout) => {
                    debug!("Aizuna: DB.flush");
                    self.db.flush()?;
                }
                Err(RecvErr::SendError(x)) => {
                    eprintln!("Aizuna::spawn: SendError: {:?}", x)
                }
                Err(RecvErr::Quit) => {
                    debug!("Responce::Quit");
                }
                Ok(_) => {}
            }
        }
        for x in handles {
            match x.join() {
                Err(x) => eprintln!("Aizuna: spawn: join: {:?}", x),
                Ok(Err(x)) => eprintln!("Aizuna: spawn: join: {:?}", x),
                Ok(Ok(_)) => {}
            }
        }
        info!("Aizuna: Stop");
        Ok(())
    }
    // ========================================================================
    /// fn drive
    pub fn drive(mut self) -> Result<()> {
        #[allow(unreachable_patterns)]
        match *self.config.as_driver() {
            Driver::Thread => self.spawn(),
            #[cfg(feature = "coroutine_fringe")]
            Driver::Fringe => {
                let fringe_stack_size =
                    self.config.as_fringe_stack_size().clone();
                self.gen(fringe_stack_size)
            }
            _ => Err(Error::Aizuna(format!(
                "unsupported driver: {:?}",
                self.config.as_driver()
            ))),
        }
    }
}
