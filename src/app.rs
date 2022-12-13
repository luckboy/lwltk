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
use crate::client_context::*;
use crate::client_error::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::thread_signal::*;
use crate::window_context::*;

pub struct App<T>
{
    client_display: ClientDisplay,
    client_context: Rc<RefCell<ClientContext>>,
    window_context: Arc<RwLock<WindowContext>>,
    queue_context: Arc<Mutex<QueueContext>>,
    thread_signal_sender: ThreadSignalSender,
    thread_signal_receiver: ThreadSignalReceiver,
    data: Arc<RwLock<T>>,
}

impl<T> App<T>
{
    pub fn new<F, G>(creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext) -> Option<T>,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>) -> Option<()>
    { Self::new_with_dyn_theme(theme_from_env()?, creating_f, setting_f) }

    pub fn new_with_theme<U: Theme + 'static, F, G>(theme: U, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext) -> Option<T>,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>) -> Option<()>
    { Self::new_with_dyn_theme(Box::new(theme), creating_f, setting_f) }
    
    pub fn new_with_dyn_theme<F, G>(theme: Box<dyn Theme>, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext) -> Option<T>,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>) -> Option<()>
    {
        let mut window_context = WindowContext::new(theme);
        match creating_f(&mut window_context) {
            Some(tmp_data) => {
                let data = Arc::new(RwLock::new(tmp_data));
                match setting_f(&mut window_context, data.clone()) {
                    Some(()) => {
                        let (client_display, client_context) = ClientContext::new()?;
                        let (thread_signal_sender, thread_signal_receiver) = thread_signal_channel()?;
                        let app = App {
                            client_display,
                            client_context: Rc::new(RefCell::new(client_context)),
                            window_context: Arc::new(RwLock::new(window_context)),
                            queue_context: Arc::new(Mutex::new(QueueContext::new())),
                            thread_signal_sender,
                            thread_signal_receiver,
                            data,
                        };
                        Ok(app)
                    },
                    None => Err(ClientError::Data),
                }
            },
            None => Err(ClientError::Data),
        }
    }

    pub fn client_context(&self) -> Rc<RefCell<ClientContext>>
    { self.client_context.clone() }

    pub fn window_context(&self) -> Arc<RwLock<WindowContext>>
    { self.window_context.clone() }

    pub fn queue_context(&self) -> Arc<Mutex<QueueContext>>
    { self.queue_context.clone() }

    pub fn thread_signal_sender(&self) -> ThreadSignalSender
    { self.thread_signal_sender }
    
    pub fn data(&self) -> Arc<RwLock<T>>
    { self.data.clone() }
    
    pub fn run(&mut self) -> Result<(), ClientError>
    { run_main_loop(&mut self.client_display, self.client_context.clone(), self.window_context.clone(), self.queue_context.clone(), self.thread_signal_sender, self.thread_signal_receiver) }
}
