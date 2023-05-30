//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use cairo::FontSlant;
use cairo::FontWeight;
use crate::theme::*;
use crate::types::*;

pub struct DefaultTheme
{
    window_content_bg_color: Color,
    window_content_border_color: Color,
    unfocused_window_content_border_color: Color,
    button_bg_color: Color,
    hover_button_bg_color: Color,
    active_button_bg_color: Color,
    button_border_color: Color,
    focused_button_border_color: Color,
    button_border_color_for_unfocused_window: Color,
    button_fg_color: Color,
    disabled_button_fg_color: Color,
    button_fg_color_for_unfocused_window: Color,
}

impl DefaultTheme
{
    pub fn new() -> Self
    {
        DefaultTheme {
            window_content_bg_color: Color::new_from_rgb_u32(0xeeeeee),
            window_content_border_color: Color::new_from_rgb_u32(0x222222),
            unfocused_window_content_border_color: Color::new_from_rgb_u32(0x888888),
            button_bg_color: Color::new_from_rgb_u32(0xaaaaaa),
            hover_button_bg_color: Color::new_from_rgb_u32(0xbbbbbb),
            active_button_bg_color: Color::new_from_rgb_u32(0xcccccc),
            button_border_color: Color::new_from_rgb_u32(0x888888),
            focused_button_border_color: Color::new_from_rgb_u32(0x222222),
            button_border_color_for_unfocused_window: Color::new_from_rgb_u32(0x888888),
            button_fg_color: Color::new_from_rgb_u32(0x222222),
            disabled_button_fg_color: Color::new_from_rgb_u32(0x888888),
            button_fg_color_for_unfocused_window: Color::new_from_rgb_u32(0x888888),
        }
    }
    
    pub fn window_content_bg_color(&self) -> Color
    { self.window_content_bg_color }
    
    pub fn set_window_content_bg_color(&mut self, color: Color)
    { self.window_content_bg_color = color; }

    pub fn window_content_border_color(&self) -> Color
    { self.window_content_border_color }

    pub fn set_window_content_border_color(&mut self, color: Color)
    { self.window_content_border_color = color; }
    
    pub fn unfocused_window_content_border_color(&self) -> Color
    { self.unfocused_window_content_border_color }
    
    pub fn set_unfocused_window_content_border_color(&mut self, color: Color)
    { self.unfocused_window_content_border_color = color; }

    pub fn button_bg_color(&self) -> Color
    { self.button_bg_color }
    
    pub fn set_button_bg_color(&mut self, color: Color)
    { self.button_bg_color = color; }

    pub fn hover_button_bg_color(&self) -> Color
    { self.hover_button_bg_color }
    
    pub fn set_hover_button_bg_color(&mut self, color: Color)
    { self.hover_button_bg_color = color; }

    pub fn active_button_bg_color(&self) -> Color
    { self.active_button_bg_color }

    pub fn set_active_button_bg_color(&mut self, color: Color)
    { self.active_button_bg_color = color; }

    pub fn button_border_color(&self) -> Color
    { self.button_border_color }

    pub fn set_button_border_color(&mut self, color: Color)
    { self.button_border_color = color; }
    
    pub fn focused_button_border_color(&self) -> Color
    { self.focused_button_border_color }

    pub fn set_focused_button_border_color(&mut self, color: Color)
    { self.focused_button_border_color = color;  }
    
    pub fn button_border_color_for_unfocused_window(&self) -> Color
    { self.focused_button_border_color }
    
    pub fn set_button_border_color_for_unfocused_window(&mut self, color: Color)
    { self.focused_button_border_color = color; }

    pub fn button_fg_color(&self) -> Color
    { self.button_fg_color }

    pub fn set_button_fg_color(&mut self, color: Color)
    { self.button_fg_color = color; }

    pub fn disabled_button_fg_color(&self) -> Color
    { self.disabled_button_fg_color }

    pub fn set_disabled_button_fg_color(&mut self, color: Color)
    { self.disabled_button_fg_color = color; }
    
    pub fn button_fg_color_for_unfocused_window(&self) -> Color
    { self.button_fg_color_for_unfocused_window }

    pub fn set_button_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.button_fg_color_for_unfocused_window = color; }
}

