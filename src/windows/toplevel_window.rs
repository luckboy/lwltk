//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use crate::as_any::*;
use crate::call_on::*;
use crate::client_context::*;
use crate::container::*;
use crate::draw::*;
use crate::events::*;
use crate::min_size::*;
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::types::*;
use crate::window::*;
use crate::windows::child_index_set::*;

pub struct ToplevelWindow
{
    title: Option<String>,
    size: Size<i32>,
    padding_bounds: Rect<i32>,
    is_visible: bool,
    is_focused: bool,
    change_flag_arc: Arc<AtomicBool>,
    min_size: Size<Option<i32>>,
    preferred_size: Size<Option<i32>>,
    child_index_set: ChildIndexSet,
}

impl ToplevelWindow
{
    pub fn new() -> Self
    {
        ToplevelWindow {
            title: None,
            size: Size::new(0, 0),
            padding_bounds: Rect::new(0, 0, 0, 0),
            is_visible: true,
            is_focused: false,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            min_size: Size::new(None, None),
            preferred_size: Size::new(None, None),
            child_index_set: ChildIndexSet::new(),
        }
    }
    
    pub fn set_title(&mut self, title: &str)
    {
        self.title = Some(String::from(title));
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
    
    pub fn unset_title(&mut self)
    {
        self.title = None;
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }

    pub fn set_visible(&mut self, is_visible: bool)
    {
        self.is_visible = is_visible;
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
}

impl Window for ToplevelWindow
{
    fn size(&self) -> Size<i32>
    { self.size }

    fn padding_bounds(&self) -> Rect<i32>
    { self.padding_bounds }

    fn is_visible(&self) -> bool
    { self.is_visible }
    
    fn is_focused(&self) -> bool
    { self.is_focused }
    
    fn set_focus(&mut self, is_focused: bool)
    {
        let old_focus_flag = self.is_focused;
        self.is_focused = is_focused;
        if old_focus_flag != self.is_focused {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    fn title(&self) -> Option<&str>
    {
        match &self.title {
            Some(title) => Some(title.as_str()),
            None => None,
        }
    }
    
    fn is_changed(&self) -> bool
    { self.change_flag_arc.load(Ordering::SeqCst) }
    
    fn clear_change_flag(&mut self)
    { self.change_flag_arc.store(false, Ordering::SeqCst); }

    fn child_index_iter(&self) -> Option<Box<dyn WindowIterator + '_>>
    { self.child_index_set.child_index_iter() }

    fn add_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    { self.child_index_set.add(idx) }

    fn remove_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    { self.child_index_set.remove(idx) }
}

impl Container for ToplevelWindow
{}

impl MinSize for ToplevelWindow
{
    fn min_size(&self) -> Size<Option<i32>>
    { self.min_size }
    
    fn set_min_size(&mut self, size: Size<Option<i32>>)
    { self.min_size = size; }
}

impl PreferredSize for ToplevelWindow
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    { self.preferred_size = size; }
}

impl Draw for ToplevelWindow
{
    fn update_size(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        match area_size.width {
            Some(width) => self.size.width = width,
            None => self.size.width = 1,
        }
        match area_size.height {
            Some(height) => self.size.height = height,
            None => self.size.height = 1,
        }
        Ok(())
    }
    
    fn update_pos(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_bounds: Rect<i32>) -> Result<(), CairoError>
    { Ok(()) }

    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, _is_focused_window: bool) -> Result<(), CairoError>
    { theme.draw_window_bg(cairo_context, Rect::new(0, 0, self.size.width, self.size.height)) }
}

impl CallOn for ToplevelWindow
{
    fn call_on(&mut self, _client_context: &mut ClientContext, _queue_context: &mut QueueContext, _event: &Event) -> Option<Option<Event>>
    { Some(None) }
}

impl AsAny for ToplevelWindow
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}
