//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_error::*;
use crate::image::*;
use crate::themes::*;
use crate::types::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ScrollBarElems
{
    Button1Button2Slider,
    Button2Button1Slider,
    Button2SliderButton1,
    Button1SliderButton2,
    SliderButton1Button2,
    SliderButton2Button1,
}

impl ScrollBarElems
{
    pub fn to_array(&self) -> [ScrollBarElem; 3]
    {
        match self {
            ScrollBarElems::Button1Button2Slider => [ScrollBarElem::FirstButton, ScrollBarElem::SecondButton, ScrollBarElem::Slider],
            ScrollBarElems::Button2Button1Slider => [ScrollBarElem::SecondButton, ScrollBarElem::FirstButton, ScrollBarElem::Slider],
            ScrollBarElems::Button2SliderButton1 => [ScrollBarElem::SecondButton, ScrollBarElem::Slider, ScrollBarElem::FirstButton],
            ScrollBarElems::Button1SliderButton2 => [ScrollBarElem::FirstButton, ScrollBarElem::Slider, ScrollBarElem::SecondButton],
            ScrollBarElems::SliderButton1Button2 => [ScrollBarElem::Slider, ScrollBarElem::FirstButton, ScrollBarElem::SecondButton],
            ScrollBarElems::SliderButton2Button1 => [ScrollBarElem::Slider, ScrollBarElem::SecondButton, ScrollBarElem::FirstButton],
        }
    }
}

pub trait Theme: Send + Sync
{
    fn set_cairo_context(&self, cairo_context: &CairoContext, scale: i32) -> Result<(), CairoError>;

    fn toplevel_window_edges(&self) -> Edges<i32>;

    fn toplevel_window_corners(&self) -> Corners<i32>;
    
    fn draw_toplevel_window_title_bar_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool) -> Result<(), CairoError>;

    fn draw_toplevel_window_content_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_focused_window: bool, is_tool_bar: bool) -> Result<(), CairoError>;

    fn draw_title_bar_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn title_margin_edges(&self) -> Edges<i32>;

    fn title_padding_edges(&self) -> Edges<i32>;

    fn draw_title_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn set_title_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>;
    
    fn draw_title_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn title_button_margin_edges(&self) -> Edges<i32>;

    fn title_button_padding_edges(&self) -> Edges<i32>;

    fn draw_title_button_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn title_button_icon_size(&self) -> Size<i32>;
    
    fn draw_title_button_icon(&self, cairo_context: &CairoContext, pos: Pos<i32>, icon: TitleButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;    

    fn empty_margin_edges(&self) -> Edges<i32>;
    
    fn label_margin_edges(&self) -> Edges<i32>;

    fn label_padding_edges(&self) -> Edges<i32>;
    
    fn draw_label_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>; 

    fn set_label_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>;
    
    fn draw_label_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;    
    
    fn button_margin_edges(&self) -> Edges<i32>;

    fn button_padding_edges(&self) -> Edges<i32>;
    
    fn button_sep_width(&self) -> i32;
    
    fn draw_button_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>; 

    fn set_button_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>;
    
    fn draw_button_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn button_icon_size(&self) -> Size<i32>;
    
    fn draw_button_icon(&self, cairo_context: &CairoContext, pos: Pos<i32>, icon: ButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn check_margin_edges(&self) -> Edges<i32>;

    fn check_padding_edges(&self) -> Edges<i32>;
    
    fn draw_check_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_checked: bool, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>; 

    fn set_check_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>;
    
    fn draw_check_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, is_checked: bool, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn radio_margin_edges(&self) -> Edges<i32>;

    fn radio_padding_edges(&self) -> Edges<i32>;
    
    fn draw_radio_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, is_selected: bool, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>; 

    fn set_radio_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>;
    
    fn draw_radio_text(&self, cairo_context: &CairoContext, pos: Pos<i32>, s: &str, is_selected: bool, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;    
    
    fn draw_linear_layout_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn draw_grid_layout_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;    
    
    fn scroll_bar_margin_edges(&self) -> Edges<i32>;
    
    fn scroll_bar_elems(&self) -> ScrollBarElems;
    
    fn h_scroll_bar_height(&self) -> i32;

    fn h_scroll_bar_button_width(&self) -> i32;

    fn v_scroll_bar_width(&self) -> i32;

    fn v_scroll_bar_button_height(&self) -> i32;
    
    fn draw_sroll_bar_first_button(&self, cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn draw_sroll_bar_second_button(&self, cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn draw_sroll_bar_trough(&self, cairo_context: &CairoContext, bounds: Rect<i32>, orient: Orient, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn draw_sroll_bar_slider(&self, cairo_context: &CairoContext, bounds: Rect<f64>, orient: Orient, state: WidgetState, is_enabled: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn set_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn set_white_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;
    
    fn set_red_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;
    
    fn set_green_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn set_blue_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn set_cyan_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;
    
    fn set_purple_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;

    fn set_yellow_fg(&self, cairo_context: &CairoContext, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>;
}

pub fn theme_from_env() -> Result<Box<dyn Theme>, ClientError>
{ Ok(Box::new(DefaultTheme::new())) }
