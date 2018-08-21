// -*- mode:rust; coding:utf-8-unix; -*-

//! message.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/04
//  @date 2018/08/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::any::Any as StdAny;
// ----------------------------------------------------------------------------
use elicit::enable_aelicit_from_self_delegate;
// ----------------------------------------------------------------------------
use super::super::super::{
    Message, MessageAelicit, MessageEAFS, MessageEAFSField,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// DiscordMessage
#[derive(Debug, Clone)]
pub(crate) struct DiscordMessage {
    _eafsf: MessageEAFSField,
    connector_id: String,
    channel_id: String,
    author_id: String,
    author_mention: String,
    msg: ::discord::model::Message,
}
// ============================================================================
impl MessageEAFS for DiscordMessage {
    enable_aelicit_from_self_delegate!(Message, MessageAelicit, _eafsf);
}
// ============================================================================
impl DiscordMessage {
    // ========================================================================
    /// new
    pub(crate) fn new(
        connector_id: String,
        msg: ::discord::model::Message,
    ) -> Self {
        DiscordMessage {
            _eafsf: MessageEAFSField::default(),
            connector_id,
            author_id: format!("{}", msg.author.id),
            channel_id: format!("{}", msg.channel_id),
            author_mention: msg.author.mention().to_string(),
            msg,
        }
    }
}
// ============================================================================
impl Message for DiscordMessage {
    // ========================================================================
    fn as_any(&self) -> &dyn StdAny {
        &self.msg
    }
    // ========================================================================
    fn as_connector_type(&self) -> &str {
        "discord"
    }
    // ========================================================================
    fn as_connector_id(&self) -> &str {
        &self.connector_id
    }
    // ========================================================================
    fn as_author_id(&self) -> &str {
        &self.author_id
    }
    // ========================================================================
    fn as_author_name(&self) -> &str {
        &self.msg.author.name
    }
    // ========================================================================
    fn as_author_mention(&self) -> &str {
        &self.author_mention
    }
    // ========================================================================
    fn as_channel_id(&self) -> &str {
        &self.channel_id
    }
    // ========================================================================
    fn as_content(&self) -> &str {
        self.msg.content.as_str()
    }
}
