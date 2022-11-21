//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use crate::theme::*;
use crate::types::*;
use crate::container::*;
use crate::widget::*;
use crate::window::*;
use crate::window_pool::*;

pub struct WindowContext
{
    theme: Box<dyn Theme>,
    window_pool: WindowPool,
    current_window_index: Option<WindowIndex>,
}

impl WindowContext
{
    pub(crate) fn new(theme: Box<dyn Theme>) -> Self
    { WindowContext { theme, window_pool: WindowPool::new(), current_window_index: None } }
    
    pub fn theme(&self) -> &dyn Theme
    { &*self.theme }
    
    pub fn window_pool(&self) -> &WindowPool
    { &self.window_pool }

    pub fn window_pool_mut(&mut self) -> &mut WindowPool
    { &mut self.window_pool }
    
    pub fn current_window_index(&self) -> Option<WindowIndex>
    { self.current_window_index }

    pub(crate) fn set_current_window_index(&mut self, idx: Option<WindowIndex>)
    { self.current_window_index = idx; }

    pub fn dyn_current_window(&self) -> Option<&dyn Window>
    {
        match self.current_window_index {
            Some(idx) => self.window_pool.dyn_window(idx),
            None => None,
        }
    }

    pub fn dyn_current_window_mut(&mut self) -> Option<&mut dyn Window>
    {
        match self.current_window_index {
            Some(idx) => self.window_pool.dyn_window_mut(idx),
            None => None,
        }
    }

    pub fn current_window<T: Any>(&self) -> Option<&T>
    {
        match self.current_window_index {
            Some(idx) => self.window_pool.window(idx),
            None => None,
        }
    }

    pub fn current_window_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        match self.current_window_index {
            Some(idx) => self.window_pool.window_mut(idx),
            None => None,
        }
    }

    pub fn add_dyn_window(&mut self, window: Box<dyn Window>) -> Option<WindowIndex>
    { self.window_pool.add_dyn(window) }
    
    pub fn add_window<T: Window + 'static>(&mut self, window: T) -> Option<WindowIndex>
    { self.window_pool.add_dyn(Box::new(window)) }
    
    pub fn remove_window(&mut self, idx: WindowIndex) -> Option<Box<dyn Window>>
    { self.window_pool.remove(idx) }

    pub fn dyn_window(&self, idx: WindowIndex) -> Option<&dyn Window>
    { self.window_pool.dyn_window(idx) }

    pub fn dyn_window_mut(&mut self, idx: WindowIndex) -> Option<&mut dyn Window>
    { self.window_pool.dyn_window_mut(idx) }

    pub fn window<T: Any>(&self, idx: WindowIndex) -> Option<&T>
    { self.window_pool.window(idx) }

    pub fn window_mut<T: Any>(&mut self, idx: WindowIndex) -> Option<&mut T>
    { self.window_pool.window_mut(idx) }

    pub fn abs_widget_path1<C: Container + Any, F>(&mut self, idx: WindowIndex, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
    { self.window_pool.abs_widget_path1(idx, f) }
    
    pub fn dyn_widget(&self, path: &AbsWidgetPath) -> Option<&dyn Widget>
    { self.window_pool.dyn_widget(path) }

    pub fn dyn_widget_mut(&mut self, path: &AbsWidgetPath) -> Option<&mut dyn Widget>
    { self.window_pool.dyn_widget_mut(path) }

    pub fn widget<T: Any>(&self, path: &AbsWidgetPath) -> Option<&T>
    { self.window_pool.widget(path) }

    pub fn widget_mut<T: Any>(&mut self, path: &AbsWidgetPath) -> Option<&mut T>
    { self.window_pool.widget_mut(path) }
    
    pub fn abs_widget_path<T: Any, F>(&mut self, path: &AbsWidgetPath, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
    { self.window_pool.abs_widget_path(path, f) }
    
    pub fn set_parent_window(&mut self, child_idx: WindowIndex, parent_idx: WindowIndex, pos: Pos<i32>) -> Option<()>
    { self.window_pool.set_parent(child_idx, parent_idx, pos) }

    pub fn unset_parent_window(&mut self, child_idx: WindowIndex) -> Option<()>
    { self.window_pool.unset_parent(child_idx) }
}
