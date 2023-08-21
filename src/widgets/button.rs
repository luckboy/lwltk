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
    { Self::new_with_opt_icon(None, s) }

    pub fn new_with_icon(icon: ButtonIcon, s: &str) -> Self
    { Self::new_with_opt_icon(Some(icon), s) }

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
        cairo_context.rectangle(self.bounds.x as f64, self.bounds.y as f64, self.bounds.width as f64, self.bounds.height as f64);
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

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;

    #[test]
    fn test_button_updates_size_and_position()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_with_icon_updates_size_and_position()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_sep_width(4);
        theme.set_button_font_size(16.0);
        theme.set_button_icon_size(Size::new(26, 24));
        let mut button = Button::new_with_icon(ButtonIcon::Ok, "OK");
        theme.set_button_font(&cairo_context).unwrap();
        let o = cairo_context.text_extents("O").unwrap().x_advance;
        let k = cairo_context.text_extents("K").unwrap().x_advance;
        let text_width = o + k;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + 26 + 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + 24 + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_with_custom_icon_updates_size_and_position()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_sep_width(4);
        theme.set_button_font_size(16.0);
        let mut button = Button::new("OK");
        button.set_icon_image(|_theme| Size::new(26, 24), |_cairo_context, _theme, _pos, _state, _is_enabled, _is_focused, _is_focused_window| Ok(()));
        theme.set_button_font(&cairo_context).unwrap();
        let o = cairo_context.text_extents("O").unwrap().x_advance;
        let k = cairo_context.text_extents("K").unwrap().x_advance;
        let text_width = o + k;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + 26 + 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + 24 + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_greater_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_less_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 - 10;
        let area_size = Size::new(Some(area_width), None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + ((text_width - n).ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) * 2 + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_greater_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        let preferred_width = 4 + (text_width.ceil() as i32) + 5 + 10;
        button.set_preferred_size(Size::new(Some(preferred_width), None));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5 + 10;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_less_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        let preferred_width = 4 + (text_width.ceil() as i32) + 5 - 10;
        button.set_preferred_size(Size::new(Some(preferred_width), None));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_less_area_width_and_greater_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        let preferred_width = 4 + (text_width.ceil() as i32) + 5 + 10;
        button.set_preferred_size(Size::new(Some(preferred_width), None));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 - 10;
        let area_size = Size::new(Some(area_width), None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5 - 10;
        let expected_height = 2 + (font_height.ceil() as i32) * 2 + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_greater_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(None, Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_less_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 - 10;
        let area_size = Size::new(None, Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3 - 10;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_greater_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        let preferred_height = 2 + (font_height.ceil() as i32) + 3 + 10;
        button.set_preferred_size(Size::new(None, Some(preferred_height)));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5 ;
        let expected_height = 2 + (font_height.ceil() as i32) + 3 + 10;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_less_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        let preferred_height = 2 + (font_height.ceil() as i32) + 3 - 10;
        button.set_preferred_size(Size::new(None, Some(preferred_height)));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5 ;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, button.margin_bounds.height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_less_area_height_and_greater_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        let preferred_height = 2 + (font_height.ceil() as i32) + 3 + 10;
        button.set_preferred_size(Size::new(None, Some(preferred_height)));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 - 10;
        let area_size = Size::new(None, Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3 - 10;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, button.margin_bounds.width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }
    
    #[test]
    fn test_button_updates_size_and_position_for_left_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_h_align(HAlign::Left);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }    

    #[test]
    fn test_button_updates_size_and_position_for_center_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_h_align(HAlign::Center);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6 + 5;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }    

    #[test]
    fn test_button_updates_size_and_position_for_right_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_h_align(HAlign::Right);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6 + 10;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_fill_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_h_align(HAlign::Fill);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5 + 10;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_top_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_v_align(VAlign::Top);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_center_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_v_align(VAlign::Center);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7 + 5;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_bottom_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_v_align(VAlign::Bottom);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7 + 10;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }

    #[test]
    fn test_button_updates_size_and_position_for_fill_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(1, 2, 3, 4));
        theme.set_button_padding_edges(Edges::new(2, 3, 4, 5));
        theme.set_button_font_size(16.0);
        let mut button = Button::new("Button");
        button.set_v_align(VAlign::Fill);
        theme.set_button_font(&cairo_context).unwrap();
        let b = cairo_context.text_extents("B").unwrap().x_advance;
        let u = cairo_context.text_extents("u").unwrap().x_advance;
        let t = cairo_context.text_extents("t").unwrap().x_advance;
        let o = cairo_context.text_extents("o").unwrap().x_advance;
        let n = cairo_context.text_extents("n").unwrap().x_advance;
        let text_width = b + u + t + t + o + n;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 3 + 4 + (text_width.ceil() as i32) + 5 + 4 + 10;
        let area_height = 1 + 2 + (font_height.ceil() as i32) + 3 + 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        match button.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_width = 4 + (text_width.ceil() as i32) + 5;
        let expected_height = 2 + (font_height.ceil() as i32) + 3 + 10;
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        let expected_margin_width = 3 + expected_width + 4;
        let expected_margin_height = 1 + expected_height + 2;
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
        let area_bounds = Rect::new(6, 7, area_width, area_height);
        match button.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_margin_x = 6;
        let expected_margin_y = 7;
        assert_eq!(Pos::new(expected_margin_x, expected_margin_y), button.margin_bounds.pos());
        let expected_x = expected_margin_x + 3;
        let expected_y = expected_margin_y + 1;
        assert_eq!(Pos::new(expected_x, expected_y), button.bounds.pos());
        assert_eq!(Size::new(expected_width, expected_height), button.bounds.size());
        assert_eq!(Size::new(expected_margin_width, expected_margin_height), button.margin_bounds.size());
    }
}
