//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::VecDeque;
use crate::client_context::*;
use crate::queue_context::*;
use crate::window_context::*;

pub struct CallbackQueue
{
    callbacks: VecDeque<Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>>,
}

impl CallbackQueue
{
    pub(crate) fn new() -> Self
    { CallbackQueue { callbacks: VecDeque::new(), } }
    
    pub fn is_empty(&self) -> bool
    { self.callbacks.is_empty() }
    
    pub fn push_dyn(&mut self, f: Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>)
    { self.callbacks.push_back(f); }
    
    pub fn push<F>(&mut self, f: F)
        where F: FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static
    { self.push_dyn(Box::new(f)); }
    
    pub(crate) fn pop(&mut self) -> Option<Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>>
    { self.callbacks.pop_front() }
}
