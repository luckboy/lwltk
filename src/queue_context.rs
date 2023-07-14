//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::FusedIterator;
use std::slice::Iter;
use std::time::Instant;
use crate::callback_queue::*;
use crate::client_context::*;
use crate::client_window::*;
use crate::events::*;
use crate::event_queue::*;
use crate::types::*;
use crate::window_context::*;

/// An enumeration of call-on identifier.
///
/// The call-on identifier identifies a pointer or a touch.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CallOnId
{
    /// A pointer.
    Pointer,
    /// A touch with an unique touch identifier.
    Touch(i32),
}

/// An enumeration of active identifier.
///
/// The active identifier identifies a call-on identifier or a keyboard. This identifier is used to
/// set an active widget state.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ActiveId
{
    /// A call-on identifier.
    CallOnId(CallOnId),
    /// A keyboard.
    Keyboard,
}

/// An iterator of queue context that iterates over pairs of widget indices.
#[derive(Clone)]
pub struct QueueContextIter<'a>
{
    iter: Iter<'a, WidgetIndexPair>,
}

impl<'a> QueueContextIter<'a>
{
    fn new(slice: &'a [WidgetIndexPair]) -> Self
    { QueueContextIter { iter: slice.iter(), } }
}

impl<'a> ExactSizeIterator for QueueContextIter<'a>
{}

impl<'a> FusedIterator for QueueContextIter<'a>
{}

impl<'a> DoubleEndedIterator for QueueContextIter<'a>
{
    fn next_back(&mut self) -> Option<Self::Item>
    { self.iter.next_back().map(|ip| *ip) }
}

impl<'a> Iterator for QueueContextIter<'a>
{
    type Item = WidgetIndexPair;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.next().map(|x| *x) }
    
    fn size_hint(&self) -> (usize, Option<usize>)
    { self.iter.size_hint() }
}

/// A structure of queue context.
///
/// The queue context is used to manage events and callbacks. The queue context contains the event
/// queue and the callback queue.
pub struct QueueContext
{
    pub(crate) event_queue: EventQueue,
    pub(crate) callback_queue: CallbackQueue,
    pub(crate) current_call_on_path: Option<CallOnPath>,
    pub(crate) current_descendant_index_pairs: Vec<WidgetIndexPair>,
    pub(crate) motion_call_on_paths: BTreeMap<CallOnId, CallOnPath>,
    pub(crate) motion_resize_edge_map: BTreeMap<CallOnId, ClientResize>,
    pub(crate) pressed_call_on_paths: BTreeMap<CallOnId, CallOnPath>,
    pub(crate) pressed_instants: BTreeMap<CallOnId, Instant>,
    pub(crate) has_double_click: bool,
    pub(crate) has_long_click: bool,
    pub(crate) active_id_sets: BTreeMap<CallOnPath, BTreeSet<ActiveId>>,
    pub(crate) has_wait_cursor: bool,
}

impl QueueContext
{
    pub(crate) fn new() -> QueueContext
    {
        QueueContext {
            event_queue: EventQueue::new(),
            callback_queue: CallbackQueue::new(),
            current_call_on_path: None,
            current_descendant_index_pairs: Vec::new(),
            motion_call_on_paths: BTreeMap::new(),
            motion_resize_edge_map: BTreeMap::new(),
            pressed_call_on_paths: BTreeMap::new(),
            pressed_instants: BTreeMap::new(),
            has_double_click: false,
            has_long_click: false,
            active_id_sets: BTreeMap::new(),
            has_wait_cursor: false,
        }
    }

    /// Returns a reference to the event queue.
    pub fn event_queue(&self) -> &EventQueue
    { &self.event_queue }

    /// Returns a mutable reference to the event queue.
    pub fn event_queue_mut(&mut self) -> &mut EventQueue
    { &mut self.event_queue }

    /// Returns a reference to the callback queue.
    pub fn callback_queue(&self) -> &CallbackQueue
    { &self.callback_queue }

    /// Returns a mutable reference to the callback queue.
    pub fn callback_queue_mut(&mut self) -> &mut CallbackQueue
    { &mut self.callback_queue }

    /// Returns a reference to the current call-on path or `None`.
    ///
    /// The current call-on path refers the widget or the window for an event that is called.
    pub fn current_call_on_path(&self) -> Option<&CallOnPath>
    {
        match &self.current_call_on_path {
            Some(call_on_path) => Some(call_on_path),
            None => None,
        }
    }
    
