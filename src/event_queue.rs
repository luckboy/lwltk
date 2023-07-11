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

/// An enumaration of call-on path.
///
/// The call-on path refers to a widget or a window.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CallOnPath
{
    /// A window index.
    Window(WindowIndex),
    /// An abslolute widget path.
    Widget(AbsWidgetPath),
}

impl CallOnPath
{
    /// Returns the window index.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::CallOnPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let call_on_path1 = CallOnPath::Window(WindowIndex(2));
    /// assert_eq!(WindowIndex(2), call_on_path1.window_index());
    ///
    /// let call_on_path2 = CallOnPath::Widget(AbsWidgetPath::new(WindowIndex(3), WidgetIndexPair(1, 2)));
    /// assert_eq!(WindowIndex(3), call_on_path2.window_index());
    /// ```
    pub fn window_index(&self) -> WindowIndex
    {
        match self {
            CallOnPath::Window(window_idx) => *window_idx,
            CallOnPath::Widget(abs_widget_path) => abs_widget_path.window_index(),
        }
    }
}

/// A structure of event pair.
#[derive(Clone)]
pub struct EventPair
{
    /// The call-on path that refers the widget or the window for an event.
    pub call_on_path: CallOnPath,
    /// The event.
    pub event: Event,
}

impl EventPair
{
    /// Creates an event pair.
    pub fn new(call_on_path: CallOnPath, event: Event) -> EventPair
    { EventPair { call_on_path, event, } }
}

/// A structure of event queue.
///
/// The event queue contains the events. The events in the event queue are popped and called when a
/// Wayland event is called or other thread sends a thread signal to a graphic thread. The event
/// queue empties before a callback queue.
pub struct EventQueue
{
    event_pairs: VecDeque<EventPair>,
}

impl EventQueue
{
    pub(crate) fn new() -> Self
    { EventQueue { event_pairs: VecDeque::new(), } }
    
    /// Returns `true` if the event queue is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool
    { self.event_pairs.is_empty() }
    
    /// Pushes an event pair to the event queue.
    pub fn push(&mut self, event_pair: EventPair)
    { self.event_pairs.push_back(event_pair); }
    
    pub(crate) fn pop(&mut self) -> Option<EventPair>
    { self.event_pairs.pop_front() }
}
