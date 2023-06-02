//
// Copyright (c) 2022-2023 Łukasz Szpakowski
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
        where F: FnOnce(&mut WindowContext, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>) -> Option<T>,
              G: FnOnce(&mut WindowContext, &mut T, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, Arc<RwLock<T>>) -> Option<()>
    { Self::new_with_dyn_theme(theme_from_env()?, creating_f, setting_f) }

    pub fn new_with_theme<U: Theme + 'static, F, G>(theme: U, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>) -> Option<T>,
              G: FnOnce(&mut WindowContext, &mut T, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, Arc<RwLock<T>>) -> Option<()>
    { Self::new_with_dyn_theme(Box::new(theme), creating_f, setting_f) }
    
    pub fn new_with_dyn_theme<F, G>(theme: Box<dyn Theme>, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>) -> Option<T>,
              G: FnOnce(&mut WindowContext, &mut T, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, Arc<RwLock<T>>) -> Option<()>
    {
        let window_context = Arc::new(RwLock::new(WindowContext::new(theme)));
        let window_context2 = window_context.clone();
        let queue_context2 = Arc::new(Mutex::new(QueueContext::new()));
        let res2 = match window_context.write() {
            Ok(mut window_context_g) => {
                match creating_f(&mut *window_context_g, window_context2.clone(), queue_context2.clone()) {
                    Some(tmp_data) => {
                        let data = Arc::new(RwLock::new(tmp_data));
                        let data2 = data.clone();
                        let res = match data.write() {
                            Ok(mut data_g) => {
                                match setting_f(&mut *window_context_g, &mut *data_g, window_context2.clone(), queue_context2.clone(), data2.clone()) {
                                    Some(()) => {
                                        let (client_display, client_context) = ClientContext::new()?;
                                        let (thread_signal_sender, thread_signal_receiver) = thread_signal_channel()?;
                                        let app = App {
                                            client_display,
                                            client_context: Rc::new(RefCell::new(client_context)),
                                            window_context: window_context2,
                                            queue_context: queue_context2,
                                            thread_signal_sender,
                                            thread_signal_receiver,
                                            data: data2,
                                        };
                                        Ok(app)
                                    },
                                    None => Err(ClientError::Data),
                                }
                            },
                            Err(_) => Err(ClientError::RwLock),
                        };
                        res
                    },
                    None => Err(ClientError::Data),
                }
            },
            Err(_) => Err(ClientError::RwLock),
        };
        res2
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

impl<T> Drop for App<T>
{
    fn drop(&mut self)
    {
        let _res1 = self.thread_signal_sender.close();
        let _res2 = self.thread_signal_receiver.close();
    }
}
