//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::cmp::max;
use std::cmp::min;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use crate::as_any::*;
use crate::call_on::*;
use crate::client_context::*;
use crate::container::*;
use crate::draw::*;
use crate::events::*;
use crate::image::*;
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::text::*;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;

pub struct Button
{
    margin_bounds: Rect<i32>,
    bounds: Rect<i32>,
    client_pos: Pos<i32>,
    weight: u32,
    h_align: HAlign,
    v_align: VAlign,
    state: WidgetState,
    is_enabled: bool,
    is_focused: bool,
    change_flag_arc: Arc<AtomicBool>,
    preferred_size: Size<Option<i32>>,
    call_on_fun: CallOnFun,
    image: Option<Image>,
    text: Text,
}

impl Button
{
    fn new_with_opt_icon(opt_icon: Option<ButtonIcon>, s: &str) -> Self
    {
        Button {
            margin_bounds: Rect::new(0, 0, 0, 0),
            bounds: Rect::new(0, 0, 0, 0),
            client_pos: Pos::new(0, 0),
            weight: 0,
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            state: WidgetState::None,
            is_enabled: true,
            is_focused: false,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            preferred_size: Size::new(None, None),
            call_on_fun: CallOnFun::new(),
            image: opt_icon.map(|i| {
                    Image::new(move |theme| {
                            theme.button_icon_size()
                    }, move |cairo_context, theme, pos, state, is_enabled, is_focused, is_focused_window| {
                            theme.draw_button_icon(cairo_context, pos, i, state, is_enabled, is_focused, is_focused_window)
                    })
            }),
            text: Text::new(s, TextAlign::Center),
        }
    }

    pub fn new(s: &str) -> Self
    { Button::new_with_opt_icon(None, s) }

    pub fn new_with_icon(icon: ButtonIcon, s: &str) -> Self
    { Button::new_with_opt_icon(Some(icon), s) }

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

