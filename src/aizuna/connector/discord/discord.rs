// -*- mode:rust; coding:utf-8-unix; -*-

//! discord.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/13
//  @date 2018/05/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeSet;
use std::thread::JoinHandle;
// ----------------------------------------------------------------------------
use discord::model::Event::MessageCreate;
use discord::model::UserId;
use toml::Value;
// ----------------------------------------------------------------------------
use super::super::super::{Command, MessageAelicit, Responce};
use super::super::{Connector, ResSen};
use super::{Config, DiscordMessage, Error, Result};
// ----------------------------------------------------------------------------
#[cfg(feature = "coroutine-fringe")]
use super::super::Generator;
#[cfg(feature = "coroutine")]
use super::receiver::Receiver;
#[cfg(feature = "coroutine")]
use std::sync::mpsc::TryRecvError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const DISCORD_MAX_LENGTH: usize = 1950usize;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Discord
#[derive(Debug, Clone)]
pub(crate) struct Discord {
    /// id
    id: String,
    /// config
    config: Config,
}
// ============================================================================
impl Discord {
    // ========================================================================
    /// new
    pub(crate) fn new(id: impl AsRef<str>, config: Value) -> Result<Self> {
        Ok(Discord {
            id: String::from(id.as_ref()),
            config: config.try_into::<Config>()?,
        })
    }
    // ========================================================================
    /// fn on_listen
    fn on_listen(
        id: &str,
        label: &str,
        state: &mut ::discord::State,
        result: ::discord::Result<::discord::model::Event>,
    ) -> Responce {
        match result {
            Err(x) => Responce::Error(Error::Discord(x)),
            Ok(x) => {
                state.update(&x);
                if let MessageCreate(m) = x {
                    debug!("{}Says: {}: {}", label, m.author.name, m.content);
                    Responce::Message(MessageAelicit::new(
                        DiscordMessage::new(id.to_string(), m),
                    ))
                } else {
                    Responce::Yield
                }
            }
        }
    }
    // ========================================================================
    #[cfg(feature = "coroutine")]
    /// fn on_listen_async
    fn on_listen_async(
        id: &str,
        label: &str,
        state: &mut ::discord::State,
        receiver: &Receiver,
    ) -> Responce {
        match receiver.try_recv() {
            Err(TryRecvError::Empty) => Responce::Yield,
            Err(TryRecvError::Disconnected) => Responce::Error(Error::Aizuna(
                String::from("Discord::connect: disconnected."),
            )),
            Ok(x) => Discord::on_listen(id, label, state, x),
        }
    }
    // ========================================================================
    /// fn on_quit
    fn on_quit(
        label: &str,
        discord: &::discord::Discord,
        msg: &MessageAelicit,
    ) -> Result<::discord::model::Message> {
        msg.with(|x| {
            if let Some(x) =
                x.as_any().downcast_ref::<::discord::model::Message>()
            {
                discord
                    .send_message(x.channel_id, "Aizuna Quit.", "", false)
                    .map_err(Error::Discord)
            } else {
                Err(Error::Downcast(format!("Discord::connect: {}", label)))
            }
        })
    }
    // ========================================================================
    /// fn on_send
    fn on_send(
        label: &str,
        discord: &::discord::Discord,
        msg: &MessageAelicit,
        s: &str,
    ) -> Responce {
        if let Err(x) = msg.with(|x| {
            if let Some(m) =
                x.as_any().downcast_ref::<::discord::model::Message>()
            {
                discord
                    .send_message(
                        m.channel_id,
                        format!(
                            "{}\n\t{}",
                            &s.chars()
                                .take(DISCORD_MAX_LENGTH)
                                .collect::<String>(),
                            m.author.mention()
                        ).as_str(),
                        "",
                        false,
                    )
                    .map_err(Error::Discord)
            } else {
                Err(Error::Downcast(format!("Discord::connect: {}", label)))
            }
        }) {
            Responce::Error(Error::Aizuna(format!("{}{:?}", label, x)))
        } else {
            Responce::Yield
        }
    }
    // ========================================================================
    /// fn on_whisper_
    fn on_whisper_(
        discord: &::discord::Discord,
        user_id: &str,
        s: &str,
    ) -> Result<Responce> {
        if s.is_empty() {
            return Ok(Responce::Yield);
        }
        let user_id = UserId(user_id.parse::<u64>()?);
        let _ = discord
            .send_message(
                discord
                    .create_private_channel(user_id)
                    .map_err(Error::Discord)?
                    .id,
                format!(
                    "{}\n\t{}",
                    &s.chars().take(DISCORD_MAX_LENGTH).collect::<String>(),
                    user_id.mention()
                ).as_str(),
                "",
                false,
            )
            .map_err(Error::Discord)?;
        Ok(Responce::Yield)
    }
    // ------------------------------------------------------------------------
    /// fn on_whisper
    fn on_whisper(
        _label: &str,
        discord: &::discord::Discord,
        vs: &BTreeSet<String>,
        s: &str,
    ) -> Responce {
        if s.is_empty() {
            return Responce::Yield;
        }
        for v in vs {
            if let Err(x) = Discord::on_whisper_(discord, v, s) {
                return Responce::Error(x);
            }
        }
        Responce::Yield
    }
    // ------------------------------------------------------------------------
    /// fn on_send_whisper_mine
    fn on_send_whisper_mine(
        label: &str,
        discord: &::discord::Discord,
        send: &(MessageAelicit, String),
        whisper: &(BTreeSet<String>, String),
        mine: &(String, String),
    ) -> Responce {
        let _ = Discord::on_send(label, discord, &send.0, &send.1);
        let _ = Discord::on_whisper(label, discord, &whisper.0, &whisper.1);
        if let Err(x) = Discord::on_whisper_(discord, &mine.0, &mine.1) {
            return Responce::Error(x);
        }
        Responce::Yield
    }
    // ========================================================================
    fn discord(
        &self,
    ) -> Result<(
        String,
        ::discord::Discord,
        ::discord::State,
        ::discord::Connection,
    )> {
        let label = format!("Discord({}): ", self.id);
        println!("{}Start", label);
        let discord =
            ::discord::Discord::from_bot_token(self.config.token.as_str())?;
        println!("{}Connect", label);
        let (connection, ready) = discord.connect()?;
        let state = ::discord::State::new(ready);
        {
            let channel_count: usize = state
                .servers()
                .iter()
                .map(|srv| {
                    srv.channels
                        .iter()
                        .filter(|chan| {
                            chan.kind == ::discord::model::ChannelType::Text
                        })
                        .count()
                })
                .sum();
            println!(
                "{}Ready: {}: {} servers with {} text channels",
                label,
                state.user().username,
                state.servers().len(),
                channel_count
            );
        }
        println!("{}Listen", label);
        Ok((label, discord, state, connection))
    }
}
// ============================================================================
impl Connector for Discord {
    // ========================================================================
    #[cfg(feature = "coroutine-fringe")]
    fn gen(&self, stack: ::fringe::OsStack) -> Result<Generator> {
        let id = self.id.clone();
        let (label, discord, mut state, connection) = self.discord()?;
        let receiver = Receiver::new(connection);
        Ok(Generator::new(stack, move |yielder, mut command| {
            println!("Discord: Gen");
            loop {
                command = match command {
                    Command::Quit(ref x) => {
                        if let &Some(ref msg) = x {
                            let _ = Discord::on_quit(&label, &discord, msg);
                        }
                        break;
                    }
                    Command::Listen => {
                        yielder.suspend(Discord::on_listen_async(
                            &id, &label, &mut state, &receiver,
                        ))
                    }
                    Command::Send(ref msg, ref s) => yielder
                        .suspend(Discord::on_send(&label, &discord, msg, s)),
                    Command::Whisper(ref vs, ref s) => yielder
                        .suspend(Discord::on_whisper(&label, &discord, vs, s)),
                    Command::SendWhisperMine(
                        ref send,
                        ref whisper,
                        ref mine,
                    ) => yielder.suspend(Discord::on_send_whisper_mine(
                        &label, &discord, send, whisper, mine,
                    )),
                }
            }
            println!("{}Disconnect", label);
            receiver.disconnect(&discord, &state);
        }))
    }
    // ========================================================================
    fn spawn(&self, res_sen: ResSen) -> Result<JoinHandle<Result<()>>> {
        let id = self.id.clone();
        let (label, discord, mut state, mut connection) = self.discord()?;
        let (cmd_sen, cmd_rec) = ::std::sync::mpsc::channel();
        Ok(::std::thread::spawn(move || {
            println!("{}Spawn", label);
            let _ = cmd_sen.send(Command::Listen)?;
            loop {
                debug!("Command::recv");
                match cmd_rec.recv() {
                    Err(_) => break,
                    Ok(Command::Quit(ref x)) => {
                        debug!("Command::Quit");
                        if let Some(ref msg) = *x {
                            let _ = Discord::on_quit(&label, &discord, msg)?;
                        }
                        break;
                    }
                    Ok(Command::Listen) => {
                        debug!("Command::Listen");
                        let _ = res_sen.send((
                            Discord::on_listen(
                                &id,
                                &label,
                                &mut state,
                                connection.recv_event(), // BLOCK
                            ),
                            Some(cmd_sen.clone()),
                        ))?;
                    }
                    Ok(Command::Send(ref msg, ref s)) => {
                        debug!("Command::Send");
                        let _ = res_sen.send((
                            Discord::on_send(&label, &discord, msg, s),
                            Some(cmd_sen.clone()),
                        ))?;
                    }
                    Ok(Command::Whisper(ref vs, ref s)) => {
                        debug!("Command::Whisper");
                        let _ = res_sen.send((
                            Discord::on_whisper(&label, &discord, vs, s),
                            Some(cmd_sen.clone()),
                        ))?;
                    }
                    Ok(Command::SendWhisperMine(
                        ref send,
                        ref whisper,
                        ref mine,
                    )) => {
                        debug!("Command::SendWhisper");
                        let _ = res_sen.send((
                            Discord::on_send_whisper_mine(
                                &label, &discord, send, whisper, mine,
                            ),
                            Some(cmd_sen.clone()),
                        ))?;
                    }
                }
            }
            println!("{}Disconnect", label);
            Ok(())
        }))
    }
}
