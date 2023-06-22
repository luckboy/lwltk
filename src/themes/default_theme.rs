//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use cairo::FontSlant;
use cairo::FontWeight;
use crate::image::*;
use crate::theme::*;
use crate::themes::default_button_icons::*;
use crate::types::*;
use crate::utils::*;

pub struct DefaultTheme
{
    // Background colors.
    bg_color: Color,
    light_bg_color: Color,
    dark_bg_color: Color,
    selected_bg_color: Color,
    title_bg_color: Color,
    title_bg_color_for_unfocused_window: Color,
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
    fg2_color: Color,
    fg3_color: Color,
    fg4_color: Color,
    fg5_color: Color,
    title_fg_color: Color,
    fg_color_for_unfocused_window: Color,
    disabled_fg_color_for_unfocused_window: Color,
    fg2_color_for_unfocused_window: Color,
    fg3_color_for_unfocused_window: Color,
    fg4_color_for_unfocused_window: Color,
    fg5_color_for_unfocused_window: Color,
}

impl DefaultTheme
{
    pub fn new() -> Self
    {
        DefaultTheme {
            // Background colors.
            bg_color: Color::new_from_argb_u32(0xffcccccc),
            light_bg_color: Color::new_from_argb_u32(0xffffffff),
            dark_bg_color: Color::new_from_argb_u32(0xffbbbbbb),
            selected_bg_color: Color::new_from_argb_u32(0xffbbbbee),
            title_bg_color: Color::new_from_argb_u32(0xff2222ee),
            title_bg_color_for_unfocused_window: Color::new_from_argb_u32(0xff4444ee),
            // Hover color and active color.
            hover_color: Color::new_from_argb_u32(0x88dddddd),
            active_color: Color::new_from_argb_u32(0x88eeeeee),
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
            fg2_color: Color::new_from_argb_u32(0xffffffff),
            fg3_color: Color::new_from_argb_u32(0xffee2222),
            fg4_color: Color::new_from_argb_u32(0xff22ee22),
            fg5_color: Color::new_from_argb_u32(0xff2222ee),
            title_fg_color: Color::new_from_argb_u32(0xffffffff),
            fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff444444),
            disabled_fg_color_for_unfocused_window: Color::new_from_argb_u32(0xff888888),
            fg2_color_for_unfocused_window: Color::new_from_argb_u32(0xffffffff),
            fg3_color_for_unfocused_window: Color::new_from_argb_u32(0xffee4444),
            fg4_color_for_unfocused_window: Color::new_from_argb_u32(0xff44ee44),
            fg5_color_for_unfocused_window: Color::new_from_argb_u32(0xff4444ee),
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

    pub fn dark_bg_color(&self) -> Color
    { self.dark_bg_color }

    pub fn set_dark_bg_color(&mut self, color: Color)
    { self.dark_bg_color = color; }
    
    pub fn selected_bg_color(&self) -> Color
    { self.selected_bg_color }

    pub fn set_selected_bg_color(&mut self, color: Color)
    { self.selected_bg_color = color; }

    pub fn title_bg_color(&self) -> Color
    { self.title_bg_color }

    pub fn set_title_bg_color(&mut self, color: Color)
    { self.title_bg_color = color; }

    pub fn title_bg_color_for_unfocused_window(&self) -> Color
    { self.title_bg_color_for_unfocused_window }

    pub fn set_title_bg_color_for_unfocused_window(&mut self, color: Color)
    { self.title_bg_color_for_unfocused_window = color; }
    
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
    
    pub fn fg2_color(&self) -> Color
    { self.fg2_color }

    pub fn set_fg2_color(&mut self, color: Color)
    { self.fg2_color = color; }

    pub fn fg3_color(&self) -> Color
    { self.fg3_color }

    pub fn set_fg3_color(&mut self, color: Color)
    { self.fg3_color = color; }
    
    pub fn fg4_color(&self) -> Color
    { self.fg4_color }

    pub fn set_fg4_color(&mut self, color: Color)
    { self.fg4_color = color; }
    
    pub fn fg5_color(&self) -> Color
    { self.fg5_color }

    pub fn set_fg5_color(&mut self, color: Color)
    { self.fg5_color = color; }

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
    
    pub fn fg2_color_for_unfocused_window(&self) -> Color
    { self.fg2_color_for_unfocused_window }
    
    pub fn set_fg2_color_for_unfocused_window(&mut self, color: Color)
    { self.fg2_color_for_unfocused_window = color; }
    
    pub fn fg3_color_for_unfocused_window(&self) -> Color
    { self.fg3_color_for_unfocused_window }

    pub fn set_fg3_color_for_unfocused_window(&mut self, color: Color)
    { self.fg3_color_for_unfocused_window = color; }
    
    pub fn fg4_color_for_unfocused_window(&self) -> Color
    { self.fg4_color_for_unfocused_window }
    
    pub fn set_fg4_color_for_unfocused_window(&mut self, color: Color)
    { self.fg4_color_for_unfocused_window = color; }

    pub fn fg5_color_for_unfocused_window(&self) -> Color
    { self.fg5_color_for_unfocused_window }

    pub fn set_fg5_color_for_unfocused_window(&mut self, color: Color)
    { self.fg5_color_for_unfocused_window = color; }
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
        set_cairo_color(cairo_context, self.bg_color);
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64);
        cairo_context.fill()?;
        if is_focused_window {
            set_cairo_color(cairo_context, self.border_color);
        } else {
            set_cairo_color(cairo_context, self.border_color_for_unfocused_window);
        }
        cairo_context.move_to((bounds.x as f64) + 1.0, bounds.y as f64);
        cairo_context.line_to((bounds.x as f64) + 1.0, (bounds.y as f64) + (bounds.height as f64) - 1.0);
        cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, (bounds.y as f64) + (bounds.height as f64) - 1.0);
        cairo_context.line_to((bounds.x as f64) + (bounds.width as f64) - 1.0, bounds.y as f64);
        cairo_context.stroke()?;
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
        set_cairo_color(cairo_context, self.dark_bg_color);
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
        cairo_context.fill()?;
        if is_focused_window && is_enabled {
            match state {
                WidgetState::None => (),
                WidgetState::Hover => {
                    set_cairo_color(cairo_context, self.hover_color);
                    cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
                    cairo_context.fill()?;
                },
                WidgetState::Active => {
                    set_cairo_color(cairo_context, self.active_color);
                    cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64); 
                    cairo_context.fill()?;
                },
            }
        }
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
        cairo_context.rectangle((bounds.x as f64) + 1.0, (bounds.y as f64) + 1.0, (bounds.width as f64) - 2.0, (bounds.height as f64) - 2.0); 
        cairo_context.stroke()?;
        Ok(())
    }

    fn set_button_font(&self, _cairo_context: &CairoContext) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_button_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        let font_extents = cairo_context.font_extents()?;
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
        cairo_context.move_to(pos.x as f64, (pos.y as f64) + font_extents.ascent);
        cairo_context.show_text(s)?;
        Ok(())
    }

    fn button_icon_size(&self) -> Size<i32>
    { Size::new(DEFAULT_BUTTON_ICON_WIDTH, DEFAULT_BUTTON_ICON_HEIGHT) } 
    
    fn draw_button_icon(&self, cairo_context: &CairoContext, pos: Pos<i32>, icon: ButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    { draw_default_button_icon(cairo_context, self, pos, icon, state, is_enabled, is_focused, is_focused_window) }

    fn draw_linear_layout_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_grid_layout_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn set_fg(&self, cairo_context: &CairoContext, _state: WidgetState, is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
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
        Ok(())
    }

    fn set_fg2(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.fg2_color);
        } else {
            set_cairo_color(cairo_context, self.fg2_color_for_unfocused_window);
        }
        Ok(())
    }
    
    fn set_fg3(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.fg3_color);
        } else {
            set_cairo_color(cairo_context, self.fg3_color_for_unfocused_window);
        }
        Ok(())
    }
    
    fn set_fg4(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.fg4_color);
        } else {
            set_cairo_color(cairo_context, self.fg4_color_for_unfocused_window);
        }
        Ok(())
    }

    fn set_fg5(&self, cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
    {
        if is_focused_window {
            set_cairo_color(cairo_context, self.fg5_color);
        } else {
            set_cairo_color(cairo_context, self.fg5_color_for_unfocused_window);
        }
        Ok(())
    }
}