    /// Returns an iterator that iterates over the current  pairs of the indices of the descendant
    /// widgets.
    ///
    /// The current pairs of the indices of the descendant widgets are arranged in order from
    /// great-...-great-grandchild to child. These pairs of widget indices are set by an event
    /// propagation.
    pub fn current_descendant_index_pairs(&self) -> QueueContextIter<'_>
    { QueueContextIter::new(self.current_descendant_index_pairs.as_slice()) }
    
    /// Returns the motion call-on path for the specified call-on identifier or `None`.
    ///
    /// The motion call-on path refers to the widget or the window that is pointed by the pointer or
    /// the touch.
    pub fn motion_call_on_path(&self, call_on_id: CallOnId) -> Option<&CallOnPath>
    { self.motion_call_on_paths.get(&call_on_id) }

    /// Sets a call-on path for the specified call-on identifier.
    ///
    /// See [`motion_call_on_path`](Self::motion_call_on_path) for more informations.
    pub fn set_motion_call_on_path(&mut self, call_on_id: CallOnId, call_on_path: CallOnPath)
    { self.motion_call_on_paths.insert(call_on_id, call_on_path); }

    /// Unsets a call-on path for the specified call-on identifier.
    ///
    /// See [`motion_call_on_path`](Self::motion_call_on_path) for more informations.
    pub fn unset_motion_call_on_path(&mut self, call_on_id: CallOnId)
    { self.motion_call_on_paths.remove(&call_on_id); }

    /// Returns the motion resize edges for the call-on identifier or `None`.
    ///
    /// The motion resize edges is used to resize a window.
    pub fn motion_resize_edges(&self, call_on_id: CallOnId) -> Option<ClientResize>
    {
        match self.motion_resize_edge_map.get(&call_on_id) {
            Some(edges) => Some(*edges),
            None => None,
        }
    }
 
    /// Sets a motion resize edges for the specified call-on identifier.
    ///
    /// See [`motion_resize_edges`](Self::motion_resize_edges) for more informations.
    pub fn set_motion_resize_edges(&mut self, call_on_id: CallOnId, edges: ClientResize)
    { self.motion_resize_edge_map.insert(call_on_id, edges); }
    
    /// Unsets the motion resize edges for the specified call-on identifier.
    ///
    /// See [`motion_resize_edges`](Self::motion_resize_edges) for more informations.
    pub fn unset_motion_resize_edges(&mut self, call_on_id: CallOnId)
    { self.motion_resize_edge_map.remove(&call_on_id); }
    
    /// Returns the call-on path of the pressed button for the specified call-on identifier or
    /// `None`.
    ///
    /// The call-on path of the pressed button refers to the widget or the window that is pressed by
    /// the pointer or touched.
    pub fn pressed_call_on_path(&self, call_on_id: CallOnId) -> Option<&CallOnPath>
    { self.pressed_call_on_paths.get(&call_on_id) }

    /// Sets a call-on path of the pressed button for the specified call-on identifier.
    ///
    /// See [`pressed_call_on_path`](Self::pressed_call_on_path) for more informations.
    pub fn set_pressed_call_on_path(&mut self, call_on_id: CallOnId, call_on_path: CallOnPath)
    { self.pressed_call_on_paths.insert(call_on_id, call_on_path); }

    /// Sets the call-on path of the pressed button for the specified call-on identifier.
    ///
    /// See [`pressed_call_on_path`](Self::pressed_call_on_path) for more informations.
    pub fn unset_pressed_call_on_path(&mut self, call_on_id: CallOnId)
    { self.pressed_call_on_paths.remove(&call_on_id); }
    
    /// Returns the clock measurement of the pressed button for the specified call-on identifier
    /// or `None`.
    ///
    /// The clock measurement of the pressed button is made when the widget or the window that is
    /// pressed by the pointer or touched.
    pub fn pressed_instant(&self, call_on_id: CallOnId) -> Option<&Instant>
    { self.pressed_instants.get(&call_on_id) }

    /// Sets a clock measurement of the pressed button for the specified call-on identifier.
    ///
    /// See [`pressed_instant`](Self::pressed_instant) for more informations.
    pub fn set_pressed_instant(&mut self, call_on_id: CallOnId, instant: Instant)
    { self.pressed_instants.insert(call_on_id, instant); }

    /// Unsets a clock measurement of the pressed button for the specified call-on identifier.
    ///
    /// See [`pressed_instant`](Self::pressed_instant) for more informations.
    pub fn unset_pressed_instant(&mut self, call_on_id: CallOnId)
    { self.pressed_instants.remove(&call_on_id); }

