//
// Copyright (c) 2022-2023 Łukasz Szpakowski
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
use crate::window_container::*;

pub struct WindowContext
{
    pub(crate) theme: Box<dyn Theme>,
    pub(crate) window_container: WindowContainer,
    pub(crate) current_window_index: Option<WindowIndex>,
    pub(crate) current_pos: Option<Pos<f64>>,
    pub(crate) focused_window_index: Option<WindowIndex>,
    pub(crate) old_focused_window_index: Option<WindowIndex>,
}

impl WindowContext
{
    pub(crate) fn new(theme: Box<dyn Theme>) -> Self
    {
        WindowContext {
            theme,
            window_container: WindowContainer::new(),
            current_window_index: None,
            current_pos: None,
            focused_window_index: None,
            old_focused_window_index: None,
        }
    }
    
    pub fn theme(&self) -> &dyn Theme
    { &*self.theme }
    
    pub fn window_container(&self) -> &WindowContainer
    { &self.window_container }

    pub fn window_container_mut(&mut self) -> &mut WindowContainer
    { &mut self.window_container }
    
    pub fn current_window_index(&self) -> Option<WindowIndex>
    { self.current_window_index }

    pub fn current_pos(&self) -> Option<Pos<f64>>
    { self.current_pos }
    
    pub fn focused_window_index(&self) -> Option<WindowIndex>
    { self.focused_window_index }
    
    pub fn set_focused_window_index(&mut self, idx: Option<WindowIndex>)
    { self.focused_window_index = idx; }
    
    pub fn dyn_current_window(&self) -> Option<&dyn Window>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.dyn_window(idx),
            None => None,
        }
    }

    pub fn dyn_current_window_mut(&mut self) -> Option<&mut dyn Window>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.dyn_window_mut(idx),
            None => None,
        }
    }

    pub fn current_window<T: Any>(&self) -> Option<&T>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.window(idx),
            None => None,
        }
    }

    pub fn current_window_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.window_mut(idx),
            None => None,
        }
    }

    pub fn add_dyn_window(&mut self, window: Box<dyn Window>) -> Option<WindowIndex>
    { self.window_container.add_dyn(window) }
    
    pub fn add_window<T: Window + 'static>(&mut self, window: T) -> Option<WindowIndex>
    { self.window_container.add_dyn(Box::new(window)) }
    
    pub fn remove_window(&mut self, idx: WindowIndex) -> Option<Box<dyn Window>>
    { self.window_container.remove(idx) }

    pub fn dyn_window(&self, idx: WindowIndex) -> Option<&dyn Window>
    { self.window_container.dyn_window(idx) }

    pub fn dyn_window_mut(&mut self, idx: WindowIndex) -> Option<&mut dyn Window>
    { self.window_container.dyn_window_mut(idx) }

    pub fn window<T: Any>(&self, idx: WindowIndex) -> Option<&T>
    { self.window_container.window(idx) }

    pub fn window_mut<T: Any>(&mut self, idx: WindowIndex) -> Option<&mut T>
    { self.window_container.window_mut(idx) }

    pub fn abs_widget_path1<C: Container + Any, F>(&mut self, idx: WindowIndex, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
    { self.window_container.abs_widget_path1(idx, f) }
    
    pub fn dyn_widget(&self, path: &AbsWidgetPath) -> Option<&dyn Widget>
    { self.window_container.dyn_widget(path) }

    pub fn dyn_widget_mut(&mut self, path: &AbsWidgetPath) -> Option<&mut dyn Widget>
    { self.window_container.dyn_widget_mut(path) }

    pub fn widget<T: Any>(&self, path: &AbsWidgetPath) -> Option<&T>
    { self.window_container.widget(path) }

    pub fn widget_mut<T: Any>(&mut self, path: &AbsWidgetPath) -> Option<&mut T>
    { self.window_container.widget_mut(path) }
    
    pub fn abs_widget_path<T: Any, F>(&mut self, path: &AbsWidgetPath, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
    { self.window_container.abs_widget_path(path, f) }
    
    pub fn set_parent_window(&mut self, child_idx: WindowIndex, parent_idx: WindowIndex, pos: Pos<i32>) -> Option<()>
    { self.window_container.set_parent(child_idx, parent_idx, pos) }

    pub fn unset_parent_window(&mut self, child_idx: WindowIndex) -> Option<()>
    { self.window_container.unset_parent(child_idx) }
}
