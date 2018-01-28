// -*- mode:rust; coding:utf-8-unix; -*-

//! aizuna.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/28
//  @date 2018/01/28

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine-fringe")]
use std::cell::RefCell;
#[cfg(feature = "coroutine-fringe")]
use std::collections::VecDeque;
// ----------------------------------------------------------------------------
use rusty_leveldb::{CompressionType, Options, DB};
// ----------------------------------------------------------------------------
use super::connector::{Connector, ResRec};
use super::rule::RuleImpl;
use super::{Behavior, Command, Config, Dice, Error, Message, MessageAelicit,
            Responce, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Driver
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Driver {
    /// Thread
    Thread,
    /// Fringe
    Fringe,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Aizuna
pub struct Aizuna {
    /// config
    config: Config,
    /// connectors
    connectors: Vec<Box<Connector>>,
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
    pub fn new<IC, IR>(
        config: Config,
        connectors: IC,
        rules: IR,
    ) -> Result<Self>
    where
        IC: IntoIterator<Item = Box<Connector>>,
        IR: IntoIterator<Item = (String, RuleImpl)>,
    {
        info!("Aizuna::new");
        let connectors = Vec::from_iter(connectors);
        if connectors.is_empty() {
            return Err(Error::InvalidArg(String::from(
                "input connectors is empty.",
            )));
        }
        let path_db = PathBuf::from(config.as_path_db());
        let path_db = path_db.to_str().ok_or(Error::Aizuna(format!(
            "failed get path_db: {:?}",
            config.as_path_db()
        )))?;
        Ok(Aizuna {
            config: config,
            connectors: connectors,
            rules: rules.into_iter().collect::<BTreeMap<_, _>>(),
            dice: Dice::new()?,
            db: {
                let mut opt = Options::default();
                opt.compression_type = CompressionType::CompressionSnappy;
                match DB::open(path_db, opt) {
                    Ok(x) => x,
                    Err(x) => return Err(Error::LevelDB(x)),
                }
            },
        })
    }
    // ========================================================================
    /// fn on_message
    fn on_message(
        &mut self,
        message: &MessageAelicit,
    ) -> Result<Option<Command>> {
        message.with(move |msg: &Message| -> Result<Option<Command>> {
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
    #[cfg(feature = "coroutine-fringe")]
    /// fn gen
    pub fn gen(mut self, stack_size: usize) -> Result<()> {
        info!("Aizuna: Fringe");
        let mut gens = VecDeque::default();
        for x in self.connectors.iter() {
            gens.push_back(RefCell::new(x.gen(::fringe::OsStack::new(
                stack_size,
            )?)?));
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
    pub fn spawn(&mut self) -> Result<()> {
        info!("Aizuna: Thread");
        let (res_rec, handles) = {
            // block for res_sen scope.
            let (res_sen, res_rec) = ::std::sync::mpsc::channel();
            let mut handles = Vec::default();
            for x in self.connectors.iter() {
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
                    Ok(cmd_sen.send(Command::Quit(None))?)
                }
                Ok((Responce::Yield, Some(ref cmd_sen))) => {
                    debug!("Responce::Yield");
                    Ok(cmd_sen.send(Command::Listen)?)
                }
                Ok((Responce::Message(ref message), Some(ref cmd_sen))) => {
                    debug!("Responce::Message({:?})", message);
                    match aizuna.on_message(message) {
                        Err(x) => {
                            eprintln!("Aizuna::on_message: Error {:?}", x);
                            Ok(cmd_sen.send(Command::Send(
                                message.clone(),
                                String::from("Inner error occured."),
                            ))?)
                        }
                        Ok(None) => Ok(cmd_sen.send(Command::Listen)?),
                        Ok(Some(cmd)) => Ok(cmd_sen.send(cmd)?),
                    }
                }
            }
        }
        loop {
            match recv(self, &res_rec) {
                Err(RecvErr::Disconnected) => break,
                Err(RecvErr::Timeout) => {
                    debug!("Aizuna: DB.flush");
                    let _ = self.db.flush()?;
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
    #[cfg(feature = "coroutine")]
    /// fn drive
    pub fn drive(mut self) -> Result<()> {
        #[allow(unreachable_patterns)]
        match self.config.as_driver() {
            &Driver::Thread => self.spawn(),
            #[cfg(feature = "coroutine-fringe")]
            &Driver::Fringe => {
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
    // ------------------------------------------------------------------------
    #[cfg(not(feature = "coroutine"))]
    /// fn drive
    pub fn drive(mut self) -> Result<()> {
        match self.config.as_driver() {
            &Driver::Thread => self.spawn(),
            _ => Err(Error::Aizuna(format!(
                "unsupported driver: {:?}",
                self.config.as_driver()
            ))),
        }
    }
}
