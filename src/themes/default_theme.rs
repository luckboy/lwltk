//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::f64::consts::PI;
use cairo::FontSlant;
use cairo::FontWeight;
use cairo::LinearGradient;
use crate::image::*;
use crate::theme::*;
use crate::themes::default_button_icons::*;
use crate::themes::default_title_button_icons::*;
use crate::types::*;
use crate::utils::*;

const CHECK_SIZE: i32 = 12;

const RADIO_SIZE: i32 = 12;

pub struct DefaultTheme
{
    // Background colors.
    bg_color: Color,
    light_bg_color: Color,
    dark_bg_color1: Color,
    dark_bg_color2: Color,
    selected_bg_color: Color,
    title_bg_color1: Color,
    title_bg_color2: Color,
    title_bg_color1_for_unfocused_window: Color,
    title_bg_color2_for_unfocused_window: Color,
    // Hover color and active color.
    hover_color: Color,
    active_color: Color,
    // Border colors.
    border_color: Color,
    disabled_border_color: Color,
    focused_border_color: Color,
    current_border_color: Color,
    border_color_for_unfocused_window: Color,
    disabled_border_color_for_unfocused_window: Color,
    focused_border_color_for_unfocused_window: Color,
    current_border_color_for_unfocused_window: Color,
    // Foreground colors.
    fg_color: Color,
    disabled_fg_color: Color,
    white_fg_color: Color,
    red_fg_color: Color,
    green_fg_color: Color,
    blue_fg_color: Color,
    cyan_fg_color: Color,
    purple_fg_color: Color,
    yellow_fg_color: Color,
    title_fg_color: Color,
    fg_color_for_unfocused_window: Color,
    disabled_fg_color_for_unfocused_window: Color,
    white_fg_color_for_unfocused_window: Color,
    red_fg_color_for_unfocused_window: Color,
    green_fg_color_for_unfocused_window: Color,
    blue_fg_color_for_unfocused_window: Color,
    cyan_fg_color_for_unfocused_window: Color,
    purple_fg_color_for_unfocused_window: Color,
    yellow_fg_color_for_unfocused_window: Color,
}

fn set_cairo_gradient(cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient, color1: Color, color2: Color) -> Result<(), CairoError>
{
    let gradient = match orient {
        Orient::Horizontal => LinearGradient::new(0.0, bounds.y as f64, 0.0, (bounds.y + bounds.height) as f64),
        Orient::Vertical => LinearGradient::new(bounds.x as f64, 0.0, (bounds.x + bounds.width) as f64, 0.0),
    };
    gradient.add_color_stop_rgba(0.0, color1.red, color1.green, color1.blue, color1.alpha);
    gradient.add_color_stop_rgba(0.5, color2.red, color2.green, color2.blue, color2.alpha);
    gradient.add_color_stop_rgba(1.0, color1.red, color1.green, color1.blue, color1.alpha);
    cairo_context.set_source(&gradient)?;
    Ok(())
}

