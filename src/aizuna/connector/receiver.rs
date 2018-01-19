// -*- mode:rust; coding:utf-8-unix; -*-

//! receiver.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/12/31
//  @date 2018/01/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::sync::Arc;
use std::result::Result as StdResult;
use std::sync::atomic::{AtomicBool, Ordering};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Recv
pub trait Recv {
    // ========================================================================
    /// type Generated
    type Generated: Send + 'static;
    // ------------------------------------------------------------------------
    /// type Return
    type Return: Send + 'static;
    // ========================================================================
    /// fn gen
    fn gen(&mut self) -> Self::Generated;
    // ========================================================================
    /// fn exit
    fn exit(self) -> Self::Return;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct RecvImpl
#[derive(Debug)]
struct RecvImpl<T, R, F0, F1>
where
    T: Send + 'static,
    R: Send + 'static,
    F0: FnMut() -> T + Send + 'static,
    F1: FnOnce() -> R + Send + 'static,
{
    on_gen: F0,
    on_exit: F1,
}
// ============================================================================
impl<T, R, F0, F1> RecvImpl<T, R, F0, F1>
where
    T: Send + 'static,
    R: Send + 'static,
    F0: FnMut() -> T + Send + 'static,
    F1: FnOnce() -> R + Send + 'static,
{
    // ========================================================================
    /// fn new
    pub fn new(gen: F0, exit: F1) -> Self {
        RecvImpl {
            on_gen: gen,
            on_exit: exit,
        }
    }
}
// ============================================================================
impl<T, R, F0, F1> Recv for RecvImpl<T, R, F0, F1>
where
    T: Send + 'static,
    R: Send + 'static,
    F0: FnMut() -> T + Send + 'static,
    F1: FnOnce() -> R + Send + 'static,
{
    // ========================================================================
    type Generated = T;
    type Return = R;
    // ========================================================================
    /// fn gen
    fn gen(&mut self) -> Self::Generated {
        (self.on_gen)()
    }
    // ========================================================================
    /// fn exit
    fn exit(self) -> Self::Return {
        (self.on_exit)()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ReceiverImpl
#[derive(Debug)]
pub struct ReceiverImpl<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    /// is_active
    is_active: Arc<AtomicBool>,
    /// receiver
    receiver: ::std::sync::mpsc::Receiver<T>,
    /// handle
    handle: ::std::thread::JoinHandle<R>,
}
// ============================================================================
impl<T, R> ReceiverImpl<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    // ========================================================================
    /// fn from_recv
    pub fn from_recv<I>(mut recv: I) -> Self
    where
        I: Recv<Generated = T, Return = R> + Send + 'static,
    {
        let is_active = Arc::new(AtomicBool::new(true));
        let (sender, receiver) = ::std::sync::mpsc::channel();
        ReceiverImpl {
            is_active: is_active.clone(),
            receiver: receiver,
            handle: ::std::thread::spawn(move || {
                while is_active.load(Ordering::SeqCst) {
                    if let Err(_) = sender.send(recv.gen()) {
                        break;
                    }
                    ::std::thread::yield_now();
                }
                recv.exit()
            }),
        }
    }
    // ========================================================================
    /// fn from_gen_exit
    pub fn from_gen_exit<F0, F1>(gen: F0, exit: F1) -> Self
    where
        F0: FnMut() -> T + Send + 'static,
        F1: FnOnce() -> R + Send + 'static,
    {
        ReceiverImpl::from_recv(RecvImpl::<T, R, F0, F1>::new(gen, exit))
    }
    // ========================================================================
    /// fn from_gen
    pub fn from_gen<F0>(gen: F0) -> ReceiverImpl<T, ()>
    where
        F0: FnMut() -> T + Send + 'static,
    {
        ReceiverImpl::from_gen_exit(gen, move || ())
    }
    // ========================================================================
    /// fn try_recv
    pub fn try_recv(&self) -> StdResult<T, ::std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
    // ========================================================================
    /// fn store_active
    pub fn store_active(&mut self, is_active: bool) {
        self.is_active.store(is_active, Ordering::SeqCst);
    }
    // ========================================================================
    /// fn join
    pub fn join(self) -> ::std::thread::Result<R> {
        self.handle.join()
    }
}