    pub fn set_dyn_icon_image(&mut self, size_f: Box<dyn Fn(&dyn Theme) -> Size<i32> + Send + Sync + 'static>, drawing_f: Box<dyn Fn(&CairoContext, &dyn Theme, Pos<i32>, WidgetState, bool, bool, bool) -> Result<(), CairoError> + Send + Sync + 'static>)
    {
        match &mut self.image {
            Some(image) => {
                image.size_fun = size_f;
                image.drawing_fun = drawing_f;
            },
            None => {
                self.image = Some(Image::new(size_f, drawing_f));
            },
        }
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
    
    pub fn set_icon_image<F, G>(&mut self, size_f: F, drawing_f: G)
        where F: Fn(&dyn Theme) -> Size<i32> + Send + Sync + 'static,
              G: Fn(&CairoContext, &dyn Theme, Pos<i32>, WidgetState, bool, bool, bool) -> Result<(), CairoError> + Send + Sync + 'static
    { self.set_dyn_icon_image(Box::new(size_f), Box::new(drawing_f)) }

    pub fn set_icon(&mut self, icon: ButtonIcon)
    {
        self.set_icon_image(move |theme| {
                theme.button_icon_size()
        }, move |cairo_context, theme, pos, state, is_enabled, is_focused, is_focused_window| {
                theme.draw_button_icon(cairo_context, pos, icon, state, is_enabled, is_focused, is_focused_window)
        })
    }
    
    pub fn unset_icon(&mut self)
    {
        self.image = None;
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
    
    pub fn text(&self) -> &str
    { self.text.text.as_str() }
    
    pub fn set_text(&mut self, s: &str)
    {
        self.text.text = String::from(s);
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }

    pub fn text_align(&self) -> TextAlign
    { self.text.align }
    
    pub fn set_text_align(&mut self, align: TextAlign)
    {
        let old_align = self.text.align;
        self.text.align = align;
        if old_align != self.text.align {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn ellipsize_count(&self) -> Option<usize>
    { self.text.ellipsize_count }
    
    pub fn set_ellipsize_count(&mut self, ellipsize_count: Option<usize>)
    {
        let old_ellipsize_count = self.text.ellipsize_count;
        self.text.ellipsize_count = ellipsize_count;
        if old_ellipsize_count != self.text.ellipsize_count {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn is_trimmed(&self) -> bool
    { self.text.is_trimmed }

    pub fn set_trim(&mut self, is_trimmed: bool)
    { self.text.is_trimmed = is_trimmed; }
}

impl Widget for Button
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
    {
        let old_state = self.state;
        self.state = state;
        if old_state != self.state {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    fn is_enabled(&self) -> bool
    { self.is_enabled }
    
    fn is_focusable(&self) -> bool
    { self.is_enabled }
    
    fn is_focused(&self) -> bool
    { self.is_enabled && self.is_focused }
    
    fn set_focus(&mut self, is_focused: bool) -> bool
    {
        if self.is_enabled {
            let old_focus_flag = self.is_focused;
            self.is_focused = is_focused;
            if old_focus_flag != self.is_focused {
                self.change_flag_arc.store(true, Ordering::SeqCst);
            }
            true
        } else {
            false
        }
    }
    
    fn h_scroll_bar_slider_x(&self, viewport_width: i32, trough_width: i32) -> f64
    { h_scroll_bar_slider_x(self.client_pos.x, self.margin_bounds.width, viewport_width, trough_width) }

    fn h_scroll_bar_slider_width(&self, viewport_width: i32, trough_width: i32) -> f64
    { h_scroll_bar_slider_width(self.margin_bounds.width, viewport_width, trough_width) }

    fn set_client_x(&mut self, viewport_width: i32, slider_x: f64, trough_width: i32)
    {
        let old_client_x = self.client_pos.x;
        set_client_x(&mut self.client_pos.x, self.margin_bounds.width, viewport_width, slider_x, trough_width);
        if old_client_x != self.client_pos.x {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
    
    fn update_client_x(&mut self, viewport_width: i32) -> bool
    { update_client_x(&mut self.client_pos.x, self.margin_bounds.width, viewport_width) }
    
    fn v_scroll_bar_slider_y(&self, viewport_height: i32, trough_height: i32) -> f64
    { v_scroll_bar_slider_y(self.client_pos.y, self.margin_bounds.height, viewport_height, trough_height) }
    
    fn v_scroll_bar_slider_height(&self, viewport_height: i32, trough_height: i32) -> f64
    { v_scroll_bar_slider_height(self.margin_bounds.height, viewport_height, trough_height) }

    fn set_client_y(&mut self, viewport_height: i32, slider_y: f64, trough_height: i32)
    {
        let old_client_y = self.client_pos.y;
        set_client_y(&mut self.client_pos.y, self.margin_bounds.height, viewport_height, slider_y, trough_height);
        if old_client_y != self.client_pos.y {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    fn update_client_y(&mut self, viewport_height: i32) -> bool
    { update_client_y(&mut self.client_pos.y, self.margin_bounds.height, viewport_height) }
    
    fn set_only_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>)
    { self.change_flag_arc = flag_arc; }
}

impl Container for Button
{}

impl PreferredSize for Button
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

impl Draw for Button
{
    fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        let inner_area_size = inner_opt_size(area_size, theme.button_margin_edges());
        let mut padding_area_size = inner_opt_size(inner_area_size, theme.button_padding_edges());
        let mut padding_size = Size::new(0, 0);
        match &self.image {
            Some(image) => {
                let tmp_size = (image.size_fun)(theme);
                padding_size.width += tmp_size.width;
                padding_size.height = tmp_size.height;
                match padding_area_size.width {
                    Some(padding_area_width) => {
                        if padding_area_width > theme.button_sep_width() + padding_size.width {
                            padding_area_size.width = Some(padding_area_width - (theme.button_sep_width() + padding_size.width));
                        } else {
                            padding_area_size.width = Some(0);
                        }
                    },
                    None => (),
                }
                padding_size.width += theme.button_sep_width();
            },
            None => (),
        }
        self.text.update_size(cairo_context, padding_area_size, |cairo_context| {
                theme.set_button_font(cairo_context)
        })?;
        padding_size.width += self.text.max_line_width();
        padding_size.height = max(padding_size.height, self.text.line_height * self.text.lines.len() as i32);
        self.bounds.set_size(outer_size(padding_size, theme.button_padding_edges()));
        self.bounds.set_size(max_size_for_opt_size(self.bounds.size(), self.preferred_size));
        self.margin_bounds.set_size(outer_size(self.bounds.size(), theme.button_margin_edges()));
        self.margin_bounds.set_size(size_for_h_align_and_v_align(self.margin_bounds.size(), area_size, self.h_align, self.v_align));
        self.bounds.set_size(inner_size(self.margin_bounds.size(), theme.button_margin_edges()));
        Ok(())
    }
    
    fn update_pos(&mut self, _cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>
    {
        self.margin_bounds.set_pos(pos_for_h_align_and_v_align(self.margin_bounds.size(), area_bounds, self.h_align, self.v_align));
        self.margin_bounds.x -= self.client_pos.x;
        self.margin_bounds.y -= self.client_pos.y;
        self.bounds.set_pos(inner_pos(self.margin_bounds, theme.button_margin_edges()));
        Ok(())
    }

    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    {
        cairo_context.save()?;
        cairo_context.rectangle(self.bounds.x as f64, self.bounds.y as f64,  self.bounds.width as f64, self.bounds.height as f64);
        cairo_context.clip();
        theme.draw_button_bg(cairo_context, self.bounds, self.state, self.is_enabled, self.is_focused(), is_focused_window)?;
        let padding_bounds = inner_rect(self.bounds, theme.button_padding_edges());
        cairo_context.rectangle(padding_bounds.x as f64, padding_bounds.y as f64,  padding_bounds.width as f64, padding_bounds.height as f64);
        cairo_context.clip();
        let mut x = padding_bounds.x;
        match &self.image {
            Some(image) => {
                let tmp_size = (image.size_fun)(theme);
                let area_width = min(tmp_size.width, padding_bounds.width);
                let area_bounds = Rect::new(padding_bounds.x, padding_bounds.y, area_width, padding_bounds.height);
                image.draw(cairo_context, theme, area_bounds, self.state, self.is_enabled, self.is_focused, is_focused_window)?;
                x += area_width + theme.button_sep_width();
                if x > padding_bounds.x + padding_bounds.width {
                    x = padding_bounds.x + padding_bounds.width;
                }
            },
            None => (),
        }
        let area_bounds = Rect::new(x, padding_bounds.y, padding_bounds.width - (x - padding_bounds.x), padding_bounds.height);
        self.text.draw(cairo_context, area_bounds, |cairo_context| {
                theme.set_button_font(cairo_context)
        }, |cairo_context, pos, s| {
                theme.draw_button_text(cairo_context, pos, s, self.state, self.is_enabled, self.is_focused(), is_focused_window)
        })?;
        cairo_context.restore()?;
        Ok(())
    }
}

impl CallOn for Button
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

impl AsAny for Button
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}
