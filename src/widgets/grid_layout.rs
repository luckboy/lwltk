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
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;
use crate::widgets::grid_layout_widgets::*;

pub struct GridLayout
{
    bounds: Rect<i32>,
    client_pos: Pos<i32>,
    weight: u32,
    orient: Orient,
    h_align: HAlign,
    v_align: VAlign,
    state: WidgetState,
    is_enabled: bool,
    change_flag_arc: Arc<AtomicBool>,
    preferred_size: Size<Option<i32>>,
    call_on_fun: CallOnFun,
    widgets: GridLayoutWidgets,
}

impl GridLayout
{
    pub fn new(max_column_count: usize) -> Self
    {
        GridLayout {
            bounds: Rect::new(0, 0, 0, 0),
            client_pos: Pos::new(0, 0),
            weight: 0,
            orient: Orient::Horizontal,
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            state: WidgetState::None,
            is_enabled: true,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            preferred_size: Size::new(None, None),
            call_on_fun: CallOnFun::new(),
            widgets: GridLayoutWidgets::new(max_column_count),
        }
    }

    pub fn orient(&self) -> Orient
    { self.orient }

    pub fn set_orient(&mut self, orient: Orient)
    {
        let old_orient = self.orient;
        self.orient = orient;
        if old_orient != self.orient {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
    
    pub fn set_weight(&mut self, weight: u32)
    {
        let old_weight = self.weight;
        self.weight = weight;
        if old_weight != self.weight {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
    
    pub fn set_h_align(&mut self, align: HAlign)
    {
        let old_h_align = self.h_align;
        self.h_align = align;
        if old_h_align != self.h_align {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn set_v_align(&mut self, align: VAlign)
    {
        let old_v_align = self.v_align;
        self.v_align = align;
        if old_v_align != self.v_align {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn set_enabled(&mut self, is_enabled: bool)
    {
        let old_enabled_flag = self.is_enabled;
        self.is_enabled = is_enabled;
        if old_enabled_flag != self.is_enabled {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn set_dyn_on(&mut self, f: Box<dyn FnMut(&mut ClientContext, &mut QueueContext, &Event) -> Option<EventOption> + Send + Sync + 'static>)
    { self.call_on_fun.fun = f; }

    pub fn set_on<F>(&mut self, f: F)
        where F: FnMut(&mut ClientContext, &mut QueueContext, &Event) -> Option<EventOption> + Send + Sync + 'static
    { self.set_dyn_on(Box::new(f)) }

    pub fn max_column_count(&self) -> usize
    { self.widgets.max_column_count }

    pub fn set_max_column_count(&mut self, count: usize)
    {
        let old_max_column_count = self.widgets.max_column_count;
        self.widgets.max_column_count = count;
        if old_max_column_count != self.widgets.max_column_count {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
    
    pub fn add_dyn(&mut self, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        match self.widgets.add_dyn(widget) {
            Some(idx_pair) => {
                self.change_flag_arc.store(true, Ordering::SeqCst);
                Some(idx_pair)
            },
            None => None,
        }
    }

    pub fn add<T: Widget + 'static>(&mut self, widget: T) -> Option<WidgetIndexPair>
    { self.add_dyn(Box::new(widget)) }

    pub fn add_empty_row(&mut self) -> bool
    {
        if self.widgets.add_empty_row() {
            self.change_flag_arc.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    
    pub fn insert_dyn(&mut self, idx_pair: WidgetIndexPair, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        match self.widgets.insert_dyn(idx_pair, widget) {
            Some(idx_pair) => {
                self.change_flag_arc.store(true, Ordering::SeqCst);
                Some(idx_pair)
            },
            None => None,
        }
    }

    pub fn insert<T: Widget + 'static>(&mut self, idx_pair: WidgetIndexPair, widget: T) -> Option<WidgetIndexPair>
    { self.insert_dyn(idx_pair, Box::new(widget)) }

    pub fn remove(&mut self, idx_pair: WidgetIndexPair) -> Option<Box<dyn Widget>>
    {
        match self.widgets.remove(idx_pair) {
            Some(mut widget) => {
                self.change_flag_arc.store(true, Ordering::SeqCst);
                widget.set_change_flag_arc(Arc::new(AtomicBool::new(false)));
                Some(widget)
            },
            None => None,
        }
    }

    pub fn remove_last(&mut self) -> Option<Box<dyn Widget>>
    {
        match self.widgets.remove_last() {
            Some(mut widget) => {
                self.change_flag_arc.store(true, Ordering::SeqCst);
                widget.set_change_flag_arc(Arc::new(AtomicBool::new(false)));
                Some(widget)
            },
            None => None,
        }
    }

    pub fn clear(&mut self)
    {
        self.widgets.widgets.clear();
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
}

impl Widget for GridLayout
{
    fn margin_bounds(&self) -> Rect<i32>
    { self.bounds }
    
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
    
    fn h_scroll_bar_slider_x(&self, viewport_width: i32, trough_width: i32) -> f64
    { h_scroll_bar_slider_x(self.client_pos.x, self.bounds.width, viewport_width, trough_width) }

    fn h_scroll_bar_slider_width(&self, viewport_width: i32, trough_width: i32) -> f64
    { h_scroll_bar_slider_width(self.bounds.width, viewport_width, trough_width) }

    fn set_client_x(&mut self, viewport_width: i32, slider_x: f64, trough_width: i32)
    {
        let old_client_x = self.client_pos.x;
        set_client_x(&mut self.client_pos.x, self.bounds.width, viewport_width, slider_x, trough_width);
        if old_client_x != self.client_pos.x {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
    
    fn update_client_x(&mut self, viewport_width: i32) -> bool
    { update_client_x(&mut self.client_pos.x, self.bounds.width, viewport_width) }
    
    fn v_scroll_bar_slider_y(&self, viewport_height: i32, trough_height: i32) -> f64
    { v_scroll_bar_slider_y(self.client_pos.y, self.bounds.height, viewport_height, trough_height) }
    
    fn v_scroll_bar_slider_height(&self, viewport_height: i32, trough_height: i32) -> f64
    { v_scroll_bar_slider_height(self.bounds.height, viewport_height, trough_height) }

    fn set_client_y(&mut self, viewport_height: i32, slider_y: f64, trough_height: i32)
    {
        let old_client_y = self.client_pos.y;
        set_client_y(&mut self.client_pos.y, self.bounds.height, viewport_height, slider_y, trough_height);
        if old_client_y != self.client_pos.y {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    fn update_client_y(&mut self, viewport_height: i32) -> bool
    { update_client_y(&mut self.client_pos.y, self.bounds.height, viewport_height) }
    
    fn set_only_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>)
    { self.change_flag_arc = flag_arc; }
}

impl Container for GridLayout
{
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { self.widgets.prev(idx_pair) }

    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { self.widgets.next(idx_pair) }

    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    { self.widgets.dyn_widget(idx_pair) }

    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    { self.widgets.dyn_widget_mut(idx_pair) }
    
    fn point_for_index_pair(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    { self.widgets.point(pos, self.orient) }
}

impl PreferredSize for GridLayout
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    {
        let old_preferred_size = self.preferred_size;
        self.preferred_size = size;
        if old_preferred_size != self.preferred_size {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
}

impl Draw for GridLayout
{
    fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        self.widgets.update_size(cairo_context, theme, area_size, self.orient, self.h_align, self.v_align, self.preferred_size)?;
        self.bounds.set_size(self.widgets.size(area_size, self.orient, self.h_align, self.v_align, self.preferred_size));
        Ok(())
    }
    
    fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>
    {
        let mut area_bounds2 = area_bounds;
        area_bounds2.x -= self.client_pos.x;
        area_bounds2.y -= self.client_pos.y;
        self.widgets.update_pos(cairo_context, theme, area_bounds2, self.orient, self.h_align, self.v_align, self.preferred_size)?;
        self.bounds.set_pos(pos_for_h_align_and_v_align(self.bounds.size(), area_bounds2, self.h_align, self.v_align));
        Ok(())
    }

    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    {
        cairo_context.save()?;
        cairo_context.rectangle(self.bounds.x as f64, self.bounds.y as f64, self.bounds.width as f64, self.bounds.height as f64);
        cairo_context.clip();
        theme.draw_linear_layout_bg(cairo_context, self.bounds, self.orient, self.state, self.is_enabled, is_focused_window)?;
        self.widgets.draw(cairo_context, theme, is_focused_window)?;
        cairo_context.restore()?;
        Ok(())
    }
}

impl CallOn for GridLayout
{
    fn call_on(&mut self, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Event>>
    {
        let default_event = if let Some(tmp_default_event) = default_widget_on(self, client_context, queue_context, event)? {
            tmp_default_event
        } else {
            None
        };
        self.call_on_fun.call_on(client_context, queue_context, event, default_event)
    }
}

impl AsAny for GridLayout
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}
