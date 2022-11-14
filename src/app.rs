//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::rc::*;
use crate::client_context::*;
use crate::client_error::*;
use crate::window_context::*;

pub struct App<T>
{
    client_context: Rc<RefCell<ClientContext>>,
    window_context: Arc<RwLock<WindowContext>>,
    data: Arc<RwLock<T>>,
}

impl<T> App<T>
{
    pub fn new<F, G>(creating_f: F, setting_f: G) -> Result<App<T>, ClientError>
        where F: FnOnce(&mut WindowContext) -> T,
              G: FnOnce(&mut WindowContext, Arc<RwLock<T>>)
    {
        let mut window_context = WindowContext::new();
        let data = Arc::new(RwLock::new(creating_f(&mut window_context)));
        setting_f(&mut window_context, data.clone());
        let client_context = ClientContext::new()?;
        let app = App {
            client_context: Rc::new(RefCell::new(client_context)),
            window_context: Arc::new(RwLock::new(window_context)),
            data,
        };
        Ok(app)
    }

    pub fn client_context(&self) -> Rc<RefCell<ClientContext>>
    { self.client_context.clone() }

    pub fn window_context(&self) -> Arc<RwLock<WindowContext>>
    { self.window_context.clone() }

    pub fn data(&self) -> Arc<RwLock<T>>
    { self.data.clone() }
    
    pub fn run(&self) -> Result<(), ClientError>
    { Ok(()) }
}
