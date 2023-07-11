//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::cmp::Ordering;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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
            Ordering::Greater
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

/// An iterator that iterates over window indices.
#[derive(Clone)]
pub struct WindowIndices<'a>
{
    iter: btree_map::Keys<'a, WindowIndex, Box<dyn Window + 'static>>,
}

impl<'a> WindowIndices<'a>
{
    fn new(windows: &'a BTreeMap<WindowIndex, Box<dyn Window + 'static>>) -> Self
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

/// An iterator that iterates over windows.
#[derive(Clone)]
pub struct Windows<'a>
{
    iter: btree_map::Values<'a, WindowIndex, Box<dyn Window + 'static>>,
}

impl<'a> Windows<'a>
{
    fn new(windows: &'a BTreeMap<WindowIndex, Box<dyn Window + 'static>>) -> Self
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
    { self.iter.next().map(|w| &**w) }
    
    fn size_hint(&self) -> (usize, Option<usize>)
    { self.iter.size_hint() }
}

/// A structure of window container.
///
/// The window container contains windows. The structure of window container allows to add windows,
/// remove the windows, have access to the wigdets, and have access to the windows.
pub struct WindowContainer
{
    windows: BTreeMap<WindowIndex, Box<dyn Window>>,
    free_indices: BTreeSet<IndexRange>,
    index_counter: Option<usize>,
    indices_to_destroy: BTreeSet<WindowIndex>,
}

impl WindowContainer
{
    pub(crate) fn new() -> WindowContainer
    {
        WindowContainer {
            windows: BTreeMap::new(),
            free_indices: BTreeSet::new(),
            index_counter: None,
            indices_to_destroy: BTreeSet::new(),
        }
    }
    
    /// Adds a dynamic window to the window container.
    ///
    /// This method is similar to the [`add`](Self::add) method but takes the dynamic window.
    pub fn add_dyn(&mut self, mut window: Box<dyn Window>) -> Option<WindowIndex>
    {
        match self.free_indices.iter().next().map(|ir| *ir) {
            Some(idx_range) => {
                self.free_indices.remove(&idx_range);
                if idx_range.min < idx_range.max {
                    let new_idx_range = IndexRange::new(idx_range.min + 1, idx_range.max);
                    self.free_indices.insert(new_idx_range);
                }
                let new_idx = WindowIndex(idx_range.min);
                window.set_index(SelfWindowIndex::new(new_idx));
                self.windows.insert(new_idx, window);
                Some(new_idx)
            },
            None => {
                match self.index_counter {
                    Some(idx_counter) => {
                        match idx_counter.checked_add(1) {
                            Some(i) => { 
                                let new_idx = WindowIndex(i);
                                window.set_index(SelfWindowIndex::new(new_idx));
                                self.windows.insert(new_idx, window);
                                self.index_counter = Some(new_idx.0);
                                Some(new_idx)
                            },
                            None => None,
                        }
                    },
                    None => {
                        let new_idx = WindowIndex(0);
                        window.set_index(SelfWindowIndex::new(new_idx));
                        self.windows.insert(new_idx, window);
                        self.index_counter = Some(new_idx.0);
                        Some(new_idx)
                    },
                }
            },
        }
    }
    
    /// Adds a window to the window container.
    ///
    /// This method returns the window index of the added window if the dynamic window is added,
    /// otherwise `None`. Also, this method sets the window index for the added window.
    pub fn add<T: Window + 'static>(&mut self, window: T) -> Option<WindowIndex>
    { self.add_dyn(Box::new(window)) }
    
