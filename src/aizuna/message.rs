// -*- mode:rust; coding:utf-8-unix; -*-

//! message.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/01/10
//  @date 2018/04/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::fmt::Debug;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[allow(unreachable_pub)]
aelicit_define!(aelicit_message, Message);
pub(crate) use self::aelicit_message::Aelicit as MessageAelicit;
pub(crate) use self::aelicit_message::EnableAelicitFromSelf as MessageEAFS;
pub(crate) use self::aelicit_message::EnableAelicitFromSelfField as MessageEAFSField;
//pub(crate) use self::aelicit_message::WeakAelicit as MessageWeakAelicit;
// ----------------------------------------------------------------------------
/// trait Message
pub trait Message: Debug + Send + Sync + MessageEAFS {
    // ========================================================================
    /// as_any
    fn as_any(&self) -> &::std::any::Any;
    // ========================================================================
    /// as_connector_type
    fn as_connector_type(&self) -> &str;
    // ========================================================================
    /// as_connector_id
    fn as_connector_id(&self) -> &str;
    // ========================================================================
    /// as_author_id
    fn as_author_id(&self) -> &str;
    // ========================================================================
    /// as_author_name
    fn as_author_name(&self) -> &str;
    // ========================================================================
    /// as_author_mention
    fn as_author_mention(&self) -> &str;
    // ========================================================================
    /// as_channel_id
    fn as_channel_id(&self) -> &str;
    // ========================================================================
    /// as_content
    fn as_content(&self) -> &str;
}
