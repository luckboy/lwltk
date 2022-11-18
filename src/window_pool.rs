//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::cmp::Ordering;
use std::collections::hash_map;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::FusedIterator;
use crate::container::*;
use crate::types::*;
use crate::widget::*;
use crate::window::*;

#[derive(Copy, Clone)]
struct IndexRange
{
    min: usize,
    max: usize,
}

impl IndexRange
{
    fn new(min: usize, max: usize) -> IndexRange
    { IndexRange { min, max, } }
}

impl Ord for IndexRange
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        if self.max < other.min {
            Ordering::Less
        } else if self.min > other.max {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Eq for IndexRange
{}

impl PartialEq for IndexRange
{
    fn eq(&self, other: &Self) -> bool
    { self.cmp(other) == Ordering::Equal }
}

impl PartialOrd for IndexRange
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    { Some(self.cmp(other)) }
}

#[derive(Clone)]
pub struct WindowIndices<'a>
{
    iter: hash_map::Keys<'a, WindowIndex, Box<dyn Window + 'static>>,
}

impl<'a> WindowIndices<'a>
{
    fn new(windows: &'a HashMap<WindowIndex, Box<dyn Window + 'static>>) -> Self
    { WindowIndices { iter: windows.keys(), } }
}

impl<'a> ExactSizeIterator for WindowIndices<'a>
{}

impl<'a> FusedIterator for WindowIndices<'a>
{}

impl<'a> Iterator for WindowIndices<'a>
{
    type Item = WindowIndex;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.next().map(|i| *i) }
    
    fn size_hint(&self) -> (usize, Option<usize>)
    { self.iter.size_hint() }
}

#[derive(Clone)]
pub struct Windows<'a>
{
    iter: hash_map::Values<'a, WindowIndex, Box<dyn Window + 'static>>,
}

impl<'a> Windows<'a>
{
    fn new(windows: &'a HashMap<WindowIndex, Box<dyn Window + 'static>>) -> Self
    { Windows { iter: windows.values(), } }
}

impl<'a> ExactSizeIterator for Windows<'a>
{}

impl<'a> FusedIterator for Windows<'a>
{}

impl<'a> Iterator for Windows<'a>
{
    type Item = &'a dyn Window;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.next().map(|i| &**i) }
    
    fn size_hint(&self) -> (usize, Option<usize>)
    { self.iter.size_hint() }
}

pub struct WindowPool
{
    windows: HashMap<WindowIndex, Box<dyn Window>>,
    free_indices: BTreeSet<IndexRange>,
    index_counter: Option<usize>,
    indices_to_destroy: BTreeSet<WindowIndex>,
}

impl WindowPool
{
    pub(crate) fn new() -> WindowPool
    {
        WindowPool {
            windows: HashMap::new(),
            free_indices: BTreeSet::new(),
            index_counter: None,
            indices_to_destroy: BTreeSet::new(),
        }
    }
    
    pub fn add_dyn(&mut self, window: Box<dyn Window>) -> Option<WindowIndex>
    {
        match self.free_indices.iter().next().map(|ir| *ir) {
            Some(idx_range) => {
                self.free_indices.remove(&idx_range);
                if idx_range.min < idx_range.max {
                    let new_idx_range = IndexRange::new(idx_range.min + 1, idx_range.max);
                    self.free_indices.insert(idx_range);
                }
                let new_idx = WindowIndex(idx_range.min);
                self.windows.insert(new_idx, window);
                Some(new_idx)
            },
            None => {
                match self.index_counter {
                    Some(idx_counter) => {
                        match idx_counter.checked_add(1) {
                            Some(i) => { 
                                let new_idx = WindowIndex(i);
                                self.windows.insert(new_idx, window);
                                self.index_counter = Some(new_idx.0);
                                Some(new_idx)
                            },
                            None => None,
                        }
                    },
                    None => {
                        let new_idx = WindowIndex(0);
                        self.windows.insert(new_idx, window);
                        self.index_counter = Some(new_idx.0);
                        Some(new_idx)
                    },
                }
            },
        }
    }
    
    pub fn add<T: Window + 'static>(&mut self, window: T) -> Option<WindowIndex>
    { self.add_dyn(Box::new(window)) }
    
    pub fn remove(&mut self, idx: WindowIndex) -> Option<Box<dyn Window>>
    {
        match self.windows.remove(&idx) {
            Some(window) => {
                if self.index_counter.map(|ic| ic != idx.0).unwrap_or(false) {
                    let idx_range1 = match idx.0.checked_sub(1) {
                        Some(i) => self.free_indices.get(&IndexRange::new(i, i)).map(|ir| *ir),
                        None => None,
                    };
                    let idx_range2 = match idx.0.checked_add(1) {
                        Some(i) => self.free_indices.get(&IndexRange::new(i, i)).map(|ir| *ir),
                        None => None,
                    };
                    match (idx_range1, idx_range2) {
                        (Some(idx_range1), Some(idx_range2)) => {
                            self.free_indices.remove(&idx_range1);
                            self.free_indices.remove(&idx_range2);
                            self.free_indices.insert(IndexRange::new(idx_range1.min, idx_range2.max));
                        },
                        (Some(idx_range1), None) => {
                            self.free_indices.remove(&idx_range1);
                            self.free_indices.insert(IndexRange::new(idx_range1.min, idx.0));
                        },
                        (None, Some(idx_range2)) => {
                            self.free_indices.remove(&idx_range2);
                            self.free_indices.insert(IndexRange::new(idx.0, idx_range2.max));
                        },
                        (None, None) => {
                            self.free_indices.insert(IndexRange::new(idx.0, idx.0));
                        },
                    }
                } else {
                    let idx_range = match idx.0.checked_sub(1) {
                        Some(i) => self.free_indices.get(&IndexRange::new(i, i)).map(|ir| *ir),
                        None => None,
                    };
                    match idx_range {
                        Some(idx_range) => {
                            self.free_indices.remove(&idx_range);
                            self.index_counter = idx_range.min.checked_sub(1);
                        },
                        None => (),
                    }
                }
                for child_idx in window.child_indices() {
                    self.unset_parent_window(child_idx);
                }
                self.indices_to_destroy.insert(idx);
                Some(window)
            },
            None => None,
        }
    }
    
