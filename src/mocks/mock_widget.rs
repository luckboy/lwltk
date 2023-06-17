//
// Copyright (c) 2022-2023 Łukasz Szpakowski
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
use crate::theme::*;
use crate::types::*;
use crate::widget::*;

pub(crate) struct MockWidget
{
    text: String,
    margin_bounds: Rect<i32>,
    bounds: Rect<i32>,
    client_pos: Pos<i32>,
    weight: u32,
    h_align: HAlign,
    v_align: VAlign,
    state: WidgetState,
    is_enabled: bool,
    is_focusable: bool,
    is_focused: bool,
    change_flag_arc: Arc<AtomicBool>,
    preferred_size: Size<Option<i32>>,
}

impl MockWidget
{
    pub(crate) fn new(s: &str) -> Self
    {
        MockWidget {
            text: String::from(s),
            margin_bounds: Rect::new(0, 0, 0, 0),
            bounds: Rect::new(0, 0, 0, 0),
            client_pos: Pos::new(0, 0),
            weight: 1,
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            state: WidgetState::None,
            is_enabled: true,
            is_focusable: true,
            is_focused: false,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            preferred_size: Size::new(None, None),
        }
    }
    
    pub(crate) fn text(&self) -> &str
    { self.text.as_str() }

    pub(crate) fn set_margin_bounds(&mut self, bounds: Rect<i32>)
    { self.margin_bounds = bounds; }

    pub(crate) fn set_bounds(&mut self, bounds: Rect<i32>)
    { self.bounds = bounds; }
    
    pub(crate) fn set_client_pos(&mut self, pos: Pos<i32>)
    { self.client_pos = pos; }

    pub(crate) fn set_weight(&mut self, weight: u32)
    { self.weight = weight; }
    
    pub(crate) fn set_h_align(&mut self, align: HAlign)
    { self.h_align = align; }

    pub(crate) fn set_v_align(&mut self, align: VAlign)
    { self.v_align = align; }

    pub(crate) fn set_enabling(&mut self, is_enabled: bool)
    { self.is_enabled = is_enabled; }

    pub(crate) fn set_focusable(&mut self, is_focusable: bool)
    { self.is_focusable = is_focusable; }

    pub(crate) fn set_change_flag(&mut self, is_changed: bool)
    { self.change_flag_arc.store(is_changed, Ordering::SeqCst); }
}

impl Widget for MockWidget
{
    fn margin_bounds(&self) -> Rect<i32>
    { self.margin_bounds }
    
    fn bounds(&self) -> Rect<i32>
    { self.bounds }

    fn weight(&self) -> u32
    { self.weight }
    
    fn h_align(&self) -> HAlign
    { self.h_align }
    
    fn v_align(&self) -> VAlign
    { self.v_align }

    fn state(&self) -> WidgetState
    { self.state }
    
    fn set_state(&mut self, state: WidgetState)
    { self.state = state; }

    fn is_enabled(&self) -> bool
    { self.is_enabled }
    
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
        let max_width = max(viewport_width, self.margin_bounds.width);
        if max_width > 0 {
            (self.client_pos.x as f64) * (trough_width as f64) / (max_width as f64)
        } else {
            0.0
        }
    }

    fn h_scroll_bar_slider_width(&self, viewport_width: i32, trough_width: i32) -> f64
    { 
        let max_width = max(viewport_width, self.margin_bounds.width);
        if max_width > 0 {
            (viewport_width as f64) * (trough_width as f64) / (max_width as f64)
        } else {
            trough_width as f64
        }
    }

    fn set_client_x(&mut self, viewport_width: i32, slider_x: f64, trough_width: i32)
    {
        let max_width = max(viewport_width, self.margin_bounds.width);
        if trough_width > 0 {
            self.client_pos.x = ((slider_x * (max_width as f64)) / (trough_width as f64)) as i32;
        } else {
            self.client_pos.x = 0;
        }
    }
    
    fn update_client_x(&mut self, viewport_width: i32) -> bool
    {
        if self.margin_bounds.width - self.client_pos.x < viewport_width {
            if self.margin_bounds.width > viewport_width {
                self.client_pos.x = self.margin_bounds.width - viewport_width;
                true
            } else {
                if self.client_pos.x != 0 {
                    self.client_pos.x = 0;
                    true
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
    
    fn v_scroll_bar_slider_y(&self, viewport_height: i32, trough_height: i32) -> f64
    {
        let max_height = max(viewport_height, self.margin_bounds.height);
        if max_height > 0 {
            (self.client_pos.y as f64) * (trough_height as f64) / (max_height as f64)
        } else {
            0.0
        }
    }
    
    fn v_scroll_bar_slider_height(&self, viewport_height: i32, trough_height: i32) -> f64
    {
        let max_height = max(viewport_height, self.margin_bounds.height);
        if max_height > 0 {
            (viewport_height as f64) * (trough_height as f64) / (max_height as f64)
        } else {
            trough_height as f64
        }
    }

    fn set_client_y(&mut self, viewport_height: i32, slider_y: f64, trough_height: i32)
    {
        let max_height = max(viewport_height, self.margin_bounds.height);
        if trough_height > 0 {
            self.client_pos.y = ((slider_y * (max_height as f64)) / (trough_height as f64)) as i32;
        } else {
            self.client_pos.y = 0;
        }
    }

    fn update_client_y(&mut self, viewport_height: i32) -> bool
    {
        if self.margin_bounds.height - self.client_pos.y < viewport_height {
            if self.margin_bounds.height > viewport_height {
                self.client_pos.y = self.margin_bounds.height - viewport_height;
                true
            } else {
                if self.client_pos.y != 0 {
                    self.client_pos.y = 0;
                    true
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
    
    fn set_only_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>)
    { self.change_flag_arc = flag_arc; }
}

impl Container for MockWidget
{}

impl PreferredSize for MockWidget
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    { self.preferred_size = size; }
}

impl Draw for MockWidget
{
    fn update_size(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_size: Size<Option<i32>>) -> Result<(), CairoError>
    { Ok(()) }
    
    fn update_pos(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_bounds: Rect<i32>) -> Result<(), CairoError>
    { Ok(()) }

    fn draw(&self, _cairo_context: &CairoContext, _theme: &dyn Theme, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
}

impl CallOn for MockWidget
{
    fn call_on(&mut self, _client_context: &mut ClientContext, _queue_context: &mut QueueContext, _event: &Event) -> Option<Option<Event>>
    { Some(None) }
}

impl AsAny for MockWidget
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}
