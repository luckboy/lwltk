//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::VecDeque;
use crate::events::*;
use crate::types::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CallOnPath
{
    Window(WindowIndex),
    Widget(AbsWidgetPath),
}

impl CallOnPath
{
    pub fn window_index(&self) -> WindowIndex
    {
        match self {
            CallOnPath::Window(window_idx) => *window_idx,
            CallOnPath::Widget(abs_widget_path) => abs_widget_path.window_index(),
        }
    }
}

#[derive(Clone)]
pub struct EventPair
{
    pub call_on_path: CallOnPath,
    pub event: Event,
}

impl EventPair
{
    pub fn new(call_on_path: CallOnPath, event: Event) -> EventPair
    { EventPair { call_on_path, event, } }
}

pub struct EventQueue
{
    event_pairs: VecDeque<EventPair>,
}

impl EventQueue
{
    pub(crate) fn new() -> Self
    { EventQueue { event_pairs: VecDeque::new(), } }
    
    pub fn is_empty(&self) -> bool
    { self.event_pairs.is_empty() }
    
    pub fn push(&mut self, event_pair: EventPair)
    { self.event_pairs.push_back(event_pair); }
    
    pub(crate) fn pop(&mut self) -> Option<EventPair>
    { self.event_pairs.pop_front() }
}