    pub(crate) fn indices_to_destroy(&self) -> &BTreeSet<WindowIndex>
    { &self.indices_to_destroy }

    pub(crate) fn clear_indices_to_destroy(&mut self)
    { self.indices_to_destroy.clear(); }

    pub fn dyn_window(&self, idx: WindowIndex) -> Option<&dyn Window>
    {
        match self.windows.get(&idx) {
            Some(window) => Some(&**window),
            None => None,
        }
    }

    pub fn dyn_window_mut(&mut self, idx: WindowIndex) -> Option<&mut dyn Window>
    { 
        match self.windows.get_mut(&idx) {
            Some(window) => Some(&mut **window),
            None => None,
        }
    }

    pub fn window<T: Any>(&self, idx: WindowIndex) -> Option<&T>
    { self.dyn_window(idx).map(|w| w.as_any().downcast_ref::<T>()).flatten() }

    pub fn window_mut<T: Any>(&mut self, idx: WindowIndex) -> Option<&mut T>
    { self.dyn_window_mut(idx).map(|w| w.as_any_mut().downcast_mut::<T>()).flatten() }
    
    pub fn window_indices(&self) -> WindowIndices
    { WindowIndices::new(&self.windows) }

    pub fn dyn_windows(&self) -> Windows
    { Windows::new(&self.windows) }

    pub fn add_widget1<C: Container + Any, F>(&mut self, idx: WindowIndex, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
    {
        match self.window_mut(idx) {
            Some(window) => {
                match add_container_widget1(window, f) {
                    Some(rel_path) => Some(rel_path.to_abs_widget_path(idx)),
                    None => None,
                }
            },
            None => None,
        }
    }
    
    pub fn dyn_widget(&self, path: &AbsWidgetPath) -> Option<&dyn Widget>
    { self.dyn_window(path.window_index()).map(|w| w.dyn_widget(path.as_rel_widget_path())).flatten() }

    pub fn dyn_widget_mut(&mut self, path: &AbsWidgetPath) -> Option<&mut dyn Widget>
    { self.dyn_window_mut(path.window_index()).map(|w| w.dyn_widget_mut(path.as_rel_widget_path())).flatten() }

    pub fn widget<T: Any>(&self, path: &AbsWidgetPath) -> Option<&T>
    { self.dyn_window(path.window_index()).map(|w| container_widget(w, path.as_rel_widget_path())).flatten() }

    pub fn widget_mut<T: Any>(&mut self, path: &AbsWidgetPath) -> Option<&mut T>
    { self.dyn_window_mut(path.window_index()).map(|w| container_widget_mut(w, path.as_rel_widget_path())).flatten() }
    
    pub fn add_widget<T: Any, F>(&mut self, path: &AbsWidgetPath, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
    { 
        match self.dyn_window_mut(path.window_index()) {
            Some(window) => {
                match add_container_widget(window, path.as_rel_widget_path(), f) {
                    Some(rel_path) => Some(rel_path.to_abs_widget_path(path.window_index())),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn set_parent_window(&mut self, child_idx: WindowIndex, parent_idx: WindowIndex) -> bool
    {
        match self.dyn_window_mut(child_idx) {
            Some(child_window) => {
                if !child_window.set_parent(Some(ParentWindowIndex::new(parent_idx))) {
                    return false;
                }
            },
            None => return false,
        }
        let is_success = match self.dyn_window_mut(parent_idx) {
            Some(parent_window) => parent_window.add_child(ChildWindowIndex::new(parent_idx)),
            None => false,
        };
        if !is_success {
            match self.dyn_window_mut(child_idx) {
                Some(child_window) => {
                    child_window.set_parent(None);
                },
                None => return false,
            }
        }
        is_success
    }

    pub fn unset_parent_window(&mut self, child_idx: WindowIndex) -> bool
    {
        let parent_idx = match self.dyn_window_mut(child_idx) {
            Some(child_window) => {
                match child_window.parent() {
                    Some(parent_idx) => {
                        if child_window.set_parent(None) {
                            Some(parent_idx)
                        } else {
                            None                            
                        }
                    },
                    None => None,
                }
            },
            None => None,
        };
        match parent_idx {
            Some(parent_idx) => {
                match self.dyn_window_mut(parent_idx) {
                    Some(parent_window) => parent_window.remove_child(ChildWindowIndex::new(child_idx)),
                    None => false
                }
            },
            None => false,
        }
    }
}