impl Theme for DefaultTheme
{
    fn set_cairo_context(&self, cairo_context: &CairoContext, scale: i32) -> Result<(), CairoError>
    {
        cairo_context.scale(scale as f64, scale as f64);
        cairo_context.set_line_width(2.0);
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        Ok(())
    }

    fn toplevel_window_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 4, 4) }

    fn toplevel_window_corners(&self) -> Corners<i32>
    { Corners::new(8, 8, 8, 8, 8, 8, 8, 8) }
    
    fn draw_toplevel_window_content_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool) -> Result<(), CairoError>
    {
        cairo_context.set_source_rgba(self.window_content_bg_color.red, self.window_content_bg_color.green, self.window_content_bg_color.blue, self.window_content_bg_color.alpha);
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64);
        cairo_context.fill()?;
        cairo_context.set_source_rgba(self.window_content_bg_color.red, self.window_content_bg_color.green, self.window_content_bg_color.blue, self.window_content_bg_color.alpha);
        if is_focused_window {
            cairo_context.set_source_rgba(self.window_content_border_color.red, self.window_content_border_color.green, self.window_content_border_color.blue, self.window_content_border_color.alpha);
        } else {
            cairo_context.set_source_rgba(self.unfocused_window_content_border_color.red, self.unfocused_window_content_border_color.green, self.unfocused_window_content_border_color.blue, self.unfocused_window_content_border_color.alpha);
        }
        cairo_context.move_to((bounds.x as f64) + 1.0, bounds.y as f64);
        cairo_context.line_to((bounds.x as f64) + 1.0, (bounds.y as f64) + (bounds.height as f64) - 1.0);
        cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, (bounds.y as f64) + (bounds.height as f64) - 1.0);
        cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, bounds.y as f64);
        cairo_context.stroke()?;
        Ok(())
    }

    fn button_margin_egdes(&self) -> Edges<i32>
    { Edges::new(4, 4, 4, 4) }

    fn button_padding_egdes(&self) -> Edges<i32>
    { Edges::new(4, 4, 4, 4) }
    
    fn draw_button_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, _is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            match state {
                WidgetState::None => cairo_context.set_source_rgba(self.button_bg_color.red, self.button_bg_color.green, self.button_bg_color.blue, self.button_bg_color.alpha),
                WidgetState::Hover => cairo_context.set_source_rgba(self.hover_button_bg_color.red, self.hover_button_bg_color.green, self.hover_button_bg_color.blue, self.hover_button_bg_color.alpha),
                WidgetState::Active => cairo_context.set_source_rgba(self.active_button_bg_color.red, self.active_button_bg_color.green, self.active_button_bg_color.blue, self.active_button_bg_color.alpha),
            }
        } else {
            cairo_context.set_source_rgba(self.button_bg_color.red, self.button_bg_color.green, self.button_bg_color.blue, self.button_bg_color.alpha);
        }
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
        cairo_context.fill()?;
        if is_focused_window {
            if is_focused {
                cairo_context.set_source_rgba(self.focused_button_border_color.red, self.focused_button_border_color.green, self.focused_button_border_color.blue, self.focused_button_border_color.alpha);
            } else {
                cairo_context.set_source_rgba(self.button_border_color.red, self.button_border_color.green, self.button_border_color.blue, self.button_border_color.alpha);
            }
        } else {
            cairo_context.set_source_rgba(self.button_border_color_for_unfocused_window.red, self.button_border_color_for_unfocused_window.green, self.button_border_color_for_unfocused_window.blue, self.button_border_color_for_unfocused_window.alpha);
        }
        cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0); 
        Ok(())
    }

    fn set_button_font(&self, _cairo_context: &CairoContext) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_button_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
        if is_focused_window {
            if is_enabled {
                cairo_context.set_source_rgba(self.button_fg_color.red, self.button_fg_color.green, self.button_fg_color.blue, self.button_fg_color.alpha);
            } else {
                cairo_context.set_source_rgba(self.disabled_button_fg_color.red, self.disabled_button_fg_color.green, self.disabled_button_fg_color.blue, self.disabled_button_fg_color.alpha);
            }
        } else {
            cairo_context.set_source_rgba(self.button_fg_color_for_unfocused_window.red, self.button_fg_color_for_unfocused_window.green, self.button_fg_color_for_unfocused_window.blue, self.button_fg_color_for_unfocused_window.alpha);
        }
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }
}
