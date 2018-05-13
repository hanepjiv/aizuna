// -*- mode:rust; coding:utf-8-unix; -*-

//! shinen.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/25
//  @date 2018/05/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{path::Path, result::Result as StdResult};
// ----------------------------------------------------------------------------
use rusty_leveldb::WriteBatch;
use toml::Value;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    super::{
        super::{Behavior, Command, SessionKind}, Rule,
    },
    CardMap, Config, Deck, Hand, Player, PlayerType, Result, Session,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ShinEn
pub struct ShinEn {
    /// config
    config: Config,
    /// cards
    cards: CardMap,
}
// ============================================================================
impl ::std::fmt::Debug for ShinEn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "ShinEn {{ config: {:?}, cards: {:?}}}",
            self.config, self.cards
        )
    }
}
// ============================================================================
impl Rule for ShinEn {
    // ========================================================================
    fn as_rule_name(&self) -> &str {
        "shinen"
    }
    // ========================================================================
    fn run(&mut self, bhv: &mut Behavior) -> Result<Command> {
        if bhv.inputs.is_empty() || bhv.inputs[0].is_empty() {
            return Ok(bhv.whisper(String::from("ShineEn: empty command.")));
        }
        match bhv.inputs[0].as_bytes() {
            b"reload" => self.on_reload(bhv),
            b"card" => self.on_card(bhv),
            b"s" | b"session" => self.on_session(bhv),
            b"shuffle" => self.on_shuffle(bhv),
            b"tsukimachi" => self.on_tsukimachi(bhv),
            b"p" | b"player" => self.on_player(bhv),
            b"deck" => self.on_deck(bhv),
            b"h" | b"hand" => self.on_hand(bhv),
            b"d" | b"draw" => self.on_draw(bhv),
            b"pick" => self.on_pick(bhv),
            b"u" | b"use" => self.on_use(bhv),
            b"totop" => self.on_totop(bhv),
            b"t" | b"top" => self.on_top(bhv),
            b"give" => self.on_give(bhv),
            _ => Ok(bhv.whisper(format!(
                "ShinEn: unsupported command {}.",
                bhv.inputs[0]
            ))),
        }
    }
    // ========================================================================
    fn new_session_kind(&mut self) -> Result<SessionKind> {
        use rand::{thread_rng, Rng};
        let mut pile = self.cards.keys().cloned().collect::<Vec<String>>();
        thread_rng().shuffle(&mut pile[..]);
        Ok(SessionKind::ShinEn(Session::new(Deck::from(pile))))
    }
}
// ============================================================================
impl ShinEn {
    // ========================================================================
    /// fn new
    pub(crate) fn new(path_root: &Path, config: Value) -> Result<Self> {
        info!(
            "::aizuna::shinen::ShiEn::new({:?}, {:?})",
            path_root, config
        );
        let mut config = config.try_into::<Config>()?;
        if config.root.is_relative() {
            let mut ret = path_root.to_path_buf();
            ret.push(config.root);
            config.root = ret
        };
        let mut shinen = ShinEn {
            config,
            cards: CardMap::default(),
        };
        let _ = shinen.reload_cards()?;
        Ok(shinen)
    }
    // ========================================================================
    /// fn reload_cards
    pub(crate) fn reload_cards(&mut self) -> Result<&Self> {
        self.cards.clear();
        let _ = super::card::import_cards(
            &mut self.cards,
            self.config.root.as_os_str(),
        )?;
        Ok(self)
    }
    // ========================================================================
    fn on_reload(&mut self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.reload: ";
        bhv.need_admin(|| {
            Ok(if let Err(x) = self.reload_cards() {
                eprintln!("{}failed: {:?}", label, x);
                bhv.whisper(format!("{}failed.", label))
            } else {
                bhv.whisper(format!("{}succeeeded.", label))
            })
        })
    }
    // ========================================================================
    fn on_card(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.cards: ";

        if bhv.inputs.len() < 2 || bhv.inputs[1].is_empty() {
            return Ok(bhv.whisper(format!("{}empty args.", label)));
        }

        Ok(
            bhv.send(if let Some(card) = self.cards.get(&bhv.inputs[1]) {
                card.pretty()
            } else {
                format!("{}card not found {}.", label, bhv.inputs[1])
            }),
        )
    }
    // ========================================================================
    fn on_session(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.session: ";

        let (_key_session, session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 0 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let shinen_session = if let Some(x) = session.as_kind().as_shinen() {
            x
        } else {
            eprintln!("{}kind != ShinEn", label);
            return Ok(bhv.whisper(format!("{}Inner Error.", label)));
        };

        Ok(bhv.whisper(format!("{} {}", label, shinen_session)))
    }
    // ========================================================================
    fn on_shuffle(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.shuffle: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 0 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        {
            let shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };
            shinen_session.shuffle();
        }

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.session_whisper(&label, &session, format!("{}", label)))
    }
    // ========================================================================
    fn on_tsukimachi(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.tsukimachi: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 0 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let s = {
            let shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };
            shinen_session.tsukimachi();

            format!(
                "{} pile: {} / discard: {} {:?}",
                label,
                shinen_session.as_pile().len(),
                shinen_session.as_discard().len(),
                shinen_session.as_discard()
            )
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.session_whisper(&label, &session, s))
    }
    // ========================================================================
    fn on_player(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player: ";
        if bhv.inputs.len() < 2 || bhv.inputs[1].is_empty() {
            return self.on_player_list(bhv);
        }
        match bhv.inputs[1].as_bytes() {
            b"ls" | b"list" => self.on_player_list(bhv),
            b"default" => self.on_player_default(bhv),
            b"new" => self.on_player_new(bhv),
            b"delete" => self.on_player_delete(bhv),
            b"assign" => self.on_player_assign(bhv),
            b"name" => self.on_player_name(bhv),
            b"type" => self.on_player_type(bhv),
            _ => Ok(bhv.whisper(format!(
                "{}unsupported command {}.",
                label, bhv.inputs[1]
            ))),
        }
    }
    // ========================================================================
    fn default_player<'a>(
        &self,
        bhv: &mut Behavior,
        label: impl AsRef<str>,
        shinen_session: &'a Session,
    ) -> StdResult<&'a Player, Result<Command>> {
        shinen_session
            .as_default_player(bhv.user.as_uuid())
            .ok_or_else(|| {
                Ok(bhv.whisper(format!(
                    "{} player not found.",
                    label.as_ref(),
                )))
            })
    }
    // ------------------------------------------------------------------------
    fn default_player_mut<'a>(
        &self,
        bhv: &mut Behavior,
        label: impl AsRef<str>,
        shinen_session: &'a mut Session,
    ) -> StdResult<&'a mut Player, Result<Command>> {
        shinen_session
            .as_default_player_mut(bhv.user.as_uuid())
            .ok_or_else(|| {
                Ok(bhv.whisper(
                    format!("{}player not found.", label.as_ref(),),
                ))
            })
    }
    // ========================================================================
    fn parse_player_uuid_str<'a>(
        &self,
        bhv: &mut Behavior,
        label: impl AsRef<str>,
        shinen_session: &'a Session,
        uuid_str: impl AsRef<str>,
    ) -> StdResult<&'a Player, Result<Command>> {
        if let Ok(uuid) = Uuid::parse_str(uuid_str.as_ref()) {
            match shinen_session.as_players().get(&uuid).ok_or_else(|| {
                Ok(bhv.whisper(format!(
                    "{}player not found {}.",
                    label.as_ref(),
                    uuid
                )))
            }) {
                x @ Err(_) => return x,
                Ok(x) => Ok(x),
            }
        } else {
            eprintln!(
                "{}failed Uuid::parse_str({:?}).",
                label.as_ref(),
                uuid_str.as_ref()
            );
            return Err(Ok(bhv.whisper(format!(
                "{}{} is invalid uuid.",
                label.as_ref(),
                uuid_str.as_ref()
            ))));
        }
    }
    // ========================================================================
    fn on_player_list(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.list: ";

        let (_key_session, session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let shinen_session = if let Some(x) = session.as_kind().as_shinen() {
            x
        } else {
            eprintln!("{}kind != ShinEn", label);
            return Ok(bhv.whisper(format!("{}Inner Error.", label)));
        };

        let current_player =
            self.default_player(bhv, &label, shinen_session).ok();

        let mut ret = String::from(label);
        for (k, v) in shinen_session.as_players().iter() {
            ret += &format!(
                "\n {} {} {} {} ({})",
                if let Some(x) = current_player {
                    if x.as_uuid() == k {
                        '*'
                    } else {
                        ' '
                    }
                } else {
                    ' '
                },
                if bhv.user.as_uuid() == v.as_user_uuid() {
                    'o'
                } else {
                    ' '
                },
                v.as_hand().len(),
                v.as_name(),
                k,
            );
        }

        Ok(bhv.whisper(ret.as_str()))
    }
    // ========================================================================
    fn on_player_default(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.default: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 2 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (player_name, player_uuid) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            if matches.free.len() < 2 || matches.free[1].is_empty() {
                let player =
                    match self.default_player(bhv, &label, shinen_session) {
                        Err(x) => return x,
                        Ok(x) => x,
                    };
                return Ok(bhv.whisper(format!(
                    "{}{} ({}).",
                    label,
                    player.as_name(),
                    player.as_uuid()
                )));
            }

            let (player_name, player_uuid) = {
                let player = {
                    let uuid_str = matches.free[1].clone();
                    match self.parse_player_uuid_str(
                        bhv,
                        &label,
                        shinen_session,
                        &uuid_str,
                    ) {
                        Err(x) => return x,
                        Ok(x) => x,
                    }
                };

                if player.as_user_uuid() != bhv.user.as_uuid() {
                    return Ok(bhv.whisper(format!(
                        "{}player not owned.",
                        label
                    )));
                }

                (player.as_name().to_string(), *player.as_uuid())
            };

            let _ = shinen_session.insert_default_player(
                bhv.user.as_uuid().clone(),
                player_uuid,
            );

            (player_name, player_uuid)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(
            bhv.whisper(format!(
                "{}{} ({}).",
                label, player_name, player_uuid
            )),
        )
    }
    // ========================================================================
    fn on_player_new(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.new: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 2 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let player_uuid = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let roll = if matches.free.len() < 2 || matches.free[1].is_empty()
            {
                PlayerType::Player
            } else if let Ok(x) = matches.free[1].parse::<PlayerType>() {
                x
            } else {
                return Ok(bhv.whisper(format!(
                    "{}unsupported player type {}.",
                    label, matches.free[1]
                )));
            };

            let player_uuid = {
                let mut ret = Uuid::new_v4();
                loop {
                    if !shinen_session.as_players().contains_key(&ret) {
                        break;
                    }
                }
                ret
            };

            let _ = shinen_session.as_players_mut().insert(
                player_uuid,
                Player::new(
                    player_uuid,
                    *bhv.user.as_uuid(),
                    bhv.user.as_author_name(),
                    roll,
                ),
            );

            let _ = shinen_session.insert_default_player(
                bhv.user.as_uuid().clone(),
                player_uuid,
            );

            player_uuid
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.send(format!("{}{}.", label, player_uuid,)))
    }
    // ========================================================================
    fn on_player_delete(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.delete: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 2 != matches.free.len() || matches.free[1].is_empty() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let player_uuid =
            match bhv.parse_uuid(&label, matches.free[1].as_str()) {
                Err(x) => return x,
                Ok(uuid) => uuid,
            };

        let is_owner = session.as_owners().contains(bhv.user.as_uuid());

        let (player_name, player_uuid) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let (player_name, player_uuid, mut hand) = {
                let player = if let Some(x) =
                    shinen_session.as_players().get(&player_uuid)
                {
                    x
                } else {
                    return Ok(bhv.whisper(format!(
                        "{}player not found.",
                        label
                    )));
                };

                if !is_owner && bhv.user.as_uuid() != player.as_user_uuid() {
                    return Ok(bhv.whisper(format!(
                        "{}player not owned.",
                        label
                    )));
                }

                (
                    String::from(player.as_name()),
                    *player.as_uuid(),
                    player.as_hand().clone(),
                )
            };

            shinen_session.as_discard_mut().append(&mut hand);
            let _ = shinen_session.as_players_mut().remove(&player_uuid);

            (player_name, player_uuid)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.session_whisper(
            &label,
            &session,
            format!(
                "{}{} delete {} ({}) from {} ({}).",
                label,
                bhv.user.as_author_name(),
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
            ),
        ))
    }
    // ========================================================================
    fn on_player_assign(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.assign: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 3 != matches.free.len() || matches.free[1].is_empty()
            || matches.free[2].is_empty()
        {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let player_uuid =
            match bhv.parse_uuid(&label, matches.free[1].as_str()) {
                Err(x) => return x,
                Ok(uuid) => uuid,
            };

        let target_user_uuid =
            match bhv.parse_uuid(&label, matches.free[2].as_str()) {
                Err(x) => return x,
                Ok(uuid) => uuid,
            };

        if !session.owners_member_contains(&target_user_uuid) {
            return Ok(bhv.whisper(format!("{}not a member.", label)));
        }

        let _target_user = match Behavior::get_user(
            &label,
            bhv.db,
            &target_user_uuid,
        ) {
            (_x, None) => {
                return Ok(bhv.whisper(format!("{}user not found.", label)));
            }
            (_x, Some(mut y)) => y,
        };

        if !session.as_owners().contains(bhv.user.as_uuid()) {
            return Ok(bhv.whisper(format!("{}session not owned.", label)));
        }

        let (player_name, player_uuid, prev_user_uuid) = {
            let shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let player = if let Some(x) =
                shinen_session.as_players_mut().get_mut(&player_uuid)
            {
                x
            } else {
                return Ok(bhv.whisper(format!("{}player not found.", label)));
            };

            let prev_user_uuid = *player.as_user_uuid();

            if target_user_uuid == prev_user_uuid {
                return Ok(bhv.whisper(format!("{}same user.", label)));
            }

            let _ = player.set_user_uuid(target_user_uuid);

            (
                String::from(player.as_name()),
                *player.as_uuid(),
                prev_user_uuid,
            )
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.session_whisper(
            &label,
            &session,
            format!(
                "{}{} assign {} ({}) user ({}) -> ({}) from {} ({}).",
                label,
                bhv.user.as_author_name(),
                player_name,
                player_uuid,
                prev_user_uuid,
                target_user_uuid,
                session.as_title(),
                session.as_uuid(),
            ),
        ))
    }
    // ========================================================================
    fn on_player_name(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.name: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if matches.free.len() < 2 || 2 < matches.free.len()
            || matches.free[1].is_empty()
        {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (prev_name, player_name, player_uuid) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let mut player = match self.default_player_mut(
                bhv,
                &label,
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            if bhv.user.as_uuid() != player.as_user_uuid() {
                return Ok(bhv.whisper(format!("{}player not owned.", label)));
            }

            let prev_name = String::from(player.as_name());
            let _ = player.set_name(matches.free[1].as_str());

            (prev_name, String::from(player.as_name()), *player.as_uuid())
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.session_whisper(
            &label,
            &session,
            format!(
                "{}{} chanege name {} -> {} ({}) in {} ({}).",
                label,
                bhv.user.as_author_name(),
                prev_name,
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
            ),
        ))
    }
    // ========================================================================
    fn on_player_type(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.player.type: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if matches.free.len() < 2 || 2 < matches.free.len()
            || matches.free[1].is_empty()
        {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (prev_type, player_type, player_name, player_uuid) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let mut player = match self.default_player_mut(
                bhv,
                &label,
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            if bhv.user.as_uuid() != player.as_user_uuid() {
                return Ok(bhv.whisper(format!("{}player not owned.", label)));
            }

            let prev_type = *player.as_player_type();
            if let Ok(x) = matches.free[1].as_str().parse::<PlayerType>() {
                let _ = player.set_player_type(x);
            } else {
                return Ok(bhv.whisper(format!("{}invalid type.", label)));
            }

            (
                prev_type,
                *player.as_player_type(),
                String::from(player.as_name()),
                *player.as_uuid(),
            )
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        Ok(bhv.session_whisper(
            &label,
            &session,
            format!(
                "{}{} chanege type {} -> {} {} ({}) in {} ({}).",
                label,
                bhv.user.as_author_name(),
                prev_type.as_ref(),
                player_type.as_ref(),
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
            ),
        ))
    }

    // ========================================================================
    fn on_deck(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.deck: ";

        let (_key_session, session, _matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        let shinen_session = if let Some(x) = session.as_kind().as_shinen() {
            x
        } else {
            eprintln!("{}kind != ShinEn", label);
            return Ok(bhv.whisper(format!("{}Inner Error.", label)));
        };

        Ok(bhv.send(format!(
            "{} pile: {} / discard: {} {:?}",
            label,
            shinen_session.as_pile().len(),
            shinen_session.as_discard().len(),
            shinen_session.as_discard()
        )))
    }
    // ========================================================================
    fn choice_hand<'a>(
        bhv: &mut Behavior,
        label: impl AsRef<str>,
        hand: &'a Hand,
        num_str: impl AsRef<str>,
    ) -> StdResult<(usize, &'a String), Result<Command>> {
        let n = if let Ok(x) = num_str.as_ref().parse::<usize>() {
            x
        } else {
            return Err(Ok(bhv.whisper(format!(
                "{}invalid arg. {}",
                label.as_ref(),
                num_str.as_ref()
            ))));
        };

        if hand.is_empty() {
            return Err(Ok(bhv.whisper(format!(
                "{}empty hand.",
                label.as_ref()
            ))));
        }

        if (hand.len() - 1) < n {
            return Err(Ok(bhv.whisper(format!(
                "{}out of the range of the hand.",
                label.as_ref()
            ))));
        }

        Ok(if let Some(x) = hand.get(n) {
            (n, x)
        } else {
            return Err(Ok(bhv.whisper(format!(
                "{}failed get card from hand.",
                label.as_ref()
            ))));
        })
    }
    // ========================================================================
    fn on_hand(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.hand. ";

        let (_key_session, session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (player_name, player_uuid, ret) = {
            let shinen_session = if let Some(x) = session.as_kind().as_shinen()
            {
                x
            } else {
                eprintln!("{}kind != ShinEn", label);
                return Ok(bhv.whisper(format!("{}Inner Error.", label)));
            };

            let player = match self.default_player(bhv, &label, shinen_session)
            {
                Err(x) => return x,
                Ok(x) => x,
            };

            if bhv.user.as_uuid() != player.as_user_uuid() {
                return Ok(bhv.whisper(format!("{}player not owned.", label)));
            }

            let ret = if matches.free.is_empty() || matches.free[0].is_empty()
            {
                if let Some(x) = player.hand_to_string(&self.cards) {
                    x
                } else {
                    return Ok(bhv.whisper(format!(
                        "{}failed get card from info.",
                        label
                    )));
                }
            } else {
                let (_card_idx, card_name) = match ShinEn::choice_hand(
                    bhv,
                    &label,
                    player.as_hand(),
                    &matches.free[0],
                ) {
                    Err(x) => return x,
                    Ok(x) => x,
                };
                let card = if let Some(x) = self.cards.get(card_name) {
                    x
                } else {
                    return Ok(bhv.whisper(format!(
                        "{}failed get card from info.",
                        label
                    )));
                };
                card.pretty()
            };

            (String::from(player.as_name()), *player.as_uuid(), ret)
        };

        Ok(bhv.whisper(format!(
            "{}{} hand {} ({}) in {} ({})\n{}",
            label,
            bhv.user.as_author_name(),
            player_name,
            player_uuid,
            session.as_title(),
            session.as_uuid(),
            ret
        )))
    }
    // ========================================================================
    fn on_draw(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.draw: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let n = if matches.free.len() < 1 || matches.free[0].is_empty() {
            1
        } else if let Ok(x) = matches.free[0].parse::<usize>() {
            x
        } else {
            return Ok(bhv.whisper(format!(
                "{}failed parse number. {}",
                label, matches.free[0]
            )));
        };

        let (player_name, player_uuid, hand) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let cards = {
                let mut ret = Vec::default();
                for _ in 0..n {
                    if let Some(x) = shinen_session.draw() {
                        ret.push(x);
                    } else {
                        return Ok(bhv.send(format!("{}no pile.", label)));
                    }
                }
                ret
            };

            let mut player = match self.default_player_mut(
                bhv,
                &label,
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            if bhv.user.as_uuid() != player.as_user_uuid() {
                return Ok(bhv.whisper(format!("{}player not owned.", label)));
            }

            for i in &cards {
                player.as_hand_mut().push_back(i.to_string());
            }

            let hand = if let Some(x) = player.hand_to_string(&self.cards) {
                x
            } else {
                return Ok(bhv.whisper(format!(
                    "{}failed get card from info.",
                    label
                )));
            };

            (String::from(player.as_name()), *player.as_uuid(), hand)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        let s0 = format!(
            "{}{} draw {} {} ({}) in {} ({}).",
            label,
            bhv.user.as_author_name(),
            n,
            player_name,
            player_uuid,
            session.as_title(),
            session.as_uuid(),
        );
        Ok(bhv.session_send_whisper_mine(
            &label,
            &session,
            s0.as_str(),
            s0.as_str(),
            format!(
                "{}{} draw {} {} ({}) in {} ({}).\n{}",
                label,
                bhv.user.as_author_name(),
                n,
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
                hand
            ),
        ))
    }
    // ========================================================================
    fn on_pick(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.pick: ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (player_name, player_uuid, card_name, hand) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let card_name = if let Some(x) =
                shinen_session.pick(matches.free[0].as_bytes())
            {
                x.clone()
            } else {
                return Ok(bhv.send(format!(
                    "{}{} not found.",
                    label, matches.free[0]
                )));
            };

            let mut player = match self.default_player_mut(
                bhv,
                &label,
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            if bhv.user.as_uuid() != player.as_user_uuid() {
                return Ok(bhv.whisper(format!("{}player not owned.", label)));
            }

            player.as_hand_mut().push_back(card_name.clone());

            let hand = if let Some(x) = player.hand_to_string(&self.cards) {
                x
            } else {
                return Ok(bhv.whisper(format!(
                    "{}failed get card from info.",
                    label
                )));
            };

            (
                String::from(player.as_name()),
                *player.as_uuid(),
                card_name,
                hand,
            )
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        let s0 = format!(
            "{}{} pick {} {} ({}) in {} ({}).",
            label,
            bhv.user.as_author_name(),
            card_name,
            player_name,
            player_uuid,
            session.as_title(),
            session.as_uuid(),
        );
        Ok(bhv.session_send_whisper_mine(
            &label,
            &session,
            s0.as_str(),
            s0.as_str(),
            format!(
                "{}{} pick {} ({}) in {} ({}).\n{}",
                label,
                bhv.user.as_author_name(),
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
                hand
            ),
        ))
    }
    // ========================================================================
    fn hand_pickup(
        &self,
        bhv: &mut Behavior,
        label: impl AsRef<str>,
        num_str: impl AsRef<str>,
        shinen_session: &mut Session,
    ) -> StdResult<(String, Uuid, String, String), Result<Command>> {
        let player = self.default_player_mut(bhv, &label, shinen_session)?;

        if bhv.user.as_uuid() != player.as_user_uuid() {
            return Err(Ok(bhv.whisper(format!(
                "{}player not owned.",
                label.as_ref()
            ))));
        }

        let (card_idx, card_name) = match ShinEn::choice_hand(
            bhv,
            &label,
            player.as_hand(),
            num_str.as_ref(),
        ) {
            Err(x) => return Err(x),
            Ok((card_idx, card_name)) => (card_idx, card_name.clone()),
        };

        let _ = player.as_hand_mut().remove(card_idx);

        let hand = if let Some(x) = player.hand_to_string(&self.cards) {
            x
        } else {
            return Err(Ok(bhv.whisper(format!(
                "{}failed get card from info.",
                label.as_ref()
            ))));
        };

        Ok((
            String::from(player.as_name()),
            *player.as_uuid(),
            card_name.clone(),
            hand,
        ))
    }
    // ========================================================================
    fn on_use(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.use. ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 != matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (player_name, player_uuid, card, hand) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let (player_name, player_uuid, card_name, hand) = match self.hand_pickup(
                bhv,
                &label,
                &matches.free[0],
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            let _ = shinen_session.discard(card_name.clone());

            let card = if let Some(x) = self.cards.get(&card_name) {
                x
            } else {
                return Ok(bhv.whisper(format!(
                    "{}failed get card from info.",
                    label
                )));
            };

            (player_name, player_uuid, card.pretty(), hand)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        let s0 = format!(
            "{}{} use {} ({}) in {} ({})\n{}",
            label,
            bhv.user.as_author_name(),
            player_name,
            player_uuid,
            session.as_title(),
            session.as_uuid(),
            card
        );
        Ok(bhv.session_send_whisper_mine(
            &label,
            &session,
            s0.as_str(),
            s0.as_str(),
            format!(
                "{}{} use {} ({}) in {} ({}).\n{}",
                label,
                bhv.user.as_author_name(),
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
                hand
            ),
        ))
    }
    // ========================================================================
    fn on_totop(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.totop. ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 != matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let (player_name, player_uuid, card, hand) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let (player_name, player_uuid, card_name, hand) = match self.hand_pickup(
                bhv,
                &label,
                &matches.free[0],
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            let _ = shinen_session.totop(card_name.clone());

            let card = if let Some(x) = self.cards.get(&card_name) {
                x
            } else {
                return Ok(bhv.whisper(format!(
                    "{}failed get card from info.",
                    label
                )));
            };

            (player_name, player_uuid, card.pretty(), hand)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        let s0 = format!(
            "{}{} totop {} ({}) in {} ({}).\n{}",
            label,
            bhv.user.as_author_name(),
            player_name,
            player_uuid,
            session.as_title(),
            session.as_uuid(),
            card
        );
        Ok(bhv.session_send_whisper_mine(
            &label,
            &session,
            s0.as_str(),
            s0.as_str(),
            format!(
                "{}{} totop {} ({}) in {} ({}).\n{}",
                label,
                bhv.user.as_author_name(),
                player_name,
                player_uuid,
                session.as_title(),
                session.as_uuid(),
                hand
            ),
        ))
    }
    // ========================================================================
    fn on_top(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.top. ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 1 < matches.free.len() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }
        let n = if matches.free.len() < 1 || matches.free[0].is_empty() {
            1
        } else if let Ok(x) = matches.free[0].parse::<usize>() {
            x
        } else {
            return Ok(bhv.whisper(format!(
                "{}failed parse number. {}",
                label, matches.free[0]
            )));
        };

        let (player_name, player_uuid, cards) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let (player_name, player_uuid) = {
                let mut player = match self.default_player_mut(
                    bhv,
                    &label,
                    &mut shinen_session,
                ) {
                    Err(x) => return x,
                    Ok(x) => x,
                };

                if bhv.user.as_uuid() != player.as_user_uuid() {
                    return Ok(bhv.whisper(format!(
                        "{}player not owned.",
                        label
                    )));
                }

                (String::from(player.as_name()), *player.as_uuid())
            };

            let mut cards = Deck::default();
            for _i in 0..n {
                if let Some(x) = shinen_session.draw() {
                    let _ = shinen_session.discard(x.clone());
                    cards.push_back(x);
                } else {
                    return Ok(bhv.send(format!("{}no pile.", label)));
                }
            }

            let mut ret = if let Some(x) = cards.to_string(&self.cards) {
                x
            } else {
                return Ok(bhv.whisper(format!("{}unknown card.", label)));
            };

            (player_name, player_uuid, ret)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        let s0 = format!(
            "{}{} top {} ({}) in {} ({})\n{}",
            label,
            bhv.user.as_author_name(),
            player_name,
            player_uuid,
            session.as_title(),
            session.as_uuid(),
            cards
        );
        Ok(bhv.session_send_whisper_mine(
            &label,
            &session,
            s0.as_str(),
            s0.as_str(),
            "",
        ))
    }
    // ========================================================================
    fn on_give(&self, bhv: &mut Behavior) -> Result<Command> {
        let label = "ShinEn.give. ";

        let (key_session, mut session, matches) =
            match bhv.current_session(&label) {
                Err(x) => return x,
                Ok(x) => x,
            };

        if 2 != matches.free.len() || matches.free[1].is_empty() {
            return Ok(bhv.whisper(format!("{}invalid args.", label)));
        }

        let target_player_uuid =
            match bhv.parse_uuid(&label, matches.free[1].as_str()) {
                Err(x) => return x,
                Ok(uuid) => uuid,
            };

        let (player_name, target_player_name, hand) = {
            let mut shinen_session =
                if let Some(x) = session.as_kind_mut().as_shinen_mut() {
                    x
                } else {
                    eprintln!("{}kind != ShinEn", label);
                    return Ok(bhv.whisper(format!("{}Inner Error.", label)));
                };

            let (player_name, _player_uuid, card_name, hand) = match self.hand_pickup(
                bhv,
                &label,
                &matches.free[0],
                &mut shinen_session,
            ) {
                Err(x) => return x,
                Ok(x) => x,
            };

            let mut target_player = if let Some(x) =
                shinen_session.as_players_mut().get_mut(&target_player_uuid)
            {
                x
            } else {
                return Ok(bhv.whisper(format!(
                    "{}target not found ({}).",
                    label, target_player_uuid
                )));
            };

            let _ = target_player.as_hand_mut().push_back(card_name);

            (player_name, String::from(target_player.as_name()), hand)
        };

        let mut batch = WriteBatch::new();
        batch.put(key_session.as_bytes(), &::serde_json::to_vec(&session)?);
        let _ = bhv.db.write(batch, false)?;

        let s0 = format!(
            "{}{} give {} -> {} in {} ({}).",
            label,
            bhv.user.as_author_name(),
            player_name,
            target_player_name,
            session.as_title(),
            session.as_uuid(),
        );
        Ok(bhv.session_send_whisper_mine(
            &label,
            &session,
            s0.as_str(),
            s0.as_str(),
            format!(
                "{}{} give {} -> {} in {} ({}).\n{}",
                label,
                bhv.user.as_author_name(),
                player_name,
                target_player_name,
                session.as_title(),
                session.as_uuid(),
                hand
            ),
        ))
    }
}