    /// Removes the window that has the specified window index.
    ///
    /// This method returns the removed dynamic window. Also, this method automatically unsets the
    /// window index for the window, unsets the parent for the window, and removes the children
    /// from the window. The window are automatically removed from the parent and the parents of
    /// the children are automatically unset for the children.
    pub fn remove(&mut self, idx: WindowIndex) -> Option<Box<dyn Window>>
    {
        match self.windows.remove(&idx) {
            Some(mut window) => {
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
                        None => self.index_counter = self.index_counter.map(|ic| ic.checked_sub(1)).flatten(),
                    }
                }
                window.unset_index(SelfWindowTag::new());
                match window.parent_index() {
                    Some(parent_idx) => {
                        window.unset_parent(ParentWindowTag::new());
                        match self.dyn_window_mut(parent_idx) {
                            Some(parent) => {
                                parent.remove_child(ChildWindowIndex::new(idx));
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
                let child_indices: Vec<WindowIndex> = window.child_indices().collect();
                for child_idx in &child_indices {
                    match self.dyn_window_mut(*child_idx) {
                        Some(child) => {
                            child.unset_parent(ParentWindowTag::new());
                        },
                        None => (),
                    }
                    window.remove_child(ChildWindowIndex::new(*child_idx));
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
    
    pub(crate) fn window_map(&self) -> &BTreeMap<WindowIndex, Box<dyn Window>>
    { &self.windows }

    /// Returns a reference to the dynamic window for the specified window index or `None`.
    pub fn dyn_window(&self, idx: WindowIndex) -> Option<&dyn Window>
    {
        match self.windows.get(&idx) {
            Some(window) => Some(&**window),
            None => None,
        }
    }

    /// Returns a mutable reference to the dynamic window for the specified window index or `None`.
    pub fn dyn_window_mut(&mut self, idx: WindowIndex) -> Option<&mut dyn Window>
    { 
        match self.windows.get_mut(&idx) {
            Some(window) => Some(&mut **window),
            None => None,
        }
    }

    /// Returns a reference to the window for the specified window index or `None`.
    pub fn window<T: Any>(&self, idx: WindowIndex) -> Option<&T>
    { self.dyn_window(idx).map(|w| w.as_any().downcast_ref::<T>()).flatten() }

    /// Returns a mutable reference to the window for the specified window index or `None`.
    pub fn window_mut<T: Any>(&mut self, idx: WindowIndex) -> Option<&mut T>
    { self.dyn_window_mut(idx).map(|w| w.as_any_mut().downcast_mut::<T>()).flatten() }
    
    /// Returns an iterator that iterates over the window indicies.
    pub fn window_indices(&self) -> WindowIndices
    { WindowIndices::new(&self.windows) }

    /// Returns an iterator that iterates over the windows.
    pub fn dyn_windows(&self) -> Windows
    { Windows::new(&self.windows) }

    /// Returns an absolute widget path that is joint the window index with a pair of widget indices
    /// from the closure or `None`.
    ///
    /// The closure can be used to set the content widget for the window. The returned absolute
    /// widget path refers to the content widget.
    pub fn abs_widget_path1<C: Container + Any, F>(&mut self, idx: WindowIndex, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
    {
        match self.window_mut(idx) {
            Some(window) => {
                match container_rel_widget_path1(window, f) {
                    Some(rel_path) => Some(rel_path.to_abs_widget_path(idx)),
                    None => None,
                }
            },
            None => None,
        }
    }
    
    /// Returns a reference to the dynamic window for the specified absolute widget path or `None`.
    pub fn dyn_widget(&self, path: &AbsWidgetPath) -> Option<&dyn Widget>
    { self.dyn_window(path.window_index()).map(|w| w.dyn_widget(path.as_rel_widget_path())).flatten() }

    /// Returns a mutable reference to the dynamic window for the specified absolute widget path
    /// or otherwise `None`.
    pub fn dyn_widget_mut(&mut self, path: &AbsWidgetPath) -> Option<&mut dyn Widget>
    { self.dyn_window_mut(path.window_index()).map(|w| w.dyn_widget_mut(path.as_rel_widget_path())).flatten() }

    /// Returns a reference to the window for the specified absolute widget path or `None`.
    pub fn widget<T: Any>(&self, path: &AbsWidgetPath) -> Option<&T>
    { self.dyn_window(path.window_index()).map(|w| container_widget(w, path.as_rel_widget_path())).flatten() }

    /// Returns a mutable reference to the window for the specified absolute widget path or `None`.
    pub fn widget_mut<T: Any>(&mut self, path: &AbsWidgetPath) -> Option<&mut T>
    { self.dyn_window_mut(path.window_index()).map(|w| container_widget_mut(w, path.as_rel_widget_path())).flatten() }
    
    /// Returns an absolute widget path that is joint the specified absolute widget path with a pair
    /// of widget indices from the closure or `None`.
    ///
    /// The closure can be used to add a widget to the descendant container. The returned absolute
    /// widget path refers to the added widget.
    pub fn abs_widget_path<T: Any, F>(&mut self, path: &AbsWidgetPath, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
    { 
        match self.dyn_window_mut(path.window_index()) {
            Some(window) => {
                match container_rel_widget_path(window, path.as_rel_widget_path(), f) {
                    Some(rel_path) => Some(rel_path.to_abs_widget_path(path.window_index())),
                    None => None,
                }
            },
            None => None,
        }
    }

    /// Sets a parent for a child.
    ///
    /// This methods automatically adds the child to the parent and returns `Some(())` if the parent
    /// is set for the child, otherwise `None`.
    pub fn set_parent(&mut self, child_idx: WindowIndex, parent_idx: WindowIndex, pos: Pos<i32>) -> Option<()>
    {
        self.unset_parent(child_idx);
        {
            let child_window = self.dyn_window_mut(child_idx)?;
            child_window.set_parent(ParentWindowIndex::new(parent_idx), pos)?;
        }
        let is_success = match self.dyn_window_mut(parent_idx) {
            Some(parent_window) => parent_window.add_child(ChildWindowIndex::new(child_idx)).is_some(),
            None => false,
        };
        if !is_success {
            let child_window = self.dyn_window_mut(child_idx)?;
            child_window.unset_parent(ParentWindowTag::new());
            return None;
        }
        Some(())
    }

    /// Unsets a parent for a child.
    ///
    /// This method automatically removes the child from the parent and returns `Some(())` if the
    /// parent is unset for the child, otherwise `None`.
    pub fn unset_parent(&mut self, child_idx: WindowIndex) -> Option<()>
    {
        let parent_idx = {
            let child_window = self.dyn_window_mut(child_idx)?;
            match child_window.parent_index() {
                Some(parent_idx) => {
                    child_window.unset_parent(ParentWindowTag::new())?;
                    Some(parent_idx)
                },
                None => None,
            }
        };
        match parent_idx {
            Some(parent_idx) => {
                let parent_window = self.dyn_window_mut(parent_idx)?;
                parent_window.remove_child(ChildWindowIndex::new(child_idx))?;
                Some(())
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;
    
    #[test]
    fn test_window_container_adds_one_window()
    {
        let mut window_container = WindowContainer::new();
        let idx = window_container.add(MockEmptyWindow::new("test"));
        assert_eq!(Some(WindowIndex(0)), idx);
        assert_eq!(1, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(0), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }

    #[test]
    fn test_window_container_adds_many_windows()
    {
        let mut window_container = WindowContainer::new();
        let idx1 = window_container.add(MockEmptyWindow::new("test1"));
        let idx2 = window_container.add(MockEmptyWindow::new("test2"));
        let idx3 = window_container.add(MockEmptyWindow::new("test3"));
        assert_eq!(Some(WindowIndex(0)), idx1);
        assert_eq!(Some(WindowIndex(1)), idx2);
        assert_eq!(Some(WindowIndex(2)), idx3);
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }

    #[test]
    fn test_window_container_removes_one_window()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let removed_window = window_container.remove(WindowIndex(1));
        match removed_window {
            Some(removed_window) => assert_eq!(Some("test2"), removed_window.title()),
            None => assert!(false),
        }
        assert_eq!(2, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(1, window_container.free_indices.len());
        match window_container.free_indices.get(&IndexRange::new(1, 1)) {
            Some(idx_range) => {
                assert_eq!(1, idx_range.min);
                assert_eq!(1, idx_range.max);
            },
            None => assert!(false),
        }
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(1, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(1))); 
    }

    #[test]
    fn test_window_container_removes_many_windows()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.add(MockEmptyWindow::new("test4"));
        window_container.add(MockEmptyWindow::new("test5"));
        window_container.add(MockEmptyWindow::new("test6"));
        window_container.add(MockEmptyWindow::new("test7"));
        let removed_window1 = window_container.remove(WindowIndex(1));
        let removed_window2 = window_container.remove(WindowIndex(2));
        let removed_window3 = window_container.remove(WindowIndex(5));
        let removed_window4 = window_container.remove(WindowIndex(4));
        match removed_window1 {
            Some(removed_window) => assert_eq!(Some("test2"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window2 {
            Some(removed_window) => assert_eq!(Some("test3"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window3 {
            Some(removed_window) => assert_eq!(Some("test6"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window4 {
            Some(removed_window) => assert_eq!(Some("test5"), removed_window.title()),
            None => assert!(false),
        }
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(window) => assert_eq!(Some("test4"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(6)) {
            Some(window) => assert_eq!(Some("test7"), window.title()),
            None => assert!(false),
        }
        assert_eq!(2, window_container.free_indices.len());
        match window_container.free_indices.get(&IndexRange::new(1, 1)) {
            Some(idx_range) => {
                assert_eq!(1, idx_range.min);
                assert_eq!(2, idx_range.max);
            },
            None => assert!(false),
        }
        match window_container.free_indices.get(&IndexRange::new(4, 4)) {
            Some(idx_range) => {
                assert_eq!(4, idx_range.min);
                assert_eq!(5, idx_range.max);
            },
            None => assert!(false),
        }
        assert_eq!(Some(6), window_container.index_counter);
        assert_eq!(4, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(1))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(2))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(4))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(5)));
    }

    #[test]
    fn test_window_container_removes_one_window_between_two_index_ranges()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.add(MockEmptyWindow::new("test4"));
        window_container.add(MockEmptyWindow::new("test5"));
        window_container.add(MockEmptyWindow::new("test6"));
        window_container.add(MockEmptyWindow::new("test7"));
        let removed_window1 = window_container.remove(WindowIndex(1));
        let removed_window2 = window_container.remove(WindowIndex(2));
        let removed_window3 = window_container.remove(WindowIndex(5));
        let removed_window4 = window_container.remove(WindowIndex(4));
        let removed_window5 = window_container.remove(WindowIndex(3));
        match removed_window1 {
            Some(removed_window) => assert_eq!(Some("test2"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window2 {
            Some(removed_window) => assert_eq!(Some("test3"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window3 {
            Some(removed_window) => assert_eq!(Some("test6"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window4 {
            Some(removed_window) => assert_eq!(Some("test5"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window5 {
            Some(removed_window) => assert_eq!(Some("test4"), removed_window.title()),
            None => assert!(false),
        }
        assert_eq!(2, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(6)) {
            Some(window) => assert_eq!(Some("test7"), window.title()),
            None => assert!(false),
        }
        assert_eq!(1, window_container.free_indices.len());
        match window_container.free_indices.get(&IndexRange::new(1, 1)) {
            Some(idx_range) => {
                assert_eq!(1, idx_range.min);
                assert_eq!(5, idx_range.max);
            },
            None => assert!(false),
        }
        assert_eq!(Some(6), window_container.index_counter);
        assert_eq!(5, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(1))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(2))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(3)));
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(4))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(5)));
    }
    
    #[test]
    fn test_window_container_removes_last_window()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.add(MockEmptyWindow::new("test4"));
        let removed_window1 = window_container.remove(WindowIndex(1));
        let removed_window2 = window_container.remove(WindowIndex(2));
        let removed_window3 = window_container.remove(WindowIndex(3));
        match removed_window1 {
            Some(removed_window) => assert_eq!(Some("test2"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window2 {
            Some(removed_window) => assert_eq!(Some("test3"), removed_window.title()),
            None => assert!(false),
        }
        match removed_window3 {
            Some(removed_window) => assert_eq!(Some("test4"), removed_window.title()),
            None => assert!(false),
        }
        assert_eq!(1, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(0), window_container.index_counter);
        assert_eq!(3, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(1))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(2))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(3))); 
    }    

    #[test]
    fn test_window_container_removes_last_window_without_index_ranges()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let removed_window = window_container.remove(WindowIndex(2));
        match removed_window {
            Some(removed_window) => assert_eq!(Some("test3"), removed_window.title()),
            None => assert!(false),
        }
        assert_eq!(2, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(1), window_container.index_counter);
        assert_eq!(1, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(2))); 
    }

    #[test]
    fn test_window_container_removes_one_window_after_one_window_adding()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        let removed_window = window_container.remove(WindowIndex(0));
        match removed_window {
            Some(removed_window) => assert_eq!(Some("test1"), removed_window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.windows.is_empty());
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(None, window_container.index_counter);
        assert_eq!(1, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(0))); 
    }

    #[test]
    fn test_window_container_does_not_remove_non_existent_window()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let removed_window = window_container.remove(WindowIndex(3));
        assert_eq!(true, removed_window.is_none());
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }    
    
    #[test]
    fn test_window_container_adds_window_after_removing_of_windows()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.add(MockEmptyWindow::new("test4"));
        window_container.add(MockEmptyWindow::new("test5"));
        window_container.add(MockEmptyWindow::new("test6"));
        window_container.add(MockEmptyWindow::new("test7"));
        window_container.remove(WindowIndex(1));
        window_container.remove(WindowIndex(2));
        window_container.remove(WindowIndex(5));
        window_container.remove(WindowIndex(4));
        let idx = window_container.add(MockEmptyWindow::new("test8"));
        assert_eq!(Some(WindowIndex(1)), idx);
        assert_eq!(4, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test8"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(window) => assert_eq!(Some("test4"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(6)) {
            Some(window) => assert_eq!(Some("test7"), window.title()),
            None => assert!(false),
        }
        assert_eq!(2, window_container.free_indices.len());
        match window_container.free_indices.get(&IndexRange::new(2, 2)) {
            Some(idx_range) => {
                assert_eq!(2, idx_range.min);
                assert_eq!(2, idx_range.max);
            },
            None => assert!(false),
        }
        match window_container.free_indices.get(&IndexRange::new(4, 4)) {
            Some(idx_range) => {
                assert_eq!(4, idx_range.min);
                assert_eq!(5, idx_range.max);
            },
            None => assert!(false),
        }
        assert_eq!(Some(6), window_container.index_counter);
        assert_eq!(4, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(1))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(2))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(4))); 
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(5)));
    }
    
    #[test]
    fn test_window_container_adds_window_after_window_removing()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.remove(WindowIndex(1));
        let idx = window_container.add(MockEmptyWindow::new("test4"));
        assert_eq!(Some(WindowIndex(1)), idx);
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test4"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(1, window_container.indices_to_destroy.len());
        assert_eq!(true, window_container.indices_to_destroy.contains(&WindowIndex(1))); 
    }

    #[test]
    fn test_window_container_gives_window()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let window: Option<&MockEmptyWindow> = window_container.window(WindowIndex(1));
        match window {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }

    #[test]
    fn test_window_container_gives_mutable_window()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let window: Option<&mut MockEmptyWindow> = window_container.window_mut(WindowIndex(1));
        match window {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }
    
    #[test]
    fn test_window_container_gives_window_index_iterator()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let mut iter = window_container.window_indices();
        assert_eq!(true, iter.next().is_some());
        assert_eq!(true, iter.next().is_some());
        assert_eq!(true, iter.next().is_some());
        assert_eq!(true, iter.next().is_none());
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }    

    #[test]
    fn test_window_container_gives_dynamic_window_iterator()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        let mut iter = window_container.dyn_windows();
        assert_eq!(true, iter.next().is_some());
        assert_eq!(true, iter.next().is_some());
        assert_eq!(true, iter.next().is_some());
        assert_eq!(true, iter.next().is_none());
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => assert_eq!(Some("test1"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(window) => assert_eq!(Some("test2"), window.title()),
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(window) => assert_eq!(Some("test3"), window.title()),
            None => assert!(false),
        }
        assert_eq!(true, window_container.free_indices.is_empty());
        assert_eq!(Some(2), window_container.index_counter);
        assert_eq!(true, window_container.indices_to_destroy.is_empty());
    }

    #[test]
    fn test_window_container_sets_indices_for_windows_while_window_adding()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.remove(WindowIndex(1));
        window_container.add(MockEmptyWindow::new("test4"));
        let window1: Option<&MockEmptyWindow> = window_container.window(WindowIndex(0));
        let window2: Option<&MockEmptyWindow> = window_container.window(WindowIndex(1));
        let window3: Option<&MockEmptyWindow> = window_container.window(WindowIndex(2));
        match window1 {
            Some(window) => {
                assert_eq!(Some("test1"), window.title());
                assert_eq!(Some(WindowIndex(0)), window.index());
            },
            None => assert!(false),
        }
        match window2 {
            Some(window) => {
                assert_eq!(Some("test4"), window.title());
                assert_eq!(Some(WindowIndex(1)), window.index());
            },
            None => assert!(false),
        }
        match window3 {
            Some(window) => {
                assert_eq!(Some("test3"), window.title());
                assert_eq!(Some(WindowIndex(2)), window.index());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_unsets_indices_for_windows_while_window_removing()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test1"));
        window_container.add(MockEmptyWindow::new("test2"));
        window_container.add(MockEmptyWindow::new("test3"));
        window_container.remove(WindowIndex(1));
        window_container.add(MockEmptyWindow::new("test4"));
        let window1 = window_container.remove(WindowIndex(0));
        let window2 = window_container.remove(WindowIndex(1));
        let window3 = window_container.remove(WindowIndex(2));
        match window1 {
            Some(window) => {
                match dyn_window_as_window::<MockEmptyWindow>(&*window) {
                    Some(window) => {
                        assert_eq!(Some("test1"), window.title());
                        assert_eq!(None, window.index());
                    },
                    None => assert!(false),
                }
            },
            None => assert!(false),
        }
        match window2 {
            Some(window) => {
                match dyn_window_as_window::<MockEmptyWindow>(&*window) {
                    Some(window) => {
                        assert_eq!(Some("test4"), window.title());
                        assert_eq!(None, window.index());
                    },
                    None => assert!(false),
                }
            },
            None => assert!(false),
        }
        match window3 {
            Some(window) => {
                match dyn_window_as_window::<MockEmptyWindow>(&*window) {
                    Some(window) => {
                        assert_eq!(Some("test3"), window.title());
                        assert_eq!(None, window.index());
                    },
                    None => assert!(false),
                }
            },
            None => assert!(false),
        }
    }    
    
    #[test]
    fn test_window_container_sets_parent()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child"));
        window_container.add(MockParentWindow::new("parent"));
        match window_container.set_parent(WindowIndex(0), WindowIndex(1), Pos::new(1, 2)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(2, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child"), child.title());
                assert_eq!(Some(WindowIndex(1)), child.parent_index());
                assert_eq!(Some(Pos::new(1, 2)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_sets_parent_for_many_children()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child1"));
        window_container.add(MockChildWindow::new("child2"));
        window_container.add(MockChildWindow::new("child3"));
        window_container.add(MockParentWindow::new("parent"));
        match window_container.set_parent(WindowIndex(0), WindowIndex(3), Pos::new(1, 2)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        match window_container.set_parent(WindowIndex(1), WindowIndex(3), Pos::new(3, 4)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        match window_container.set_parent(WindowIndex(2), WindowIndex(3), Pos::new(5, 6)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(4, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child1"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(1, 2)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(child) => {
                assert_eq!(Some("child2"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(3, 4)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(child) => {
                assert_eq!(Some("child3"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(5, 6)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(Some(WindowIndex(1)), child_indices.next());
                assert_eq!(Some(WindowIndex(2)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_sets_many_parents()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child1"));
        window_container.add(MockChildWindow::new("child2"));
        window_container.add(MockParentWindow::new("parent1"));
        window_container.add(MockParentWindow::new("parent2"));
        match window_container.set_parent(WindowIndex(0), WindowIndex(2), Pos::new(1, 2)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        match window_container.set_parent(WindowIndex(1), WindowIndex(3), Pos::new(3, 4)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(4, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child1"), child.title());
                assert_eq!(Some(WindowIndex(2)), child.parent_index());
                assert_eq!(Some(Pos::new(1, 2)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(child) => {
                assert_eq!(Some("child2"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(3, 4)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(parent) => {
                assert_eq!(Some("parent1"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(parent) => {
                assert_eq!(Some("parent2"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(1)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_does_not_set_parent()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockEmptyWindow::new("test"));
        window_container.add(MockParentWindow::new("parent"));
        match window_container.set_parent(WindowIndex(0), WindowIndex(1), Pos::new(1, 2)) {
            Some(()) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(2, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(window) => {
                assert_eq!(Some("test"), window.title());
                assert_eq!(None, window.parent_index());
                assert_eq!(None, window.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_container_unsets_parent()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child1"));
        window_container.add(MockChildWindow::new("child2"));
        window_container.add(MockChildWindow::new("child3"));
        window_container.add(MockParentWindow::new("parent"));
        window_container.set_parent(WindowIndex(0), WindowIndex(3), Pos::new(1, 2));
        window_container.set_parent(WindowIndex(1), WindowIndex(3), Pos::new(3, 4));
        window_container.set_parent(WindowIndex(2), WindowIndex(3), Pos::new(5, 6));
        match window_container.unset_parent(WindowIndex(1)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(4, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child1"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(1, 2)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(child) => {
                assert_eq!(Some("child2"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(child) => {
                assert_eq!(Some("child3"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(5, 6)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(Some(WindowIndex(2)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }    

    #[test]
    fn test_window_container_unsets_many_parent()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child1"));
        window_container.add(MockChildWindow::new("child2"));
        window_container.add(MockChildWindow::new("child3"));
        window_container.add(MockChildWindow::new("child4"));
        window_container.add(MockParentWindow::new("parent"));
        window_container.set_parent(WindowIndex(0), WindowIndex(4), Pos::new(1, 2));
        window_container.set_parent(WindowIndex(1), WindowIndex(4), Pos::new(3, 4));
        window_container.set_parent(WindowIndex(2), WindowIndex(4), Pos::new(5, 6));
        window_container.set_parent(WindowIndex(3), WindowIndex(4), Pos::new(7, 8));
        match window_container.unset_parent(WindowIndex(1)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        match window_container.unset_parent(WindowIndex(2)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(5, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child1"), child.title());
                assert_eq!(Some(WindowIndex(4)), child.parent_index());
                assert_eq!(Some(Pos::new(1, 2)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(child) => {
                assert_eq!(Some("child2"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(child) => {
                assert_eq!(Some("child3"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(child) => {
                assert_eq!(Some("child4"), child.title());
                assert_eq!(Some(WindowIndex(4)), child.parent_index());
                assert_eq!(Some(Pos::new(7, 8)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(4)) {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(Some(WindowIndex(3)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_container_automatically_unsets_parent_for_parent_removing()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child1"));
        window_container.add(MockChildWindow::new("child2"));
        window_container.add(MockChildWindow::new("child3"));
        window_container.add(MockParentWindow::new("parent"));
        window_container.set_parent(WindowIndex(0), WindowIndex(3), Pos::new(1, 2));
        window_container.set_parent(WindowIndex(1), WindowIndex(3), Pos::new(3, 4));
        window_container.set_parent(WindowIndex(2), WindowIndex(3), Pos::new(5, 6));
        let parent = window_container.remove(WindowIndex(3));
        match parent {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child1"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(child) => {
                assert_eq!(Some("child2"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(child) => {
                assert_eq!(Some("child3"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_automatically_unsets_parent_for_child_removing()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child1"));
        window_container.add(MockChildWindow::new("child2"));
        window_container.add(MockChildWindow::new("child3"));
        window_container.add(MockParentWindow::new("parent"));
        window_container.set_parent(WindowIndex(0), WindowIndex(3), Pos::new(1, 2));
        window_container.set_parent(WindowIndex(1), WindowIndex(3), Pos::new(3, 4));
        window_container.set_parent(WindowIndex(2), WindowIndex(3), Pos::new(5, 6));
        let child = window_container.remove(WindowIndex(1));
        match child {
            Some(child) => {
                assert_eq!(Some("child2"), child.title());
                assert_eq!(None, child.parent_index());
                assert_eq!(None, child.pos_in_parent());
            },
            None => assert!(false),
        }
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child1"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(1, 2)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(child) => {
                assert_eq!(Some("child3"), child.title());
                assert_eq!(Some(WindowIndex(3)), child.parent_index());
                assert_eq!(Some(Pos::new(5, 6)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(3)) {
            Some(parent) => {
                assert_eq!(Some("parent"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(Some(WindowIndex(2)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_container_automatically_unsets_parent_for_parent_setting()
    {
        let mut window_container = WindowContainer::new();
        window_container.add(MockChildWindow::new("child"));
        window_container.add(MockParentWindow::new("parent1"));
        window_container.add(MockParentWindow::new("parent2"));
        window_container.set_parent(WindowIndex(0), WindowIndex(1), Pos::new(1, 2));
        match window_container.set_parent(WindowIndex(0), WindowIndex(2), Pos::new(3, 4)) {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(3, window_container.windows.len());
        match window_container.windows.get(&WindowIndex(0)) {
            Some(child) => {
                assert_eq!(Some("child"), child.title());
                assert_eq!(Some(WindowIndex(2)), child.parent_index());
                assert_eq!(Some(Pos::new(3, 4)), child.pos_in_parent());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(1)) {
            Some(parent) => {
                assert_eq!(Some("parent1"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
        match window_container.windows.get(&WindowIndex(2)) {
            Some(parent) => {
                assert_eq!(Some("parent2"), parent.title());
                let mut child_indices = parent.child_indices();
                assert_eq!(Some(WindowIndex(0)), child_indices.next());
                assert_eq!(None, child_indices.next());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_sets_one_widget_to_window()
    {
        let mut window_container = WindowContainer::new();
        let idx = match window_container.add(MockWindow::new("test1")) {
            Some(tmp_idx) => tmp_idx, 
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget = MockWidget::new("test2");
        let path = match window_container.abs_widget_path1(idx, |w: &mut MockWindow| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let expected_path = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let widget: Option<&MockWidget> = window_container.widget(&path);
        match widget {
            Some(widget) => assert_eq!("test2", widget.text()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_container_adds_widgets_to_layout()
    {
        let mut window_container = WindowContainer::new();
        let idx = match window_container.add(MockWindow::new("test1")) {
            Some(tmp_idx) => tmp_idx, 
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout = MockLayout::new("test2");
        let path = match window_container.abs_widget_path1(idx, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match window_container.abs_widget_path(&path, |w: &mut MockLayout| w.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match window_container.abs_widget_path(&path, |w: &mut MockLayout| w.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match window_container.abs_widget_path(&path, |w: &mut MockLayout| w.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let expected_path = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let mut expected_path1 = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        expected_path1.push(WidgetIndexPair(0, 0));
        assert_eq!(expected_path1, path1);
        let mut expected_path2 = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        expected_path2.push(WidgetIndexPair(1, 0));
        assert_eq!(expected_path2, path2);
        let mut expected_path3 = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        expected_path3.push(WidgetIndexPair(2, 0));
        assert_eq!(expected_path3, path3);
        let layout: Option<&MockLayout> = window_container.widget(&path);
        match layout {
            Some(layout) => assert_eq!("test2", layout.text()),
            None => assert!(false),
        }
        let widget1: Option<&MockWidget> = window_container.widget(&path1);
        match widget1 {
            Some(widget1) => assert_eq!("test3", widget1.text()),
            None => assert!(false),
        }
        let widget2: Option<&MockWidget> = window_container.widget(&path2);
        match widget2 {
            Some(widget2) => assert_eq!("test4", widget2.text()),
            None => assert!(false),
        }
        let widget3: Option<&MockWidget> = window_container.widget(&path3);
        match widget3 {
            Some(widget3) => assert_eq!("test5", widget3.text()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_container_sets_one_widget_to_window_for_mutable()
    {
        let mut window_container = WindowContainer::new();
        let idx = match window_container.add(MockWindow::new("test1")) {
            Some(tmp_idx) => tmp_idx, 
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget = MockWidget::new("test2");
        let path = match window_container.abs_widget_path1(idx, |w: &mut MockWindow| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let expected_path = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let widget: Option<&mut MockWidget> = window_container.widget_mut(&path);
        match widget {
            Some(widget) => assert_eq!("test2", widget.text()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_container_adds_widgets_to_layout_for_mutable()
    {
        let mut window_container = WindowContainer::new();
        let idx = match window_container.add(MockWindow::new("test1")) {
            Some(tmp_idx) => tmp_idx, 
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout = MockLayout::new("test2");
        let path = match window_container.abs_widget_path1(idx, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match window_container.abs_widget_path(&path, |w: &mut MockLayout| w.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match window_container.abs_widget_path(&path, |w: &mut MockLayout| w.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match window_container.abs_widget_path(&path, |w: &mut MockLayout| w.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let expected_path = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let mut expected_path1 = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        expected_path1.push(WidgetIndexPair(0, 0));
        assert_eq!(expected_path1, path1);
        let mut expected_path2 = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        expected_path2.push(WidgetIndexPair(1, 0));
        assert_eq!(expected_path2, path2);
        let mut expected_path3 = AbsWidgetPath::new(idx, WidgetIndexPair(0, 0));
        expected_path3.push(WidgetIndexPair(2, 0));
        assert_eq!(expected_path3, path3);
        let layout: Option<&mut MockLayout> = window_container.widget_mut(&path);
        match layout {
            Some(layout) => assert_eq!("test2", layout.text()),
            None => assert!(false),
        }
        let widget1: Option<&mut MockWidget> = window_container.widget_mut(&path1);
        match widget1 {
            Some(widget1) => assert_eq!("test3", widget1.text()),
            None => assert!(false),
        }
        let widget2: Option<&mut MockWidget> = window_container.widget_mut(&path2);
        match widget2 {
            Some(widget2) => assert_eq!("test4", widget2.text()),
            None => assert!(false),
        }
        let widget3: Option<&mut MockWidget> = window_container.widget_mut(&path3);
        match widget3 {
            Some(widget3) => assert_eq!("test5", widget3.text()),
            None => assert!(false),
        }
    }
}
