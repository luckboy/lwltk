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

/// A structure of window context.
///
/// The window context is used to menage widgets and windows. The structure of window context
/// contains a theme and a window container.
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
    
    /// Returns the reference to the theme.
    pub fn theme(&self) -> &dyn Theme
    { &*self.theme }
    
    /// Returns a reference to the window container.
    pub fn window_container(&self) -> &WindowContainer
    { &self.window_container }

    /// Returns a mutable reference to the window container.
    pub fn window_container_mut(&mut self) -> &mut WindowContainer
    { &mut self.window_container }
    
    /// Returns the current window index or `None`.
    ///
    /// The current window index refers the window for an wayland event that is called or a thread
    /// signal.
    pub fn current_window_index(&self) -> Option<WindowIndex>
    { self.current_window_index }

    /// Returns the current position or `None`.
    ///
    /// The current position is pointed by the pointer or the touch for an wayland event that is
    /// called or a thread signal.
    pub fn current_pos(&self) -> Option<Pos<f64>>
    { self.current_pos }
    
    /// Returns the focused window index or `None`.
    pub fn focused_window_index(&self) -> Option<WindowIndex>
    { self.focused_window_index }
    
    /// Sets the focused window index.
    pub fn set_focused_window_index(&mut self, idx: Option<WindowIndex>)
    { self.focused_window_index = idx; }
    
    /// Returns a reference to the dynamic current window or `None`.
    pub fn dyn_current_window(&self) -> Option<&dyn Window>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.dyn_window(idx),
            None => None,
        }
    }

    /// Returns a mutable reference to the dynamic current window or `None`.
    pub fn dyn_current_window_mut(&mut self) -> Option<&mut dyn Window>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.dyn_window_mut(idx),
            None => None,
        }
    }

    /// Returns a reference to the current window or `None`.
    pub fn current_window<T: Any>(&self) -> Option<&T>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.window(idx),
            None => None,
        }
    }

    /// Returns a mutable reference to the current window or `None`.
    pub fn current_window_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        match self.current_window_index {
            Some(idx) => self.window_container.window_mut(idx),
            None => None,
        }
    }

    /// Returns a reference to the dynamic focused window or `None`.
    pub fn dyn_focused_window(&self) -> Option<&dyn Window>
    {
        match self.focused_window_index {
            Some(idx) => self.window_container.dyn_window(idx),
            None => None,
        }
    }

    /// Returns a mutable reference to the dynamic focused window or `None`.
    pub fn dyn_focused_window_mut(&mut self) -> Option<&mut dyn Window>
    {
        match self.focused_window_index {
            Some(idx) => self.window_container.dyn_window_mut(idx),
            None => None,
        }
    }    
    
    /// Returns a reference to the focused window or `None`.
    pub fn focused_window<T: Any>(&self) -> Option<&T>
    {
        match self.focused_window_index {
            Some(idx) => self.window_container.window(idx),
            None => None,
        }
    }

    /// Returns a mutable reference to the focused window or `None`.
    pub fn focused_window_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        match self.focused_window_index {
            Some(idx) => self.window_container.window_mut(idx),
            None => None,
        }
    }
    
    /// See [`WindowContainer::add_dyn`].
    pub fn add_dyn_window(&mut self, window: Box<dyn Window>) -> Option<WindowIndex>
    { self.window_container.add_dyn(window) }
    
    /// See [`WindowContainer::add`].
    pub fn add_window<T: Window + 'static>(&mut self, window: T) -> Option<WindowIndex>
    { self.window_container.add_dyn(Box::new(window)) }
    
    /// See [`WindowContainer::remove`].
    pub fn remove_window(&mut self, idx: WindowIndex) -> Option<Box<dyn Window>>
    { self.window_container.remove(idx) }

    /// See [`WindowContainer::dyn_window`].
    pub fn dyn_window(&self, idx: WindowIndex) -> Option<&dyn Window>
    { self.window_container.dyn_window(idx) }

    /// See [`WindowContainer::dyn_window_mut`].
    pub fn dyn_window_mut(&mut self, idx: WindowIndex) -> Option<&mut dyn Window>
    { self.window_container.dyn_window_mut(idx) }

    /// See [`WindowContainer::window`].
    pub fn window<T: Any>(&self, idx: WindowIndex) -> Option<&T>
    { self.window_container.window(idx) }

    /// See [`WindowContainer::window_mut`].
    pub fn window_mut<T: Any>(&mut self, idx: WindowIndex) -> Option<&mut T>
    { self.window_container.window_mut(idx) }

    /// See [`WindowContainer::abs_widget_path1`].
    pub fn abs_widget_path1<C: Container + Any, F>(&mut self, idx: WindowIndex, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
    { self.window_container.abs_widget_path1(idx, f) }
    
    /// See [`WindowContainer::dyn_widget`].
    pub fn dyn_widget(&self, path: &AbsWidgetPath) -> Option<&dyn Widget>
    { self.window_container.dyn_widget(path) }

    /// See [`WindowContainer::dyn_widget_mut`].
    pub fn dyn_widget_mut(&mut self, path: &AbsWidgetPath) -> Option<&mut dyn Widget>
    { self.window_container.dyn_widget_mut(path) }

    /// See [`WindowContainer::widget`].
    pub fn widget<T: Any>(&self, path: &AbsWidgetPath) -> Option<&T>
    { self.window_container.widget(path) }

    /// See [`WindowContainer::widget_mut`].
    pub fn widget_mut<T: Any>(&mut self, path: &AbsWidgetPath) -> Option<&mut T>
    { self.window_container.widget_mut(path) }
    
    /// See [`WindowContainer::abs_widget_path`].
    pub fn abs_widget_path<T: Any, F>(&mut self, path: &AbsWidgetPath, f: F) -> Option<AbsWidgetPath>
        where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
    { self.window_container.abs_widget_path(path, f) }
    
    /// See [`WindowContainer::set_parent`].
    pub fn set_parent_window(&mut self, child_idx: WindowIndex, parent_idx: WindowIndex, pos: Pos<i32>) -> Option<()>
    { self.window_container.set_parent(child_idx, parent_idx, pos) }

    /// See [`WindowContainer::unset_parent`].
    pub fn unset_parent_window(&mut self, child_idx: WindowIndex) -> Option<()>
    { self.window_container.unset_parent(child_idx) }
}
