// -*- mode:rust; coding:utf-8-unix; -*-

//! behavior.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/09
//  @date 2018/10/03

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::{FromIterator, IntoIterator},
    result::Result as StdResult,
    str::from_utf8_unchecked,
};
// ----------------------------------------------------------------------------
use rusty_leveldb::{LdbIterator, WriteBatch, DB};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    rule::{Rule, RuleImpl},
    Command, Config, Dice, Error, Message, Result, Session, SessionImpl, User,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type UserSessions
type UserSessions = BTreeSet<Uuid>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Behavior
pub(crate) struct Behavior<'a, 'b>
where
    'b: 'a,
{
    /// db
    pub(crate) db: &'a mut DB,
    /// msg
    pub(crate) msg: &'a dyn Message,
    /// inputs
    pub(crate) inputs: &'a mut Vec<String>,
    /// user
    pub(crate) user: &'a User<'b>,
    /// options
    options: ::getopts::Options,
}
// ============================================================================
impl<'a, 'b> ::std::fmt::Debug for Behavior<'a, 'b>
where
    'b: 'a,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            r##"Behavior {{
  msg:          {:?},
  inputs:       {:?},
  user:         {:?},
 }}"##,
            self.msg, self.inputs, self.user,
        )
    }
}
// ============================================================================
impl<'a, 'b> Behavior<'a, 'b>
where
    'b: 'a,
{
    // ========================================================================
    /// fn key_user_sessions
    fn key_user_sessions(uuid: &Uuid) -> String {
        let mut ret = String::from("aizuna-user-sessions=");
        ret += &uuid.to_string();
        ret
    }
    // ------------------------------------------------------------------------
    /// fn key_user_default_session_uuid
    fn key_user_default_session_uuid(uuid: &Uuid) -> String {
        let mut ret = String::from("aizuna-user-default-session_uuid=");
        ret += &uuid.to_string();
        ret
    }
    // ------------------------------------------------------------------------
    /// fn key_session
    fn key_session(uuid: &Uuid) -> String {
        let mut ret = String::from("aizuna-session=");
        ret += &uuid.to_string();
        ret
    }
    // ========================================================================
    /// fn send
    pub(crate) fn send(&self, s: impl Into<String>) -> Command {
        Command::Send(self.msg.aelicit().expect("aelicit"), s.into())
    }
    // ------------------------------------------------------------------------
    /// fn whisper
    pub(crate) fn whisper(&self, s: impl Into<String>) -> Command {
        let mut x = BTreeSet::<String>::new();
        let _ = x.insert(String::from(self.user.as_author_id()));
        Command::Whisper(x, s.into())
    }
    // ------------------------------------------------------------------------
    /// fn multi_whisper
    pub(crate) fn multi_whisper(
        users: impl IntoIterator<Item = String>,
        s: impl Into<String>,
    ) -> Command {
        Command::Whisper(
            BTreeSet::<String>::from_iter(users.into_iter()),
            s.into(),
        )
    }
    // ------------------------------------------------------------------------
    /// fn owners_member_author_id
    pub(crate) fn owners_member_author_id(
        &mut self,
        label: impl AsRef<str>,
        session: &SessionImpl,
    ) -> BTreeSet<String> {
        let mut ret = BTreeSet::default();
        for i in session.owners_member_iter() {
            match Behavior::get_user(label.as_ref(), &mut self.db, i) {
                (_, None) => {
                    continue;
                }
                (_, Some(y)) => {
                    let _ = ret.insert(String::from(y.as_author_id()));
                }
            }
        }
        ret
    }
    // ------------------------------------------------------------------------
    /// fn session_whisper
    pub(crate) fn session_whisper(
        &mut self,
        label: impl AsRef<str>,
        session: &SessionImpl,
        s: impl Into<String>,
    ) -> Command {
        Behavior::multi_whisper(
            self.owners_member_author_id(label, session),
            s,
        )
    }
    // ------------------------------------------------------------------------
    /// fn session_send_whisper_mine
    pub(crate) fn session_send_whisper_mine(
        &mut self,
        label: impl AsRef<str>,
        session: &SessionImpl,
        s0: impl Into<String>,
        s1: impl Into<String>,
        s2: impl Into<String>,
    ) -> Command {
        let mut whispers = self.owners_member_author_id(label, session);
        let _ = whispers.remove(self.user.as_author_id());
        Command::SendWhisperMine(
            (self.msg.aelicit().expect("aelicit"), s0.into()),
            (whispers, s1.into()),
            (String::from(self.user.as_author_id()), s2.into()),
        )
    }
    // ========================================================================
    /// fn need_admin
    pub(crate) fn need_admin(
        &self,
        func: impl FnOnce() -> Result<Command>,
    ) -> Result<Command> {
        if *self.user.as_admin() {
            func()
        } else {
            Ok(self.whisper(format!(
                "{} command needs the administrator's authority.",
                self.inputs[0]
            )))
        }
    }
    // ========================================================================
    /// fn is_admin
    fn is_admin(config: &Config, msg: &dyn Message) -> bool {
        if let Some(x) = config.as_admin().get(msg.as_connector_id()) {
            for re in x.iter() {
                if re.is_match(msg.as_author_id()) {
                    return true;
                }
            }
        }
        false
    }
    // ========================================================================
    /// fn new_user
    fn new_user(
        config: &Config,
        db: &mut DB,
        msg: &'a dyn Message,
        key_user_uuid: &str,
        key_user: &str,
        user_uuid: &Uuid,
    ) -> Result<User<'a>> {
        let user = User::new(
            *user_uuid,
            msg.as_connector_id(),
            msg.as_author_id(),
            msg.as_author_name(),
            Behavior::is_admin(config, msg),
        );
        let mut batch = WriteBatch::new();
        batch.put(key_user_uuid.as_bytes(), user_uuid.to_string().as_bytes());
        batch.put(key_user.as_bytes(), &::serde_json::to_vec(&user)?);
        db.write(batch, false)?;
        Ok(user)
    }
    // ========================================================================
    /// fn get_user
    pub(crate) fn get_user(
        label: impl AsRef<str>,
        db: &mut DB,
        user_uuid: &Uuid,
    ) -> (String, Option<User<'a>>) {
        let mut key_user = String::from("aizuna-user=");
        key_user += &user_uuid.to_string();
        if let Some(u) = db.get(key_user.as_bytes()) {
            let user = if let Ok(x) = ::serde_json::from_slice::<User>(&u) {
                x
            } else {
                eprintln!(
                    "{}failed user from_slice. {}",
                    label.as_ref(),
                    key_user
                );
                return (key_user, None);
            };
            if user_uuid != user.as_uuid() {
                eprintln!(
                    "{}db user is invalid. {} != {}.",
                    label.as_ref(),
                    user_uuid,
                    user.as_uuid(),
                );
                (key_user, None)
            } else {
                (key_user, Some(user))
            }
        } else {
            eprintln!("{}db user not found. {}.", label.as_ref(), key_user);
            (key_user, None)
        }
    }
    // ========================================================================
    /// fn get_current_user
    fn get_current_user(
        label: impl AsRef<str>,
        config: &Config,
        db: &mut DB,
        msg: &'a dyn Message,
    ) -> Result<User<'a>> {
        let user_id = User::make_id(msg.as_connector_id(), msg.as_author_id());

        let mut key_user_uuid = String::from("aizuna-user-uuid=");
        key_user_uuid += &user_id;

        if let Some(x) = db.get(key_user_uuid.as_bytes()) {
            let user_uuid =
                Uuid::parse_str(unsafe { from_utf8_unchecked(&x) })?;
            match Behavior::get_user(label, db, &user_uuid) {
                (x, Some(mut y)) => {
                    if msg.as_author_name() == y.as_author_name() {
                        y.set_admin(Behavior::is_admin(config, msg));
                        Ok(y)
                    } else {
                        Behavior::new_user(
                            config,
                            db,
                            msg,
                            &key_user_uuid,
                            &x,
                            &user_uuid,
                        )
                    }
                }
                (x, None) => Behavior::new_user(
                    config,
                    db,
                    msg,
                    &key_user_uuid,
                    &x,
                    &user_uuid,
                ),
            }
        } else {
            let (user_uuid, key_user) =
                Behavior::make_unique_uuid_key(db, "aizuna-user=");
            Behavior::new_user(
                config,
                db,
                msg,
                &key_user_uuid,
                &key_user,
                &user_uuid,
            )
        }
    }
    // ========================================================================
    /// fn make_unique_uuid_key
    pub(crate) fn make_unique_uuid_key(
        db: &mut DB,
        prefix: impl Into<String>,
    ) -> (Uuid, String) {
        let mut uuid;
        let mut key;
        let pre = prefix.into();
        loop {
            uuid = Uuid::new_v4();
            key = pre.clone();
            key += &uuid.to_string();
            if db.get(key.as_bytes()).is_none() {
                break;
            }
        }
        (uuid, key)
    }
    // ========================================================================
    /// fn on_msg
    pub(crate) fn on_msg(
        config: &Config,
        rules: &mut BTreeMap<String, RuleImpl>,
        dice: &Dice,
        db: &mut DB,
        msg: &'a dyn Message,
    ) -> Result<Option<Command>> {
        let label = "Aizuna: ";

        let mut inputs: Vec<String> = msg
            .as_content()
            .split_whitespace()
            .map(String::from)
            .collect();
        if inputs.is_empty() {
            return Ok(None); // Yield
        }
        inputs[0] = {
            let prefix = config.as_prefix().as_bytes();
            let head: &[u8] = inputs[0].as_bytes();
            let l = prefix.len();
            if head.len() <= l || &head[..l] != prefix {
                return Ok(None); // Yield
            }
            String::from(unsafe { from_utf8_unchecked(&head[l..]) })
        };

        let user = Behavior::get_current_user(label, config, db, msg)?;

        Behavior::new(db, msg, &mut inputs, &user)
            .on_input(config, rules, dice)
    }
    // ========================================================================
    /// fn new
    fn new(
        db: &'a mut DB,
        msg: &'a dyn Message,
        inputs: &'a mut Vec<String>,
        user: &'a User<'b>,
    ) -> Self {
        let mut opts = ::getopts::Options::new();
        let _ = opts.optopt("s", "session", "set session", "SESSION_UUID");
        Behavior {
            db,
            msg,
            inputs,
            user,
            options: opts,
        }
    }
    // ========================================================================
    /// fn help_msg
    fn help_msg(prefix: &str) -> String {
        format!(
            r##"
Aizuna v{0}:
 {1}help              Print this message.
 {1}user / {1}u         Print User info.
 {1}session / {1}s      Session controll.
 {1}[0-9]*d[0-9]*     Dice roll. etc. {1}2d6 / {1}3d / {1}d10
 {1}quit / {1}Q         Aizuna logout server. Need administrator's authority.
"##,
            // {1}database Dump database. Need administrator's authority.
            env!("CARGO_PKG_VERSION"),
            prefix
        )
    }
    // ========================================================================
    /// fn on_input
    fn on_input(
        &mut self,
        config: &'a Config,
        rules: &'a mut BTreeMap<String, RuleImpl>,
        dice: &'a Dice,
    ) -> Result<Option<Command>> {
        Ok(Some(match self.inputs[0].as_bytes() {
            b"help" => self.whisper(Behavior::help_msg(config.as_prefix())),
            b"quit" | b"Q" => self.need_admin(|| {
                Ok(Command::Quit(Some(self.msg.aelicit().expect("aelicit"))))
            })?,
            b"user" | b"u" => self.on_user()?,
            b"database" => self.on_database()?,
            b"greeting" => self.on_greeting(config)?,
            b"session" | b"s" => self.on_session(rules)?,
            _ => {
                if let Ok(cmd) = self.on_rules(rules) {
                    cmd
                } else if let Ok((n, m, v, ret)) = dice.parse(&self.inputs[0])
                {
                    self.send(format!("{}d{} = {:?} = {}.", n, m, v, ret,))
                } else {
                    self.whisper(format!(
                        "unsupported command: {}. See {}help.",
                        self.inputs[0],
                        config.as_prefix(),
                    ))
                }
            }
        }))
    }
    // ========================================================================
    /// fn on_user
    fn on_user(&mut self) -> Result<Command> {
        let label = "user: ";
        Ok(self.send(format!("{}Uuid: {}", label, self.user.as_uuid())))
    }
    // ========================================================================
    /// fn on_database
    fn on_database(&mut self) -> Result<Command> {
        let mut iter = self.db.new_iter()?;
        self.need_admin(move || {
            let mut count = 0usize;
            while let Some((k, v)) = iter.next() {
                if count > 100 {
                    println!("Aizuna database dump limit.");
                    break;
                }
                count += 1;
                println!(
                    "\n {}\n\t{}",
                    unsafe { ::std::str::from_utf8_unchecked(&k) },
                    unsafe { ::std::str::from_utf8_unchecked(&v) }
                );
            }
            Ok(Command::Listen)
        })
    }
    // ========================================================================
    /// fn on_greeting
    fn on_greeting(&mut self, config: &Config) -> Result<Command> {
        Ok(self.whisper(config.as_greeting()))
    }
    // ========================================================================
    /// fn on_session
    fn on_session(
        &mut self,
        rules: &'a mut BTreeMap<String, RuleImpl>,
    ) -> Result<Command> {
        if self.inputs.len() < 2 || self.inputs[1].is_empty() {
            return self.on_session_list(false);
        }
        match self.inputs[1].as_bytes() {
            b"ls" | b"list" => self.on_session_list(false),
            b"all" => self.on_session_list(true),
            b"default" => self.on_session_default(),
            b"new" => self.on_session_new(rules),
            b"close" => self.on_session_close(),
            b"reopen" => self.on_session_reopen(),
            b"delete" => self.on_session_delete(),
            b"info" => self.on_session_info(),
            b"title" => self.on_session_title(),
            b"invite" => self.on_session_invite(),
            b"kick" => self.on_session_kick(),
            b"owner" => self.on_session_owner(),
            b"waiver" => self.on_session_waiver(),
            b"request" => self.on_session_request(),
            b"bye" => self.on_session_bye(),
            _ => Ok(self.whisper(format!(
                "session: unsupported command {}.",
                self.inputs[1],
            ))),
        }
    }
    // ========================================================================
    /// fn user_sessions
    fn user_sessions(
        &mut self,
        label: impl AsRef<str>,
        uuid: &Uuid,
    ) -> StdResult<(String, UserSessions), Result<Command>> {
        let key_user_sessions = Behavior::key_user_sessions(uuid);
        let user_sessions = {
            if let Some(ref x) = self.db.get(key_user_sessions.as_bytes()) {
                if let Ok(x) = ::serde_json::from_slice::<UserSessions>(x) {
                    x
                } else {
                    eprintln!(
                        "{}failed ::serde_json::from_slice({:?}).",
                        label.as_ref(),
                        &key_user_sessions
                    );
                    return Err(Ok(self
                        .whisper(format!("{}Inner Error.", label.as_ref()))));
                }
            } else {
                UserSessions::default()
            }
        };
        Ok((key_user_sessions, user_sessions))
    }
    // ========================================================================
    /// fn session
    fn session(
        &mut self,
        label: impl AsRef<str>,
        session_uuid: &Uuid,
    ) -> StdResult<(String, SessionImpl), Result<Command>> {
        let key_session = Behavior::key_session(session_uuid);
        let session = if let Some(ref x) = self.db.get(key_session.as_bytes())
        {
            if let Ok(x) = ::serde_json::from_slice::<SessionImpl>(x) {
                x
            } else {
                eprintln!(
                    "{}failed ::serde_json::from_slice({:?}).",
                    label.as_ref(),
                    &key_session
                );
                return Err(Ok(self.whisper("{}Inner Error.")));
            }
        } else {
            return Err(Ok(self.whisper(format!(
                "{}session({}) is not found.",
                label.as_ref(),
                session_uuid,
            ))));
        };
        Ok((key_session, session))
    }
    // ========================================================================
    /// fn parse_uuid
    pub(crate) fn parse_uuid(
        &self,
        label: impl AsRef<str>,
        uuid_str: impl AsRef<str>,
    ) -> StdResult<Uuid, Result<Command>> {
        if let Ok(x) = Uuid::parse_str(uuid_str.as_ref()) {
            Ok(x)
        } else {
            return Err(Ok(self.whisper(format!(
                "{}{} is invalid uuid.",
                label.as_ref(),
                uuid_str.as_ref(),
            ))));
        }
    }
    // ========================================================================
    /// fn get_uuid
    pub(crate) fn get_uuid(
        &mut self,
        label: impl AsRef<str>,
        key: &[u8],
    ) -> StdResult<Uuid, Result<Command>> {
        if let Some(ref x) = self.db.get(key) {
            self.parse_uuid(label, unsafe { from_utf8_unchecked(x) })
        } else {
            return Err(Ok(self
                .whisper(format!("{}default is not found.", label.as_ref()))));
        }
    }
    // ========================================================================
    /// fn user_default_session
    fn user_default_session(
        &mut self,
        label: impl AsRef<str>,
    ) -> StdResult<(String, SessionImpl), Result<Command>> {
        let user_default_session_uuid = self.get_uuid(
            &label,
            Behavior::key_user_default_session_uuid(self.user.as_uuid())
                .as_bytes(),
        )?;
        self.session(label, &user_default_session_uuid)
    }
    // ========================================================================
    /// fn current_session
    pub(crate) fn current_session(
        &mut self,
        label: impl AsRef<str>,
    ) -> StdResult<(String, SessionImpl, ::getopts::Matches), Result<Command>>
    {
        let matches = match self.options.parse(&self.inputs[1..]) {
            Err(_) => {
                return Err(Ok(self.whisper(format!(
                    "{}failed parse options.",
                    label.as_ref(),
                ))));
            }
            Ok(x) => x,
        };

        if let Some(ref x) = matches.opt_str("s") {
            self.parse_uuid(&label, &x).and_then(|uuid| {
                self.session(label, &uuid)
                    .and_then(|(x, y)| Ok((x, y, matches)))
            })
        } else {
            self.user_default_session(label)
                .and_then(|(x, y)| Ok((x, y, matches)))
        }
    }
    // ========================================================================
    /// fn on_session_list
    fn on_session_list(&mut self, all: bool) -> Result<Command> {
        let label = "session.list: ";

        let (key_user_sessions, mut user_sessions) =
            match self.user_sessions(label, self.user.as_uuid()) {
                Err(x) => return x,
                Ok(x) => x,
            };

        let mut list = Vec::default();
        for v in &user_sessions {
            let key_session = Behavior::key_session(v);
            if let Some(ref x) = self.db.get(key_session.as_bytes()) {
                let mut session = ::serde_json::from_slice::<SessionImpl>(x)?;
                list.push(session);
            }
        }

        user_sessions.clear();
        for v in &list {
            let _ = user_sessions.insert(v.as_uuid().clone());
        }
        {
            let mut batch = WriteBatch::new();
            batch.put(
                key_user_sessions.as_bytes(),
                &::serde_json::to_vec(&user_sessions)?,
            );
            self.db.write(batch, false)?;
        }

        list.sort_unstable_by(|lhs, rhs| rhs.as_utc().cmp(lhs.as_utc()));

        let user_default_session_uuid = match self.get_uuid(
            label,
            Behavior::key_user_default_session_uuid(self.user.as_uuid())
                .as_bytes(),
        ) {
            Err(_) => None,
            Ok(x) => Some(x),
        };

        let mut ret = String::from(label);
        for v in &list {
            if all || v.is_open() {
                ret += &format!(
                    "\n {} {} {} {}  {}  {}  {}",
                    if let Some(ref x) = user_default_session_uuid {
                        if x == v.as_uuid() {
                            '*'
                        } else {
                            ' '
                        }
                    } else {
                        ' '
                    },
                    if v.as_owners().contains(self.user.as_uuid()) {
                        'o'
                    } else {
                        ' '
                    },
                    if !v.is_open() { 'x' } else { ' ' },
                    v.as_title(),
                    v.as_rule_name(),
                    v.with_local().format("%Y-%m-%d %H:%M %:z"),
                    v.as_uuid(),
                );
            }
        }

        Ok(self.whisper(ret))
    }
    // ========================================================================
    /// fn on_session_default
    fn on_session_default(&mut self) -> Result<Command> {
        let label = "session.default: ";

        if 3 < self.inputs.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        if self.inputs.len() < 3 || self.inputs[2].is_empty() {
            match self.user_default_session(label) {
                Err(x) => return x,
                Ok((_key_session, session)) => {
                    return Ok(self.whisper(format!(
                        "{}{} ({})",
                        label,
                        session.as_title(),
                        session.as_uuid(),
                    )));
                }
            }
        }

        let (_key_session, session) = {
            match self.parse_uuid(&label, self.inputs[2].as_str()) {
                Err(x) => return x,
                Ok(uuid) => match self.session(label, &uuid) {
                    Err(x) => return x,
                    Ok(x) => x,
                },
            }
        };

        if !session.owners_member_contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not a member.", label)));
        }

        let mut batch = WriteBatch::new();
        batch.put(
            Behavior::key_user_default_session_uuid(self.user.as_uuid())
                .as_bytes(),
            session.as_uuid().to_string().as_bytes(),
        );
        self.db.write(batch, false)?;

        Ok(self.whisper(format!(
            "{}{} ({})",
            label,
            session.as_title(),
            session.as_uuid(),
        )))
    }
    // ========================================================================
    /// fn on_session_new
    fn on_session_new(
        &mut self,
        rules: &'a mut BTreeMap<String, RuleImpl>,
    ) -> Result<Command> {
        let label = "session.new: ";

        if self.inputs.len() < 3
            || 3 < self.inputs.len()
            || self.inputs[2].is_empty()
        {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        let (session_uuid, key_session) =
            Behavior::make_unique_uuid_key(&mut self.db, "aizuna-session=");

        let (key_user_sessions, mut user_sessions) =
            match self.user_sessions(label, self.user.as_uuid()) {
                Err(x) => return x,
                Ok(x) => x,
            };

        let session_kind = match {
            let mut ret = Err(Ok(self
                .whisper(format!("{}not found {}.", label, self.inputs[2]))));
            for rule in rules.values_mut() {
                println!("{}, {}", self.inputs[2], rule.as_rule_name());
                if self.inputs[2].as_bytes() == rule.as_rule_name().as_bytes()
                {
                    ret = rule.new_session_kind().map_err(|_| {
                        Ok(self.whisper(format!(
                            "{}failed new session kind {}.",
                            label, self.inputs[2]
                        )))
                    });
                    break;
                }
            }
            ret
        } {
            Err(x) => return x,
            Ok(x) => x,
        };

        let session = SessionImpl::new(
            session_uuid,
            vec![*self.user.as_uuid()],
            session_kind,
        );
        let _ = user_sessions.insert(session_uuid);

        let mut batch = WriteBatch::new();
        batch.put(
            key_user_sessions.as_bytes(),
            &::serde_json::to_vec(&user_sessions)?,
        );
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        batch.put(
            Behavior::key_user_default_session_uuid(self.user.as_uuid())
                .as_bytes(),
            session_uuid.to_string().as_bytes(),
        );
        self.db.write(batch, false)?;

        Ok(self.send(format!("{}Session Uuid = {}", label, session_uuid,)))
    }
    // ========================================================================
    /// fn on_session_close
    fn on_session_close(&mut self) -> Result<Command> {
        let label = "session.close: ";

        let (key_session, mut session, matches) =
            match self.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owned.", label)));
        }

        if !session.is_open() {
            return Ok(self.whisper(format!("{}already closed.", label)));
        }

        let _ = session.close();
        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        self.db.write(batch, false)?;

        Ok({
            let s = format!(
                "{}{} close {} ({}).",
                label,
                self.user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_reopen
    fn on_session_reopen(&mut self) -> Result<Command> {
        let label = "session.reopen: ";

        let (key_session, mut session, matches) =
            match self.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owned.", label)));
        }

        if session.is_open() {
            return Ok(self.whisper(format!("{}already open.", label)));
        }

        let _ = session.open();
        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        self.db.write(batch, false)?;

        Ok({
            let s = format!(
                "{}{} reopen {} ({}).",
                label,
                self.user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_delete
    fn on_session_delete(&mut self) -> Result<Command> {
        let label = "session.delete: ";

        let (key_session, session, matches) = match self.current_session(label)
        {
            Err(x) => return x,
            Ok(x) => x,
        };

        if 1 < matches.free.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owned.", label)));
        }

        if session.as_owners().len() != 1 {
            return Ok(
                self.whisper(format!("{}other owner still exist.", label))
            );
        }

        if session.is_open() {
            return Ok(self.whisper(format!("{}still open.", label)));
        }

        let (key_user_sessions, mut user_sessions) =
            match self.user_sessions(label, self.user.as_uuid()) {
                Err(x) => return x,
                Ok(x) => x,
            };

        let _ = user_sessions.remove(session.as_uuid());

        let mut batch = WriteBatch::new();
        batch.delete(key_session.as_bytes());
        batch.put(
            key_user_sessions.as_bytes(),
            &::serde_json::to_vec(&user_sessions)?,
        );
        self.db.write(batch, false)?;

        Ok({
            let s = format!(
                "{}{} delete {} ({}).",
                label,
                self.user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_info
    fn on_session_info(&mut self) -> Result<Command> {
        let label = "session.info: ";

        let (_key_session, session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if !session.owners_member_contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not a member.", label)));
        }

        if 1 < matches.free.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        Ok(self.whisper(format!("{}{}", label, session)))
    }
    // ========================================================================
    /// fn on_session_title
    fn on_session_title(&mut self) -> Result<Command> {
        let label = "session.title: ";

        let (key_session, mut session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        let title = match matches.free.len() {
            1 => None,
            2 => Some(matches.free[1].as_str()),
            _ => {
                return Ok(self.whisper(format!("{}unknown options.", label)));
            }
        };

        if let Some(x) = title {
            if !session.as_owners().contains(self.user.as_uuid()) {
                return Ok(self.whisper(format!("{}not owned.", label)));
            }

            if !session.is_open() {
                return Ok(self.whisper(format!("{}closed session.", label)));
            }

            let _ = session.set_title(&x);
            let mut batch = WriteBatch::new();
            batch
                .put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
            self.db.write(batch, false)?;
        }

        Ok({
            let s = format!(
                "{}{} title {} ({}).",
                label,
                self.user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_invite
    fn on_session_invite(&mut self) -> Result<Command> {
        let label = "session.invite: ";

        let (key_session, mut session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owned.", label)));
        }

        if !session.is_open() {
            return Ok(self.whisper(format!("{}closed session.", label)));
        }

        let (_key_target_user, target_user) = {
            let target_user_uuid = match matches.free.len() {
                1 => {
                    return Ok(self.whisper(format!("{}no user uuid.", label)));
                }
                2 => match self.parse_uuid(&label, matches.free[1].as_str()) {
                    Err(x) => return x,
                    Ok(uuid) => uuid,
                },
                _ => {
                    return Ok(
                        self.whisper(format!("{}unknown options.", label))
                    );
                }
            };
            match Behavior::get_user(&label, &mut self.db, &target_user_uuid) {
                (_, None) => {
                    return Ok(
                        self.whisper(format!("{}user not found.", label))
                    );
                }
                (x, Some(y)) => (x, y),
            }
        };

        if target_user.as_connector_id() != self.user.as_connector_id() {
            return Ok(self.whisper(format!("{}other connector user.", label)));
        }

        if session.owners_member_contains(target_user.as_uuid()) {
            return Ok(self.whisper(format!("{}already a member.", label)));
        }

        let _ = session
            .as_member_mut()
            .insert(target_user.as_uuid().clone());

        let (key_target_user_sessions, mut target_user_sessions) =
            match self.user_sessions(label, target_user.as_uuid()) {
                Err(x) => return x,
                Ok(x) => x,
            };

        let _ = target_user_sessions.insert(session.as_uuid().clone());

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        batch.put(
            key_target_user_sessions.as_bytes(),
            &::serde_json::to_vec(&target_user_sessions)?,
        );
        self.db.write(batch, false)?;

        Ok({
            let s = format!(
                "{}{} invite {} in {} ({}).",
                label,
                self.user.as_author_name(),
                target_user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_kick
    fn on_session_kick(&mut self) -> Result<Command> {
        let label = "session.kick: ";

        let (key_session, mut session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owned.", label)));
        }

        if !session.is_open() {
            return Ok(self.whisper(format!("{}closed session.", label)));
        }

        let (_key_target_user, target_user) = {
            let target_user_uuid = match matches.free.len() {
                1 => {
                    return Ok(self.whisper(format!("{}no user uuid.", label)));
                }
                2 => match self.parse_uuid(&label, matches.free[1].as_str()) {
                    Err(x) => return x,
                    Ok(uuid) => uuid,
                },
                _ => {
                    return Ok(
                        self.whisper(format!("{}unknown options.", label))
                    );
                }
            };
            match Behavior::get_user(&label, &mut self.db, &target_user_uuid) {
                (_, None) => {
                    return Ok(
                        self.whisper(format!("{}user not found.", label))
                    );
                }
                (x, Some(y)) => (x, y),
            }
        };

        if !session.owners_member_contains(target_user.as_uuid()) {
            return Ok(self.whisper(format!("{}not a member.", label)));
        }

        // make whisper command for return befor remove session mameber.
        let ret = {
            let s = format!(
                "{}{} kick {} ({}) from {} ({}).",
                label,
                self.user.as_author_name(),
                target_user.as_author_name(),
                target_user.as_uuid(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        };

        let _ = session.as_member_mut().remove(target_user.as_uuid());

        let (key_target_user_sessions, mut target_user_sessions) =
            match self.user_sessions(label, target_user.as_uuid()) {
                Err(x) => return x,
                Ok(x) => x,
            };
        let _ = target_user_sessions.remove(session.as_uuid());

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        batch.put(
            key_target_user_sessions.as_bytes(),
            &::serde_json::to_vec(&target_user_sessions)?,
        );
        self.db.write(batch, false)?;

        Ok(ret)
    }
    // ========================================================================
    /// fn on_session_owner
    fn on_session_owner(&mut self) -> Result<Command> {
        let label = "session.owner: ";

        let (key_session, mut session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owned.", label)));
        }

        if !session.is_open() {
            return Ok(self.whisper(format!("{}closed session.", label)));
        }

        let (_key_target_user, target_user) = {
            let target_user_uuid = match matches.free.len() {
                1 => {
                    return Ok(self.whisper(format!("{}no user uuid.", label)));
                }
                2 => match self.parse_uuid(&label, matches.free[1].as_str()) {
                    Err(x) => return x,
                    Ok(uuid) => uuid,
                },
                _ => {
                    return Ok(
                        self.whisper(format!("{}unknown options.", label))
                    );
                }
            };
            match Behavior::get_user(&label, &mut self.db, &target_user_uuid) {
                (_, None) => {
                    return Ok(
                        self.whisper(format!("{}user not found.", label))
                    );
                }
                (x, Some(y)) => (x, y),
            }
        };

        if session.as_owners().contains(target_user.as_uuid()) {
            return Ok(self.whisper(format!("{}already owner.", label)));
        }

        if !session.as_member().contains(target_user.as_uuid()) {
            return Ok(self.whisper(format!("{}not a member.", label)));
        }

        let _ = session
            .as_owners_mut()
            .insert(target_user.as_uuid().clone());
        let _ = session.as_member_mut().remove(target_user.as_uuid());

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        self.db.write(batch, false)?;

        Ok({
            let s = format!(
                "{}{} gave {} ({}) {} ({}) ownership.",
                label,
                self.user.as_author_name(),
                target_user.as_author_name(),
                target_user.as_uuid(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_waiver
    fn on_session_waiver(&mut self) -> Result<Command> {
        let label = "session.waiver: ";

        let (key_session, mut session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        if !session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not owner.", label)));
        }

        let _ = session.as_owners_mut().remove(self.user.as_uuid());
        let _ = session.as_member_mut().insert(self.user.as_uuid().clone());

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        self.db.write(batch, false)?;

        Ok({
            let s = format!(
                "{}{} waiver {} ({}).",
                label,
                self.user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_request
    fn on_session_request(&mut self) -> Result<Command> {
        let label = "session.request: ";

        if self.inputs.len() < 3
            || 3 < self.inputs.len()
            || self.inputs[2].is_empty()
        {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        let (_key_session, mut session) =
            match self.parse_uuid(&label, &self.inputs[2]) {
                Err(x) => return x,
                Ok(uuid) => match self.session(label, &uuid) {
                    Err(x) => return x,
                    Ok(x) => x,
                },
            };

        if session.owners_member_contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}already member.", label)));
        }

        let _ = session.as_member_mut().insert(self.user.as_uuid().clone());
        // for session_whisper. NOT Save DB.

        Ok({
            let s = format!(
                "{}{} ({}) request {} ({}).",
                label,
                self.user.as_author_name(),
                self.user.as_uuid(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        })
    }
    // ========================================================================
    /// fn on_session_bye
    fn on_session_bye(&mut self) -> Result<Command> {
        let label = "session.bye: ";

        let (key_session, mut session, matches) =
            match self.current_session(label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(self.whisper(format!("{}invalid args.", label)));
        }

        if session.as_owners().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}still owner.", label)));
        }

        if !session.as_member().contains(self.user.as_uuid()) {
            return Ok(self.whisper(format!("{}not a member.", label)));
        }

        // make whisper command for return befor remove session mameber.
        let ret = {
            let s = format!(
                "{}{} bye from {} ({}).",
                label,
                self.user.as_author_name(),
                session.as_title(),
                session.as_uuid(),
            );
            self.session_whisper(&label, &session, s)
        };

        let _ = session.as_member_mut().remove(self.user.as_uuid());

        let (key_user_sessions, mut user_sessions) =
            match self.user_sessions(label, self.user.as_uuid()) {
                Err(x) => return x,
                Ok(x) => x,
            };
        let _ = user_sessions.remove(session.as_uuid());

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        batch.put(
            key_user_sessions.as_bytes(),
            &::serde_json::to_vec(&user_sessions)?,
        );
        self.db.write(batch, false)?;

        Ok(ret)
    }
    // ========================================================================
    /// fn on_rules
    fn on_rules(
        &mut self,
        rules: &'a mut BTreeMap<String, RuleImpl>,
    ) -> Result<Command> {
        for (k, mut v) in rules.iter_mut() {
            let l = k.as_bytes().len();
            if k.as_bytes() == &(self.inputs[0].as_bytes())[..l] {
                self.inputs[0] = String::from(unsafe {
                    from_utf8_unchecked(&(self.inputs[0].as_bytes())[l..])
                });
                return v.run(self);
            }
        }
        Err(Error::Aizuna(format!(
            "unsupported command: {}.",
            self.inputs[0]
        )))
    }
}
