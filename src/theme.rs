//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_error::*;
use crate::themes::*;
use crate::types::*;

pub trait Theme: Send + Sync
{
    fn set_cairo_context(&self, cairo_context: &CairoContext, scale: i32) -> Result<(), CairoError>;

    fn toplevel_window_edges(&self) -> Edges<i32>;

    fn toplevel_window_corners(&self) -> Corners<i32>;
    
    fn draw_toplevel_window_content_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool) -> Result<(), CairoError>;
    
    fn button_margin_egdes(&self) -> Edges<i32>;

    fn button_padding_egdes(&self) -> Edges<i32>;
    
    fn draw_button_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>; 

    fn set_button_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>;
    
    fn draw_button_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;
}

pub fn theme_from_env() -> Result<Box<dyn Theme>, ClientError>
{ Ok(Box::new(DefaultTheme::new())) }
