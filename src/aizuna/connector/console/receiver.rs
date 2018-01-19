// -*- mode:rust; coding:utf-8-unix; -*-

//! receiver.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/31
//  @date 2018/01/08

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::sync::mpsc::TryRecvError;
// ----------------------------------------------------------------------------
use super::super::ReceiverImpl;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Receiver
#[derive(Debug)]
pub struct Receiver {
    /// receiver_impl
    receiver_impl: ReceiverImpl<String, ()>,
}
// ============================================================================
impl Receiver {
    // ========================================================================
    /// new
    pub fn new() -> Self {
        Receiver {
            receiver_impl: ReceiverImpl::<String, ()>::from_gen(move || {
                /*
                {
                    use std::io::Write;
                    let out = ::std::io::stdout();
                    let mut lock = out.lock();
                    let _ = lock.write("> ".as_bytes());
                    let _ = lock.flush();
                }
                 */
                let mut i = String::default();
                let _ = ::std::io::stdin().read_line(&mut i); // BLOCK
                String::from(i.trim())
            }),
        }
    }
    // ========================================================================
    /// try_recv
    pub fn try_recv(&self) -> ::std::result::Result<String, TryRecvError> {
        self.receiver_impl.try_recv()
    }
    // ========================================================================
    /// disconnect
    pub fn disconnect(mut self) {
        self.receiver_impl.store_active(false);
        let _ = self.try_recv();
        // let _ = self.receiver.join();
    }
}
