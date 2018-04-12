// -*- mode:rust; coding:utf-8-unix; -*-

//! receiver.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/27
//  @date 2018/01/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
use std::sync::mpsc::TryRecvError;
// ----------------------------------------------------------------------------
use discord::Connection;
// ----------------------------------------------------------------------------
use super::super::{ReceiverImpl, Recv};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
type ResultEvent = ::discord::Result<::discord::model::Event>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
struct RecvImpl {
    connection: Connection,
}
// ============================================================================
impl RecvImpl {
    // ========================================================================
    /// new
    pub(crate) fn new(connection: Connection) -> Self {
        RecvImpl {
            connection: connection,
        }
    }
}
// ============================================================================
impl Recv for RecvImpl {
    // ========================================================================
    type Generated = ResultEvent;
    type Return = ();
    // ========================================================================
    /// gen
    fn gen(&mut self) -> Self::Generated {
        self.connection.recv_event()
    }
    // ========================================================================
    /// exit
    fn exit(self) -> Self::Return {
        let _ = self.connection.shutdown();
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Receiver
#[derive(Debug)]
pub(crate) struct Receiver {
    /// receiver_impl
    receiver_impl: ReceiverImpl<ResultEvent, ()>,
}
// ============================================================================
impl Receiver {
    // ========================================================================
    /// new
    pub(crate) fn new(connection: Connection) -> Self {
        Receiver {
            receiver_impl: ReceiverImpl::from_recv(RecvImpl::new(connection)),
        }
    }
    // ========================================================================
    /// try_recv
    pub(crate) fn try_recv(&self) -> StdResult<ResultEvent, TryRecvError> {
        self.receiver_impl.try_recv()
    }
    // ========================================================================
    /// disconnect
    pub(crate) fn disconnect(
        mut self,
        discord: &::discord::Discord,
        state: &::discord::State,
    ) {
        self.receiver_impl.store_active(false);
        'cast: for ref s in state.servers() {
            for c in &s.channels {
                // send event for recv_event.
                let _ = discord.broadcast_typing(c.id);
                break 'cast; // one shot
            }
        }
        let _ = self.try_recv();
        // let _ = self.receiver_impl.join();
    }
}
