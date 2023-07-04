//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::VecDeque;
use crate::client_context::*;
use crate::queue_context::*;
use crate::window_context::*;

/// A structure of callback queue.
///
/// The callback queue contains the callabcks. The callbacks are used to get access to the window
/// context in event handler, because event handler hasn't access to this context. The callbacks in
/// the callback queue are popped and called when a wayland event is called or other thread sends a
/// thread signal to a graphic thread. The callback queue empties after an event queue.
pub struct CallbackQueue
{
    callbacks: VecDeque<Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>>,
}

impl CallbackQueue
{
    pub(crate) fn new() -> Self
    { CallbackQueue { callbacks: VecDeque::new(), } }
    
    /// Returns `true` if the callback queue is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool
    { self.callbacks.is_empty() }
    
    /// Pushes a dynamic callback to the callback queue.
    pub fn push_dyn(&mut self, f: Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>)
    { self.callbacks.push_back(f); }
    
    /// Pushes a callback to the callback queue.
    pub fn push<F>(&mut self, f: F)
        where F: FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static
    { self.push_dyn(Box::new(f)); }
    
    pub(crate) fn pop(&mut self) -> Option<Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>>
    { self.callbacks.pop_front() }
}
