//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::callback_queue::*;
use crate::client_context::*;
use crate::events::*;
use crate::event_queue::*;
use crate::window_context::*;

pub struct QueueContext
{
    pub(crate) event_queue: EventQueue,
    pub(crate) callback_queue: CallbackQueue,
    pub(crate) current_call_on_path: Option<CallOnPath>,
}

impl QueueContext
{
    pub(crate) fn new() -> QueueContext
    {
        QueueContext {
            event_queue: EventQueue::new(),
            callback_queue: CallbackQueue::new(),
            current_call_on_path: None,
        }
    }

    pub fn event_queue(&self) -> &EventQueue
    { &self.event_queue }

    pub fn event_queue_mut(&mut self) -> &mut EventQueue
    { &mut self.event_queue }

    pub fn callback_queue(&self) -> &CallbackQueue
    { &self.callback_queue }

    pub fn callback_queue_mut(&mut self) -> &mut CallbackQueue
    { &mut self.callback_queue }

    pub fn current_call_on_path(&self) -> Option<&CallOnPath>
    {
        match &self.current_call_on_path {
            Some(call_on_path) => Some(call_on_path),
            None => None,
        }
    }

    pub fn push_event(&mut self, event: Event) -> Option<()>
    {
        match self.current_call_on_path.clone() {
            Some(call_on_path) => {
                self.event_queue.push(EventPair::new(call_on_path, event));
                Some(())
            },
            None => None,
        }
    }

    pub fn push_dyn_callback(&mut self, f: Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>)
    { self.callback_queue.push_dyn(f); }

    pub fn push_callback<F>(&mut self, f: F)
        where F: FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static
    { self.callback_queue.push(f); }
}
