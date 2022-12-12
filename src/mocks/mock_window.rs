//
// Copyright (c) 2022 Łukasz Szpakowski
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
use crate::widget::*;
use crate::window::*;

pub(crate) struct MockWindow
{
    title: String,
    size: Size<i32>,
    padding_bounds: Rect<i32>,
    is_visible: bool,
    is_focused: bool,
    change_flag_arc: Arc<AtomicBool>,
    min_size: Size<Option<i32>>,
    preferred_size: Size<Option<i32>>,
    content: Option<Box<dyn Widget>>,
}

impl MockWindow
{
    pub(crate) fn new(title: &str) -> Self
    {
        MockWindow {
            title: String::from(title),
            size: Size::new(0, 0),
            padding_bounds: Rect::new(0, 0, 0, 0),
            is_visible: true,
            is_focused: false,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            min_size: Size::new(None, None),
            preferred_size: Size::new(None, None),
            content: None,
        }
    }
    
    pub(crate) fn set_size(&mut self, size: Size<i32>)
    { self.size = size; }

    pub(crate) fn set_padding_bounds(&mut self, bounds: Rect<i32>)
    { self.padding_bounds = bounds; }
    
    pub(crate) fn set_visible(&mut self, is_visible: bool)
    { self.is_visible = is_visible; }
    
    pub(crate) fn set_change_flag(&mut self, is_changed: bool)
    { self.change_flag_arc.store(is_changed, Ordering::SeqCst); }
    
    pub(crate) fn set_dyn(&mut self, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        self.content = Some(widget);
        Some(WidgetIndexPair(0, 0))
    }

    pub(crate) fn set<T: Widget + 'static>(&mut self, widget: T) -> Option<WidgetIndexPair>
    { self.set_dyn(Box::new(widget)) }
}

impl Window for MockWindow
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
    { self.is_focused = is_focused; }

    fn title(&self) -> Option<&str>
    { Some(self.title.as_str()) }
    
    fn is_changed(&self) -> bool
    { self.change_flag_arc.load(Ordering::SeqCst) }
    
    fn clear_change_flag(&mut self)
    { self.change_flag_arc.store(false, Ordering::SeqCst); }
    
    fn content_index_pair(&self) -> Option<WidgetIndexPair>
    {
        if self.content.is_some() {
            Some(WidgetIndexPair(0, 0))
        } else {
            None
        }
    }
}

impl Container for MockWindow
{
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if self.content.is_some() {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if self.content.is_some() {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        if idx_pair == WidgetIndexPair(0, 0) {
            match &self.content {
                Some(widget) => Some(&**widget),
                None => None,
            }
        } else {
            None
        }
    }

    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    {
        if idx_pair == WidgetIndexPair(0, 0) {
            match &mut self.content {
                Some(widget) => Some(&mut **widget),
                None => None,
            }
        } else {
            None
        }
    }
    
    fn point_for_index_pair(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    {
        match &self.content {
            Some(widget) => {
                if widget.bounds().to_f64_rect().contains(pos) {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

impl MinSize for MockWindow
{
    fn min_size(&self) -> Size<Option<i32>>
    { self.min_size }
    
    fn set_min_size(&mut self, size: Size<Option<i32>>)
    { self.min_size = size; }
}

impl PreferredSize for MockWindow
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    { self.preferred_size = size; }
}

impl Draw for MockWindow
{
    fn update_size(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_size: Size<Option<i32>>)
    {}
    
    fn update_pos(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_bounds: Rect<i32>)
    {}

    fn draw(&self, _cairo_context: &CairoContext, _theme: &dyn Theme, _is_focused_window: bool)
    {}
}

impl CallOn for MockWindow
{
    fn call_on(&mut self, _client_context: &mut ClientContext, _queue_context: &mut QueueContext, _event: &Event) -> Option<Option<Event>>
    { Some(None) }
}

impl AsAny for MockWindow
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}