    /// Returns `true` if a double click occurred by the pointer, otherwise `false`.
    pub fn has_double_click(&self) -> bool
    { self.has_double_click }

    /// Sets the double click flag for the pointer.
    pub fn set_double_click(&mut self, flag: bool)
    { self.has_double_click = flag }
    
    /// Returns `true` if a long click occurred by the pointer, otherwise `false`.
    pub fn has_long_click(&self) -> bool
    { self.has_long_click }

    /// Sets the long click flag for the pointer.
    pub fn set_long_click(&mut self, flag: bool)
    { self.has_long_click = flag }
    
    /// Adds an active identifier for the specified call-on path.
    ///
    /// This method returns `true` if a new set of active identidfiers is created while add the
    /// active identifier, otherwise `false`. The set of active identifier contains the active
    /// identifiers for the widget or the window.
    pub fn add_active_id(&mut self, call_on_path: &CallOnPath, active_id: ActiveId) -> bool
    {
        match self.active_id_sets.get_mut(call_on_path) {
            Some(set) => {
                set.insert(active_id);
                false
            },
            None => {
                let mut set: BTreeSet<ActiveId> = BTreeSet::new();
                set.insert(active_id);
                self.active_id_sets.insert(call_on_path.clone(), set);
                true
            },
        }
    }

    /// Removes the active identifier for the specified call-on path.
    ///
    /// This method returns `true` if the set of active identifiers is removed while remove the
    /// active identifier, otherwise `false`. The set of active identifier contains the active
    /// identifiers for the widget or the window.
    pub fn remove_active_id(&mut self, call_on_path: &CallOnPath, active_id: ActiveId) -> bool
    {
        match self.active_id_sets.get_mut(call_on_path) {
            Some(set) => {
                set.remove(&active_id);
                if set.is_empty() {
                    self.active_id_sets.remove(call_on_path);
                    true
                } else {
                    false
                }
            },
            None => true,
        }
    }
    
    pub(crate) fn clear_for_client_windows_to_destroy(&mut self, client_windows_to_destroy: &BTreeMap<WindowIndex, Box<ClientWindow>>)
    {
        if !client_windows_to_destroy.is_empty() {
            let motion_call_on_ids: Vec<CallOnId> = self.motion_call_on_paths.iter().filter(|p| {
                    client_windows_to_destroy.keys().any(|i| *i == p.1.window_index())
            }).map(|p| *(p.0)).collect();
            for call_on_id in &motion_call_on_ids {
                self.motion_call_on_paths.remove(call_on_id);
            }
            for call_on_id in &motion_call_on_ids {
                self.motion_resize_edge_map.remove(call_on_id);
            }
            let pressed_call_on_ids: Vec<CallOnId> = self.pressed_call_on_paths.iter().filter(|p| {
                    client_windows_to_destroy.keys().any(|i| *i == p.1.window_index())
            }).map(|p| *(p.0)).collect();
            for call_on_id in &pressed_call_on_ids {
                self.pressed_call_on_paths.remove(call_on_id);
            }
            for call_on_id in &pressed_call_on_ids {
                self.pressed_instants.remove(call_on_id);
            }
            let active_call_on_paths: Vec<CallOnPath> = self.active_id_sets.keys().filter(|k| {
                    client_windows_to_destroy.keys().any(|i| *i == k.window_index()) 
            }).map(|k| k.clone()).collect();
            for call_on_path in &active_call_on_paths {
                self.active_id_sets.remove(call_on_path);
            }
        }
    }

    /// Returns `true` if the wait cursor is set, otherwise `false`.
    pub fn has_wait_cursor(&self) -> bool
    { self.has_wait_cursor }

    /// Sets the flag of the wait cursor.
    pub fn set_wait_cursor(&mut self, flag: bool)
    { self.has_wait_cursor = flag }
    
    /// Pushes event to the event queue for the current call-on path.
    ///
    /// This method returns `Some(())` if the current call-on path exists, otherwise `None`.
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

    /// See [`CallbackQueue::push_dyn`].
    pub fn push_dyn_callback(&mut self, f: Box<dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static>)
    { self.callback_queue.push_dyn(f); }

    /// See [`CallbackQueue::push`].
    pub fn push_callback<F>(&mut self, f: F)
        where F: FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static
    { self.callback_queue.push(f); }
}
