//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CallOnId
{
    Pointer,
    Touch(i32),
}

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
    pub(crate) active_counts: BTreeMap<CallOnPath, usize>,
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
            active_counts: BTreeMap::new(),
            has_wait_cursor: false,
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
    
    pub fn current_descendant_index_pairs(&self) -> QueueContextIter<'_>
    { QueueContextIter::new(self.current_descendant_index_pairs.as_slice()) }
    
    pub fn motion_call_on_path(&self, call_on_id: CallOnId) -> Option<&CallOnPath>
    { self.motion_call_on_paths.get(&call_on_id) }

    pub fn set_motion_call_on_path(&mut self, call_on_id: CallOnId, call_on_path: CallOnPath)
    { self.motion_call_on_paths.insert(call_on_id, call_on_path); }

    pub fn unset_motion_call_on_path(&mut self, call_on_id: CallOnId)
    { self.motion_call_on_paths.remove(&call_on_id); }

    pub fn motion_resize_edges(&self, call_on_id: CallOnId) -> Option<ClientResize>
    {
        match self.motion_resize_edge_map.get(&call_on_id) {
            Some(edges) => Some(*edges),
            None => None,
        }
    }
    
    pub fn set_motion_resize_edges(&mut self, call_on_id: CallOnId, edges: ClientResize)
    { self.motion_resize_edge_map.insert(call_on_id, edges); }
    
    pub fn unset_motion_resize_edges(&mut self, call_on_id: CallOnId)
    { self.motion_resize_edge_map.remove(&call_on_id); }
    
    pub fn pressed_call_on_path(&self, call_on_id: CallOnId) -> Option<&CallOnPath>
    { self.pressed_call_on_paths.get(&call_on_id) }

    pub fn set_pressed_call_on_path(&mut self, call_on_id: CallOnId, call_on_path: CallOnPath)
    { self.pressed_call_on_paths.insert(call_on_id, call_on_path); }

    pub fn unset_pressed_call_on_path(&mut self, call_on_id: CallOnId)
    { self.pressed_call_on_paths.remove(&call_on_id); }
    
    pub fn pressed_instant(&self, call_on_id: CallOnId) -> Option<&Instant>
    { self.pressed_instants.get(&call_on_id) }

    pub fn set_pressed_instant(&mut self, call_on_id: CallOnId, instant: Instant)
    { self.pressed_instants.insert(call_on_id, instant); }

    pub fn unset_pressed_instant(&mut self, call_on_id: CallOnId)
    { self.pressed_instants.remove(&call_on_id); }

    pub fn has_double_click(&self) -> bool
    { self.has_double_click }

    pub fn set_double_click(&mut self, flag: bool)
    { self.has_double_click = flag }
    
    pub fn has_long_click(&self) -> bool
    { self.has_long_click }

    pub fn set_long_click(&mut self, flag: bool)
    { self.has_long_click = flag }
    
    pub fn increase_active_count(&mut self, call_on_path: &CallOnPath) -> bool
    {
        match self.active_counts.get_mut(call_on_path) {
            Some(count) => {
                *count += 1;
                false
            },
            None => {
                self.active_counts.insert(call_on_path.clone(), 1);
                true
            },
        }
    }

    pub fn decrease_active_count(&mut self, call_on_path: &CallOnPath) -> bool
    {
        match self.active_counts.get_mut(call_on_path) {
            Some(count) => {
                *count -= 1;
                if *count <= 0 {
                    self.active_counts.remove(call_on_path);
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
            let active_call_on_paths: Vec<CallOnPath> = self.active_counts.keys().filter(|k| {
                    client_windows_to_destroy.keys().any(|i| *i == k.window_index()) 
            }).map(|k| k.clone()).collect();
            for call_on_path in &active_call_on_paths {
                self.active_counts.remove(call_on_path);
            }
        }
    }

    pub fn has_wait_cursor(&self) -> bool
    { self.has_wait_cursor }

    pub fn set_wait_cursor(&mut self, flag: bool)
    { self.has_wait_cursor = flag }
    
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
