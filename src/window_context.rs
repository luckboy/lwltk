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
}

impl WindowContext
{
    pub(crate) fn new(theme: Box<dyn Theme>) -> Self
    { WindowContext { theme, window_pool: WindowPool::new(), } }
    
    pub fn theme(&self) -> &dyn Theme
    { &*self.theme }
    
    pub fn window_pool(&self) -> &WindowPool
    { &self.window_pool }

    pub fn window_pool_mut(&mut self) -> &mut WindowPool
    { &mut self.window_pool }

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
}
