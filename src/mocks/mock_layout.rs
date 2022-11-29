//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::cmp::max;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use crate::as_any::*;
use crate::call_on::*;
use crate::client_context::*;
use crate::container::*;
use crate::draw::*;
use crate::events::*;
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::types::*;
use crate::widget::*;

pub(crate) struct MockLayout
{
    text: String,
    bounds: Rect<i32>,
    client_pos: Pos<i32>,
    h_align: HAlign,
    v_align: VAlign,
    state: WidgetState,
    is_focusable: bool,
    is_focused: bool,
    change_flag_arc: Arc<AtomicBool>,
    preferred_size: Size<Option<i32>>,
    widgets: Vec<Box<dyn Widget>>,
}

impl MockLayout
{
    pub(crate) fn new(s: &str) -> Self
    {
        MockLayout {
            text: String::from(s),
            bounds: Rect::new(0, 0, 0, 0),
            client_pos: Pos::new(0, 0),
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            state: WidgetState::None,
            is_focusable: false,
            is_focused: false,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            preferred_size: Size::new(None, None),
            widgets: Vec::new(),
        }
    }
    
    pub(crate) fn text(&self) -> &str
    { self.text.as_str() }

    pub(crate) fn set_bounds(&mut self, bounds: Rect<i32>)
    { self.bounds = bounds; }

    pub(crate) fn set_client_pos(&mut self, pos: Pos<i32>)
    { self.client_pos = pos; }
    
    pub(crate) fn set_h_align(&mut self, align: HAlign)
    { self.h_align = align; }

    pub(crate) fn set_v_align(&mut self, align: VAlign)
    { self.v_align = align; }

    pub(crate) fn set_focusable(&mut self, is_focusable: bool)
    { self.is_focusable = is_focusable; }

    pub(crate) fn set_change_flag(&mut self, is_changed: bool)
    { self.change_flag_arc.store(is_changed, Ordering::SeqCst); }
    
    pub(crate) fn add_dyn(&mut self, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        let i = self.widgets.len();
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        self.widgets.push(widget);
        Some(WidgetIndexPair(i, 0))
    }

    pub(crate) fn add<T: Widget + 'static>(&mut self, widget: T) -> Option<WidgetIndexPair>
    { self.add_dyn(Box::new(widget)) }
}

impl Widget for MockLayout
{
    fn margin_bounds(&self) -> Rect<i32>
    { self.bounds }
    
    fn bounds(&self) -> Rect<i32>
    { self.bounds }

    fn h_align(&self) -> HAlign
    { self.h_align }
    
    fn v_align(&self) -> VAlign
    { self.v_align }

    fn state(&self) -> WidgetState
    { self.state }
    
    fn set_state(&mut self, state: WidgetState)
    { self.state = state; }
    
    fn is_focusable(&self) -> bool
    { self.is_focusable }
    
    fn is_focused(&self) -> bool
    { self.is_focusable && self.is_focused }
    
    fn set_focus(&mut self, is_focused: bool) -> bool
    {
        if self.is_focusable {
            self.is_focused = is_focused;
            true
        } else {
            false
        }
    }
    
    fn h_scroll_bar_slider_x(&self, viewport_width: i32, trough_width: i32) -> f64
    { 
        let client_width = max(viewport_width, self.bounds.width);
        (self.client_pos.x as f64) * (trough_width as f64) / (client_width as f64)
    }

    fn h_scroll_bar_slider_width(&self, viewport_width: i32, trough_width: i32) -> f64
    { 
        let client_width = max(viewport_width, self.bounds.width);
        (viewport_width as f64) * (trough_width as f64) / (client_width as f64)
    }

    fn set_client_x(&mut self, viewport_width: i32, slider_x: f64, trough_width: i32)
    {
        let client_width = max(viewport_width, self.bounds.width);
        self.client_pos.x = ((slider_x * (client_width as f64)) / (trough_width as f64)) as i32;
    }
    
    fn v_scroll_bar_slider_y(&self, viewport_height: i32, trough_height: i32) -> f64
    {
        let client_height = max(viewport_height, self.bounds.height);
        (self.client_pos.y as f64) * (trough_height as f64) / (client_height as f64)
    }
    
    fn v_scroll_bar_slider_height(&self, viewport_height: i32, trough_height: i32) -> f64
    {
        let client_height = max(viewport_height, self.bounds.height);
        (viewport_height as f64) * (trough_height as f64) / (client_height as f64)
    }

    fn set_client_y(&mut self, viewport_height: i32, slider_y: f64, trough_height: i32)
    {
        let client_height = max(viewport_height, self.bounds.height);
        self.client_pos.y = ((slider_y * (client_height as f64)) / (trough_height as f64)) as i32;
    }
    
    fn set_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>)
    { self.change_flag_arc = flag_arc; }
}

impl Container for MockLayout
{
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if !self.widgets.is_empty() {
                    Some(WidgetIndexPair(self.widgets.len() - 1, 0))
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(i, 0)) => {
                match i.checked_sub(1) {
                    Some(j) if j < self.widgets.len() => Some(WidgetIndexPair(j, 0)),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if !self.widgets.is_empty() {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(i, 0)) => {
                match i.checked_add(1) {
                    Some(j) if j < self.widgets.len() => Some(WidgetIndexPair(j, 0)),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, 0) => {
                match self.widgets.get(i) {
                    Some(widget) => Some(&**widget),
                    None => None,
                }
            },
            _ => None,
        }
    }

    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, 0) => {
                match self.widgets.get_mut(i) {
                    Some(widget) => Some(&mut **widget),
                    None => None,
                }
            },
            _ => None,
        }
    }
    
    fn point_for_index_pair(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    { self.widgets.iter().enumerate().find(|p| p.1.bounds().to_f64_rect().contains(pos)).map(|p| WidgetIndexPair(p.0, 0)) }
}

impl PreferredSize for MockLayout
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    { self.preferred_size = size; }
}

impl Draw for MockLayout
{
    fn update_size(&mut self, _cairo_context: &CairoContext, _area_size: Size<Option<i32>>, _is_focused_window: bool)
    {}
    
    fn update_pos(&mut self, _cairo_context: &CairoContext, _area_pos: Pos<i32>, _is_focused_window: bool)
    {}

    fn draw(&self, _cairo_context: &CairoContext, _is_focused_window: bool)
    {}
}

impl CallOn for MockLayout
{
    fn call_on(&mut self, _client_context: &mut ClientContext, _queue_context: &mut QueueContext, _event: &Event) -> Option<Option<Event>>
    { Some(None) }
}

impl AsAny for MockLayout
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}