impl DefaultTheme
{
    pub fn new() -> Self
    {
        DefaultTheme {
            // Background colors.
            bg_color: Color::new_from_argb_u32(0xffcccccc),
            light_bg_color: Color::new_from_argb_u32(0xffffffff),
            dark_bg_color1: Color::new_from_argb_u32(0xff888888),
            dark_bg_color2: Color::new_from_argb_u32(0xffcccccc),
            selected_bg_color: Color::new_from_argb_u32(0xffbbbbee),
            title_bg_color1: Color::new_from_argb_u32(0xff2222ee),
            title_bg_color2: Color::new_from_argb_u32(0xff6666ee),
            title_bg_color1_for_unfocused_window: Color::new_from_argb_u32(0xff4444ee),
            title_bg_color2_for_unfocused_window: Color::new_from_argb_u32(0xff8888ee),
            // Hover color and active color.
            hover_color: Color::new_from_argb_u32(0x88e5e5e5),
            active_color: Color::new_from_argb_u32(0xcce5e5e5),
            // Border colors.
            border_color: Color::new_from_argb_u32(0xff222222),
            disabled_border_color: Color::new_from_argb_u32(0xff666666),
            focused_border_color: Color::new_from_argb_u32(0xff2222ee),
            current_border_color: Color::new_from_argb_u32(0xff22ee22),
            border_color_for_unfocused_window: Color::new_from_argb_u32(0xff444444),
            disabled_border_color_for_unfocused_window: Color::new_from_argb_u32(0xff888888),
            focused_border_color_for_unfocused_window: Color::new_from_argb_u32(0xff4444ee),
            current_border_color_for_unfocused_window: Color::new_from_argb_u32(0xff44ee44),
            // Foreground colors.
            fg_color: Color::new_from_argb_u32(0xff222222),
            disabled_fg_color: Color::new_from_argb_u32(0xff666666),
            white_fg_color: Color::new_from_argb_u32(0xffffffff),
            red_fg_color: Color::new_from_argb_u32(0xffee2222),
            green_fg_color: Color::new_from_argb_u32(0xff22ee22),
            blue_fg_color: Color::new_from_argb_u32(0xff2222ee),
            cyan_fg_color: Color::new_from_argb_u32(0xff22eeee),
            purple_fg_color: Color::new_from_argb_u32(0xffee22ee),
            yellow_fg_color: Color::new_from_argb_u32(0xffeeee22),
            title_fg_color: Color::new_from_argb_u32(0xffffffff),
            fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff444444),
            disabled_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff888888),
            white_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xffffffff),
            red_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xffee4444),
            green_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff44ee44),
            blue_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff4444ee),
            cyan_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff44eeee),
            purple_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xffee44ee),
            yellow_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xffeeee44),
        }
    }
    
    pub fn bg_color(&self) -> Color
    { self.bg_color }

    pub fn set_bg_color(&mut self, color: Color)
    { self.bg_color = color; }
    
    pub fn light_bg_color(&self) -> Color
    { self.light_bg_color }
    
    pub fn set_light_bg_color(&mut self, color: Color)
    { self.light_bg_color = color; }

    pub fn dark_bg_color1(&self) -> Color
    { self.dark_bg_color1 }

    pub fn set_dark_bg_color1(&mut self, color: Color)
    { self.dark_bg_color1 = color; }

    pub fn dark_bg_color2(&self) -> Color
    { self.dark_bg_color2 }

    pub fn set_dark_bg_color2(&mut self, color: Color)
    { self.dark_bg_color2 = color; }
    
    pub fn selected_bg_color(&self) -> Color
    { self.selected_bg_color }

    pub fn set_selected_bg_color(&mut self, color: Color)
    { self.selected_bg_color = color; }

    pub fn title_bg_color1(&self) -> Color
    { self.title_bg_color1 }

    pub fn set_title_bg_color1(&mut self, color: Color)
    { self.title_bg_color1 = color; }

    pub fn title_bg_color2(&self) -> Color
    { self.title_bg_color2 }

    pub fn set_title_bg_color2(&mut self, color: Color)
    { self.title_bg_color2 = color; }    
    
    pub fn title_bg_color1_for_unfocused_window(&self) -> Color
    { self.title_bg_color1_for_unfocused_window }

    pub fn set_title_bg_color1_for_unfocused_window(&mut self, color: Color)
    { self.title_bg_color2_for_unfocused_window = color; }

    pub fn title_bg_color2_for_unfocused_window(&self) -> Color
    { self.title_bg_color2_for_unfocused_window }

    pub fn set_title_bg_color2_for_unfocused_window(&mut self, color: Color)
    { self.title_bg_color2_for_unfocused_window = color; }
    
    pub fn hover_color(&self) -> Color
    { self.hover_color }

    pub fn set_hover_color(&mut self, color: Color)
    { self.hover_color = color; }
    
    pub fn active_color(&self) -> Color
    { self.active_color }

    pub fn set_active_color(&mut self, color: Color)
    { self.active_color = color; }

    pub fn border_color(&self) -> Color
    { self.border_color }

    pub fn set_border_color(&mut self, color: Color)
    { self.border_color = color; }
    
    pub fn disabled_border_color(&self) -> Color
    { self.disabled_border_color }

    pub fn set_disabled_border_color(&mut self, color: Color)
    { self.disabled_border_color = color; }

    pub fn focused_border_color(&self) -> Color
    { self.focused_border_color }

    pub fn set_focused_border_color(&mut self, color: Color)
    { self.focused_border_color = color; }
    
    pub fn current_border_color(&self) -> Color
    { self.current_border_color }

    pub fn set_current_border_color(&mut self, color: Color)
    { self.current_border_color = color; }

    pub fn border_color_for_unfocused_window(&self) -> Color
    { self.border_color_for_unfocused_window }

    pub fn set_border_color_for_unfocused_window(&mut self, color: Color)
    { self.border_color_for_unfocused_window = color; }
    
    pub fn disabled_border_color_for_unfocused_window(&self) -> Color
    { self.disabled_border_color_for_unfocused_window }
    
    pub fn set_disabled_border_color_for_unfocused_window(&mut self, color: Color)
    { self.disabled_border_color_for_unfocused_window = color; }
    
    pub fn focused_border_color_for_unfocused_window(&self) -> Color
    { self.focused_border_color_for_unfocused_window }
    
    pub fn set_focused_border_color_for_unfocused_window(&mut self, color: Color)
    { self.focused_border_color_for_unfocused_window = color; }

    pub fn current_border_color_for_unfocused_window(&self) -> Color
    { self.current_border_color_for_unfocused_window }
    
    pub fn set_current_border_color_for_unfocused_window(&mut self, color: Color)
    { self.current_border_color_for_unfocused_window = color; }    
    
    pub fn fg_color(&self) -> Color
    { self.fg_color }

    pub fn set_fg_color(&mut self, color: Color)
    { self.fg_color = color; }

    pub fn disabled_fg_color(&self) -> Color
    { self.disabled_fg_color }

    pub fn set_disabled_fg_color(&mut self, color: Color)
    { self.disabled_fg_color = color; }
    
    pub fn white_fg_color(&self) -> Color
    { self.white_fg_color }

    pub fn set_white_fg_color(&mut self, color: Color)
    { self.white_fg_color = color; }

    pub fn red_fg_color(&self) -> Color
    { self.red_fg_color }

    pub fn set_red_fg_color(&mut self, color: Color)
    { self.red_fg_color = color; }
    
    pub fn green_fg_color(&self) -> Color
    { self.green_fg_color }

    pub fn set_green_fg_color(&mut self, color: Color)
    { self.green_fg_color = color; }
    
    pub fn blue_fg_color(&self) -> Color
    { self.blue_fg_color }

    pub fn set_blue_fg_color(&mut self, color: Color)
    { self.blue_fg_color = color; }

    pub fn cyan_fg_color(&self) -> Color
    { self.cyan_fg_color }

    pub fn set_cyan_fg_color(&mut self, color: Color)
    { self.cyan_fg_color = color; }
    
    pub fn purple_fg_color(&self) -> Color
    { self.purple_fg_color }

    pub fn set_purple_fg_color(&mut self, color: Color)
    { self.purple_fg_color = color; }
    
    pub fn yellow_fg_color(&self) -> Color
    { self.yellow_fg_color }

    pub fn set_yellow_fg_color(&mut self, color: Color)
    { self.yellow_fg_color = color; }    
    
    pub fn title_fg_color(&self) -> Color
    { self.title_fg_color }
    
    pub fn set_title_fg_color(&mut self, color: Color)
    { self.title_fg_color = color; }
    
    pub fn fg_color_for_unfocused_window(&self) -> Color
    { self.fg_color_for_unfocused_window }
    
    pub fn set_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.fg_color_for_unfocused_window = color; }

    pub fn disabled_fg_color_for_unfocused_window(&self) -> Color
    { self.disabled_fg_color_for_unfocused_window }
    
    pub fn set_disabled_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.disabled_fg_color_for_unfocused_window = color; }
    
    pub fn white_fg_color_for_unfocused_window(&self) -> Color
    { self.white_fg_color_for_unfocused_window }
    
    pub fn set_white_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.white_fg_color_for_unfocused_window = color; }
    
    pub fn red_fg_color_for_unfocused_window(&self) -> Color
    { self.red_fg_color_for_unfocused_window }

    pub fn set_red_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.red_fg_color_for_unfocused_window = color; }
    
    pub fn green_fg_color_for_unfocused_window(&self) -> Color
    { self.green_fg_color_for_unfocused_window }
    
    pub fn set_green_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.green_fg_color_for_unfocused_window = color; }

    pub fn blue_fg_color_for_unfocused_window(&self) -> Color
    { self.blue_fg_color_for_unfocused_window }

    pub fn set_blue_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.blue_fg_color_for_unfocused_window = color; }

    pub fn cyan_fg_color_for_unfocused_window(&self) -> Color
    { self.cyan_fg_color_for_unfocused_window }

    pub fn set_cyan_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.cyan_fg_color_for_unfocused_window = color; }
    
    pub fn purple_fg_color_for_unfocused_window(&self) -> Color
    { self.purple_fg_color_for_unfocused_window }

    pub fn set_purple_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.purple_fg_color_for_unfocused_window = color; }
    
    pub fn yellow_fg_color_for_unfocused_window(&self) -> Color
    { self.yellow_fg_color_for_unfocused_window }

    pub fn set_yellow_fg_color_for_unfocused_window(&mut self, color: Color)
    { self.yellow_fg_color_for_unfocused_window = color; }
        
    fn set_bg_cairo_color(&self, cairo_context: &CairoContext)
    { set_cairo_color(cairo_context, self.bg_color); }

    fn set_light_bg_cairo_color(&self, cairo_context: &CairoContext, is_enabled: bool)
    {
        if is_enabled {
            set_cairo_color(cairo_context, self.light_bg_color);
        } else {
            set_cairo_color(cairo_context, self.bg_color);
        }
    }

    fn set_dark_bg_cairo_gradient(&self, cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient) -> Result<(), CairoError>
    { set_cairo_gradient(cairo_context, bounds, orient, self.dark_bg_color1, self.dark_bg_color2) }
    
    fn set_title_bg_cairo_gradient(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_gradient(cairo_context, bounds, Orient::Horizontal, self.title_bg_color1, self.title_bg_color2)            
        } else {
            set_cairo_gradient(cairo_context, bounds, Orient::Horizontal, self.title_bg_color1_for_unfocused_window, self.title_bg_color2_for_unfocused_window)
        }
    }
    
    fn set_state_cairo_color(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> bool
    {
        if is_focused_window && is_enabled {
            match state {
                WidgetState::None => false,
                WidgetState::Hover => {
                    set_cairo_color(cairo_context, self.hover_color);
                    true
                },
                WidgetState::Active => {
                    set_cairo_color(cairo_context, self.active_color);
                    true
                },
            }
        } else {
            false
        }
    }
    
    fn set_border_cairo_color(&self, cairo_context: &CairoContext, is_enabled: bool, is_focused: bool, is_focused_window: bool)
    {
        if is_focused_window {
            if is_enabled {
                if is_focused {
                    set_cairo_color(cairo_context, self.focused_border_color);
                } else {
                    set_cairo_color(cairo_context, self.border_color);
                }
            } else {
                set_cairo_color(cairo_context, self.disabled_border_color);
            }
        } else {
            if is_enabled {
                if is_focused {
                    set_cairo_color(cairo_context, self.focused_border_color_for_unfocused_window);
                } else {
                    set_cairo_color(cairo_context, self.border_color_for_unfocused_window);
                }
            } else {
                set_cairo_color(cairo_context, self.disabled_border_color_for_unfocused_window);
            }
        }
    }
    
    fn set_focused_border_cairo_color(&self, cairo_context: &CairoContext, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> bool
    {
        if is_focused_window {
            if is_enabled && is_focused {
                set_cairo_color(cairo_context, self.focused_border_color);
                true
            } else {
                false
            }
        } else {
            if is_enabled && is_focused {
                set_cairo_color(cairo_context, self.focused_border_color_for_unfocused_window);
                true
            } else {
                false
            }
        }
    }

    fn set_fg_cairo_color(&self, cairo_context: &CairoContext, is_enabled: bool, is_focused_window: bool)
    {
        if is_focused_window {
            if is_enabled {
                set_cairo_color(cairo_context, self.fg_color);
            } else {
                set_cairo_color(cairo_context, self.disabled_fg_color);
            }
        } else {
            if is_enabled {
                set_cairo_color(cairo_context, self.fg_color_for_unfocused_window);
            } else {
                set_cairo_color(cairo_context, self.disabled_fg_color_for_unfocused_window);
            }
        }
    }    
    
    fn set_title_fg_cairo_color(&self, cairo_context: &CairoContext)
    { set_cairo_color(cairo_context, self.title_fg_color); }

    fn draw_check(&self, cairo_context: &CairoContext, pos: Pos<i32>, is_checked: bool, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError> 
    {
        let x = pos.x as f64;
        let y = pos.y as f64;
        self.set_light_bg_cairo_color(cairo_context, is_enabled);
        cairo_context.rectangle(x + 1.0, y + 1.0, 10.0, 10.0);
        cairo_context.fill()?;
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        cairo_context.rectangle(x + 1.0, y + 1.0, 10.0, 10.0);
        cairo_context.stroke()?;
        if is_checked {
            cairo_context.move_to(x + 2.0, y + 6.0);
            cairo_context.line_to(x + 6.0, y + 10.0);
            cairo_context.line_to(x + 10.0, y + 2.0);
            cairo_context.stroke()?;
        }
        Ok(())
    }

    fn draw_radio(&self, cairo_context: &CairoContext, pos: Pos<i32>, is_selected: bool, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError> 
    {
        let x = pos.x as f64;
        let y = pos.y as f64;
        self.set_light_bg_cairo_color(cairo_context, is_enabled);
        cairo_context.arc(x + 6.0, y + 6.0, 5.0, 0.0, PI * 2.0);
        cairo_context.fill()?;
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        cairo_context.arc(x + 6.0, y + 6.0, 5.0, 0.0, PI * 2.0);
        cairo_context.stroke()?;
        if is_selected {
            cairo_context.arc(x + 6.0, y + 6.0, 2.0, 0.0, PI * 2.0);
            cairo_context.fill()?;
        }
        Ok(())
    }
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

    fn draw_toplevel_window_title_bar_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool) -> Result<(), CairoError>
    {
        self.set_title_bg_cairo_gradient(cairo_context, bounds, is_focused_window)?;
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64);
        cairo_context.fill()?;
        self.set_border_cairo_color(cairo_context, true, false, is_focused_window);
        cairo_context.move_to((bounds.x as f64) + 1.0, (bounds.y as f64) + (bounds.height as f64));
        cairo_context.line_to((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0);
        cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, (bounds.y as f64) + 1.0);
        cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, (bounds.y as f64) + (bounds.height as f64));
        cairo_context.stroke()?;
        Ok(())
    }
    
    fn draw_toplevel_window_content_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool, is_tool_bar: bool) -> Result<(), CairoError>
    {
        self.set_bg_cairo_color(cairo_context);
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64);
        cairo_context.fill()?;
        self.set_border_cairo_color(cairo_context, true, false, is_focused_window);
        if !is_tool_bar {
            cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0);
            cairo_context.stroke()?;
        } else {
            cairo_context.move_to((bounds.x as f64) + 1.0, bounds.y as f64);
            cairo_context.line_to((bounds.x as f64) + 1.0, (bounds.y as f64) + (bounds.height as f64) - 1.0);
            cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, (bounds.y as f64) + (bounds.height as f64) - 1.0);
            cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, bounds.y as f64);
            cairo_context.stroke()?;
        }
        Ok(())
    }

    fn draw_title_bar_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn title_margin_edges(&self) -> Edges<i32>
    { Edges::new(0, 0, 0, 0) }

    fn title_padding_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 2, 2) }

    fn draw_title_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_title_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>
    { 
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Bold);
        cairo_context.set_font_size(16.0);
        Ok(())
    }
    
    fn draw_title_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
        self.set_title_fg_cairo_color(cairo_context);
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }

    fn title_button_margin_edges(&self) -> Edges<i32>
    { Edges::new(2, 2, 2, 2) }

    fn title_button_padding_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 4, 4) }

    fn draw_title_button_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        self.set_dark_bg_cairo_gradient(cairo_context, bounds, Orient::Horizontal)?;
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
        cairo_context.fill()?;
        if self.set_state_cairo_color(cairo_context, state, is_enabled, is_focused_window) {
            cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
            cairo_context.fill()?;
        }
        self.set_border_cairo_color(cairo_context, is_enabled, false, is_focused_window);
        cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0); 
        cairo_context.stroke()?;
        Ok(())
    }

    fn title_button_icon_size(&self) -> Size<i32>
    { Size::new(DEFAULT_TITLE_BUTTON_ICON_SIZE, DEFAULT_TITLE_BUTTON_ICON_SIZE) }

    fn draw_title_button_icon(&self, cairo_context: &CairoContext, pos: Pos<i32>, icon: TitleButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    { draw_default_title_button_icon(cairo_context, self, pos, icon, state, is_enabled, is_focused, is_focused_window) }

    fn empty_margin_edges(&self) -> Edges<i32>
    { Edges::new(2, 2, 2, 2) }
    
    fn label_margin_edges(&self) -> Edges<i32>
    { Edges::new(2, 2, 2, 2) }

    fn label_padding_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 4, 4) }
    
    fn draw_label_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if self.set_state_cairo_color(cairo_context, state, is_enabled, is_focused_window) {
            cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
            cairo_context.fill()?;
        }
        Ok(())
    }

    fn set_label_font(&self, _cairo_context: &CairoContext) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_label_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }
    
    fn button_margin_edges(&self) -> Edges<i32>
    { Edges::new(2, 2, 2, 2) }

    fn button_padding_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 4, 4) }

    fn button_sep_width(&self) -> i32
    { 4 }
    
    fn draw_button_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        self.set_dark_bg_cairo_gradient(cairo_context, bounds, Orient::Horizontal)?;
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
        cairo_context.fill()?;
        if self.set_state_cairo_color(cairo_context, state, is_enabled, is_focused_window) {
            cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
            cairo_context.fill()?;
        }
        self.set_border_cairo_color(cairo_context, is_enabled, is_focused, is_focused_window);
        cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0); 
        cairo_context.stroke()?;
        Ok(())
    }

    fn set_button_font(&self, _cairo_context: &CairoContext) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_button_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }

    fn button_icon_size(&self) -> Size<i32>
    { Size::new(DEFAULT_BUTTON_ICON_SIZE, DEFAULT_BUTTON_ICON_SIZE) } 
    
    fn draw_button_icon(&self, cairo_context: &CairoContext, pos: Pos<i32>, icon: ButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    { draw_default_button_icon(cairo_context, self, pos, icon, state, is_enabled, is_focused, is_focused_window) }

    fn check_margin_edges(&self) -> Edges<i32>
    { Edges::new(2, 2, 2, 2) }

    fn check_padding_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 4 + CHECK_SIZE + 4, 4) }
    
    fn draw_check_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_checked: bool, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if self.set_state_cairo_color(cairo_context, state, is_enabled, is_focused_window) {
            cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
            cairo_context.fill()?;
        }
        let pos = Pos::new(bounds.x + 4, bounds.y + (bounds.height - CHECK_SIZE) / 2);
        self.draw_check(cairo_context, pos, is_checked, is_enabled, is_focused_window)?;
        if self.set_focused_border_cairo_color(cairo_context, is_enabled, is_focused, is_focused_window) {
            cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0); 
            cairo_context.stroke()?;
        }
        Ok(())
    }

    fn set_check_font(&self, _cairo_context: &CairoContext) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_check_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _is_checked: bool, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }

    fn radio_margin_edges(&self) -> Edges<i32>
    { Edges::new(2, 2, 2, 2) }

    fn radio_padding_edges(&self) -> Edges<i32>
    { Edges::new(4, 4, 4 + RADIO_SIZE + 4, 4) } 
    
    fn draw_radio_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_selected: bool, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if self.set_state_cairo_color(cairo_context, state, is_enabled, is_focused_window) {
            cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
            cairo_context.fill()?;
        }
        let pos = Pos::new(bounds.x + 4, bounds.y + (bounds.height - RADIO_SIZE) / 2);
        self.draw_radio(cairo_context, pos, is_selected, is_enabled, is_focused_window)?;
        if self.set_focused_border_cairo_color(cairo_context, is_enabled, is_focused, is_focused_window) {
            cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0); 
            cairo_context.stroke()?;
        }
        Ok(())
    }

    fn set_radio_font(&self, _cairo_context: &CairoContext) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_radio_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _is_selected: bool, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }
    
    fn draw_linear_layout_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_grid_layout_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn set_fg(&self, cairo_context: &CairoContext, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        self.set_fg_cairo_color(cairo_context, is_enabled, is_focused_window);
        Ok(())
    }

    fn set_white_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.white_fg_color);
        } else {
            set_cairo_color(cairo_context, self.white_fg_color_for_unfocused_window);
        }
        Ok(())
    }
    
    fn set_red_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.red_fg_color);
        } else {
            set_cairo_color(cairo_context, self.red_fg_color_for_unfocused_window);
        }
        Ok(())
    }
    
    fn set_green_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.green_fg_color);
        } else {
            set_cairo_color(cairo_context, self.green_fg_color_for_unfocused_window);
        }
        Ok(())
    }

    fn set_blue_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.blue_fg_color);
        } else {
            set_cairo_color(cairo_context, self.blue_fg_color_for_unfocused_window);
        }
        Ok(())
    }
    
    fn set_cyan_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.cyan_fg_color);
        } else {
            set_cairo_color(cairo_context, self.cyan_fg_color_for_unfocused_window);
        }
        Ok(())
    }
    
    fn set_purple_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.purple_fg_color);
        } else {
            set_cairo_color(cairo_context, self.purple_fg_color_for_unfocused_window);
        }
        Ok(())
    }

    fn set_yellow_fg(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.yellow_fg_color);
        } else {
            set_cairo_color(cairo_context, self.yellow_fg_color_for_unfocused_window);
        }
        Ok(())
    }
}
