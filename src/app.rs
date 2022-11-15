//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::*;
use crate::callback_queue::*;
use crate::client_context::*;
use crate::client_error::*;
use crate::theme::*;
use crate::thread_signal::*;
use crate::window_context::*;

pub struct App<T>
{
    client_context: Rc<RefCell<ClientContext>>,
    window_context: Arc<RwLock<WindowContext>>,
    callback_queue: Arc<Mutex<CallbackQueue>>,
    thread_signal_sender: ThreadSignalSender,
    thread_signal_receiver: ThreadSignalReceiver,
    data: Arc<RwLock<T>>,
}

impl<T> App<T>
{
    pub fn new<F, G>(creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext) -> T,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>)
    { Self::new_with_dyn_theme(theme_from_env()?, creating_f, setting_f) }

    pub fn new_with_theme<U: Theme + 'static, F, G>(theme: U, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext) -> T,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>)
    { Self::new_with_dyn_theme(Box::new(theme), creating_f, setting_f) }
    
    pub fn new_with_dyn_theme<F, G>(theme: Box<dyn Theme>, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext) -> T,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>)
    {
        let mut window_context = WindowContext::new(theme);
        let data = Arc::new(RwLock::new(creating_f(&mut window_context)));
        setting_f(&mut window_context, data.clone());
        let client_context = ClientContext::new()?;
        let (thread_signal_sender, thread_signal_receiver) = thread_signal_channel()?;
        let app = App {
            client_context: Rc::new(RefCell::new(client_context)),
            window_context: Arc::new(RwLock::new(window_context)),
            callback_queue: Arc::new(Mutex::new(CallbackQueue::new())),
            thread_signal_sender,
            thread_signal_receiver,
            data,
        };
        Ok(app)
    }

    pub fn client_context(&self) -> Rc<RefCell<ClientContext>>
    { self.client_context.clone() }

    pub fn window_context(&self) -> Arc<RwLock<WindowContext>>
    { self.window_context.clone() }

    pub fn callback_queue(&self) -> Arc<Mutex<CallbackQueue>>
    { self.callback_queue.clone() }

    pub fn thread_signal_sender(&self) -> ThreadSignalSender
    { self.thread_signal_sender }
    
    pub fn data(&self) -> Arc<RwLock<T>>
    { self.data.clone() }
    
    pub fn run(&self) -> Result<(), ClientError>
    { Ok(()) }
}
