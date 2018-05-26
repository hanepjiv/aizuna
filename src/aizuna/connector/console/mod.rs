// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/28
//  @date 2018/05/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::any::Any as StdAny;
use std::thread::JoinHandle;
// ----------------------------------------------------------------------------
use super::{
    super::{
        Command, Message, MessageAelicit, MessageEAFS, MessageEAFSField,
        Responce,
    },
    Connector, ResSen,
};
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine")]
use std::collections::BTreeSet;
#[cfg(feature = "coroutine")]
use std::sync::mpsc::TryRecvError;
// ----------------------------------------------------------------------------
pub(crate) use super::Result;
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine")]
use self::receiver::Receiver;
#[cfg(feature = "coroutine")]
pub(crate) use super::Error;
#[cfg(feature = "coroutine-fringe")]
use super::Generator;
// mod  =======================================================================
#[cfg(feature = "coroutine")]
mod receiver;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// ConsoleMessage
#[derive(Debug, Clone)]
pub(crate) struct ConsoleMessage {
    _eafsf: MessageEAFSField,
    id: String,
    mention: String,
    msg: String,
}
// ============================================================================
impl MessageEAFS for ConsoleMessage {
    enable_aelicit_from_self_delegate!(Message, MessageAelicit, _eafsf);
}
// ============================================================================
impl ConsoleMessage {
    // ========================================================================
    /// new
    pub(crate) fn new(id: impl AsRef<str>, msg: impl AsRef<str>) -> Self {
        ConsoleMessage {
            _eafsf: MessageEAFSField::default(),
            id: String::from(id.as_ref()),
            mention: String::from("@") + id.as_ref(),
            msg: String::from(msg.as_ref()),
        }
    }
}
// ============================================================================
impl Message for ConsoleMessage {
    // ========================================================================
    fn as_any(&self) -> &dyn StdAny {
        &self.msg
    }
    // ========================================================================
    fn as_connector_type(&self) -> &str {
        "console"
    }
    // ========================================================================
    fn as_connector_id(&self) -> &str {
        &self.id
    }
    // ========================================================================
    fn as_author_name(&self) -> &str {
        &self.id
    }
    // ========================================================================
    fn as_author_mention(&self) -> &str {
        &self.mention
    }
    // ========================================================================
    fn as_author_id(&self) -> &str {
        &self.id
    }
    // ========================================================================
    fn as_channel_id(&self) -> &str {
        &self.id
    }
    // ========================================================================
    fn as_content(&self) -> &str {
        self.msg.as_str()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Console
#[derive(Debug, Clone)]
pub(crate) struct Console {
    /// id
    id: String,
}
// ============================================================================
impl Console {
    // ========================================================================
    /// new
    pub(crate) fn new(id: impl AsRef<str>) -> Self {
        println!("Console::new: *** Caution! This is a DEBUG console. ***");
        Console {
            id: String::from(id.as_ref()),
        }
    }
    // ========================================================================
    #[cfg(feature = "coroutine")]
    /// on_listen
    fn on_listen(&self, receiver: &mut Receiver) -> Responce {
        match receiver.try_recv() {
            Ok(input) => {
                println!(r##"Console: Says: {}"##, input);
                Responce::Message(MessageAelicit::new(ConsoleMessage::new(
                    self.id.clone(),
                    input,
                )))
            }
            Err(TryRecvError::Empty) => Responce::Yield,
            Err(TryRecvError::Disconnected) => {
                Responce::Error(Error::Aizuna(String::from(
                    "Console::on_listen: sync::channel sender disconnected.",
                )))
            }
        }
    }
    // ========================================================================
    #[cfg(feature = "coroutine")]
    /// on_send
    fn on_send(&self, msg: &MessageAelicit, s: &String) -> Responce {
        if s.is_empty() {
            return Responce::Yield;
        }
        if let Err(x) = msg.with(|x: &dyn Message| {
            x.as_any()
                .downcast_ref::<String>()
                .ok_or(Error::Downcast(String::from("Console::on_send")))
                .and_then(|m| {
                    Ok(println!(r##"Console: Send: {:?} => {}"##, m, s))
                })
        }) {
            Responce::Error(x)
        } else {
            Responce::Yield
        }
    }
    // ========================================================================
    #[cfg(feature = "coroutine")]
    /// on_whisper
    fn on_whisper(&self, _: &BTreeSet<String>, s: &String) -> Responce {
        if s.is_empty() {
            return Responce::Yield;
        }
        println!(r##"Console: Whisper: {}"##, s);
        Responce::Yield
    }
    // ========================================================================
    #[cfg(feature = "coroutine")]
    /// on_send_whisper_mine
    fn on_send_whisper_mine(
        &self,
        send: &(MessageAelicit, String),
        whisper: &(BTreeSet<String>, String),
        mine: &(String, String),
    ) -> Responce {
        let _ = self.on_send(&send.0, &send.1);
        let _ = self.on_whisper(&whisper.0, &whisper.1);
        if !mine.1.is_empty() {
            println!(r##"Console: Whisper: {}"##, &mine.1);
        }
        Responce::Yield
    }
}
// ============================================================================
impl Connector for Console {
    // ========================================================================
    #[cfg(feature = "coroutine-fringe")]
    fn gen(&self, stack: ::fringe::OsStack) -> Result<Generator> {
        let mut receiver = Receiver::new();
        Ok(Generator::new(stack, move |yielder, mut command| {
            println!("Console: Gen");
            loop {
                command = match command {
                    Command::Quit(_x) => {
                        break;
                    }
                    Command::Listen => {
                        yielder.suspend(self.on_listen(&mut receiver))
                    }
                    Command::Send(ref msg, ref s) => {
                        yielder.suspend(self.on_send(msg, s))
                    }
                    Command::Whisper(ref vs, ref s) => {
                        yielder.suspend(self.on_whisper(vs, s))
                    }
                    Command::SendWhisperMine(
                        ref send,
                        ref whisper,
                        ref mine,
                    ) => yielder.suspend(
                        self.on_send_whisper_mine(send, whisper, mine),
                    ),
                }
            }
            receiver.disconnect();
            println!("Console: Exit");
        }))
    }
    // ========================================================================
    fn spawn(&self, res_sen: ResSen) -> Result<JoinHandle<Result<()>>> {
        let id = self.id.clone();
        let (cmd_sen, cmd_rec) = ::std::sync::mpsc::channel();
        Ok(::std::thread::spawn(move || {
            println!("Console: Spawn");
            cmd_sen.send(Command::Listen)?;
            loop {
                debug!("Command::recv");
                match cmd_rec.recv() {
                    Err(_) => break,
                    Ok(Command::Quit(_x)) => {
                        debug!("Command::Quit");
                        res_sen.send((Responce::Yield, None))?;
                        break;
                    }
                    Ok(Command::Listen) => {
                        debug!("Command::Listen");
                        let mut i = String::default();
                        let _ = ::std::io::stdin().read_line(&mut i)?;
                        res_sen.send((
                            Responce::Message(MessageAelicit::new(
                                ConsoleMessage::new(
                                    id.clone(),
                                    String::from(i.trim()),
                                ),
                            )),
                            Some(cmd_sen.clone()),
                        ))?;
                    }
                    Ok(Command::Send(_msg, s)) => {
                        debug!("Command::Send");
                        if !s.is_empty() {
                            println!("Console: Send: {}", s);
                        }
                        res_sen
                            .send((Responce::Yield, Some(cmd_sen.clone())))?;
                    }
                    Ok(Command::Whisper(_, s)) => {
                        debug!("Command::Whisper");
                        if !s.is_empty() {
                            println!("Console: Whisper: {}", s);
                        }
                        res_sen
                            .send((Responce::Yield, Some(cmd_sen.clone())))?;
                    }
                    Ok(Command::SendWhisperMine(
                        ref send,
                        ref whisper,
                        ref mine,
                    )) => {
                        if !send.1.is_empty() {
                            println!("Console: Send: {}", send.1);
                        }
                        if !whisper.1.is_empty() {
                            println!("Console: Whisper: {}", whisper.1);
                        }
                        if !mine.1.is_empty() {
                            println!("Console: Whisper: {}", mine.1);
                        }
                        res_sen
                            .send((Responce::Yield, Some(cmd_sen.clone())))?;
                    }
                }
            }
            println!("Console: Exit");
            Ok(())
        }))
    }
}
