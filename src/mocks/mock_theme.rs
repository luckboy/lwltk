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
use crate::types::*;

pub(crate) struct MockTheme
{
    font_size: f64,
    toplevel_window_edges: Edges<i32>,
    toplevel_window_corners: Corners<i32>,
    title_margin_edges: Edges<i32>,
    title_padding_edges: Edges<i32>,
    title_font_size: f64,
    title_button_margin_edges: Edges<i32>,
    title_button_padding_edges: Edges<i32>,
    title_button_icon_size: Size<i32>,
    empty_margin_edges: Edges<i32>,
    label_margin_edges: Edges<i32>,
    label_padding_edges: Edges<i32>,
    label_font_size: f64,
    button_margin_edges: Edges<i32>,
    button_padding_edges: Edges<i32>,
    button_sep_width: i32,
    button_font_size: f64,
    button_icon_size: Size<i32>,
    check_margin_edges: Edges<i32>,
    check_padding_edges: Edges<i32>,
    check_font_size: f64,
    radio_margin_edges: Edges<i32>,
    radio_padding_edges: Edges<i32>,
    radio_font_size: f64,
    scroll_bar_margin_edges: Edges<i32>,
    scroll_bar_elems: ScrollBarElems,
    scroll_bar_button_size: Size<i32>,
}

impl MockTheme
{
    pub(crate) fn new() -> Self
    {
        MockTheme {
            font_size: 0.0,
            toplevel_window_edges: Edges::new(0, 0, 0, 0),
            toplevel_window_corners: Corners::new(0, 0, 0, 0, 0, 0, 0, 0),
            title_margin_edges: Edges::new(0, 0, 0, 0),
            title_padding_edges: Edges::new(0, 0, 0, 0),
            title_font_size: 0.0,
            title_button_margin_edges: Edges::new(0, 0, 0, 0),
            title_button_padding_edges: Edges::new(0, 0, 0, 0),
            title_button_icon_size: Size::new(0, 0),
            empty_margin_edges: Edges::new(0, 0, 0, 0),
            label_margin_edges: Edges::new(0, 0, 0, 0),
            label_padding_edges: Edges::new(0, 0, 0, 0),
            label_font_size: 0.0,
            button_margin_edges: Edges::new(0, 0, 0, 0),
            button_padding_edges: Edges::new(0, 0, 0, 0),
            button_sep_width: 0,
            button_font_size: 0.0,
            button_icon_size: Size::new(0, 0),
            check_margin_edges: Edges::new(0, 0, 0, 0),
            check_padding_edges: Edges::new(0, 0, 0, 0),
            check_font_size: 0.0,
            radio_margin_edges: Edges::new(0, 0, 0, 0),
            radio_padding_edges: Edges::new(0, 0, 0, 0),
            radio_font_size: 0.0,
            scroll_bar_margin_edges: Edges::new(0, 0, 0, 0),
            scroll_bar_elems: ScrollBarElems::Button1Button2Slider,
            scroll_bar_button_size: Size::new(0, 0),
        }
    }

    pub(crate) fn set_font_size(&mut self, font_size: f64)
    { self.font_size = font_size; }
    
    pub(crate) fn set_toplevel_window_edges(&mut self, edges: Edges<i32>)
    { self.toplevel_window_edges = edges; }
    
    pub(crate) fn set_toplevel_window_corners(&mut self, corners: Corners<i32>)
    { self.toplevel_window_corners = corners; }

    pub(crate) fn set_title_margin_edges(&mut self, edges: Edges<i32>)
    { self.title_margin_edges = edges; }
    
    pub(crate) fn set_title_padding_edges(&mut self, edges: Edges<i32>)
    { self.title_padding_edges = edges; }

    pub(crate) fn set_title_font_size(&mut self, font_size: f64)
    { self.title_font_size = font_size; }
    
    pub(crate) fn set_title_button_margin_edges(&mut self, edges: Edges<i32>)
    { self.title_button_margin_edges = edges; }
    
    pub(crate) fn set_title_button_padding_edges(&mut self, edges: Edges<i32>)
    { self.title_button_padding_edges = edges; }

    pub(crate) fn set_title_button_icon_size(&mut self, size: Size<i32>)
    { self.title_button_icon_size = size; }

    pub(crate) fn set_empty_margin_edges(&mut self, edges: Edges<i32>)
    { self.empty_margin_edges = edges; }

    pub(crate) fn set_label_margin_edges(&mut self, edges: Edges<i32>)
    { self.label_margin_edges = edges; }

    pub(crate) fn set_label_padding_edges(&mut self, edges: Edges<i32>)
    { self.label_padding_edges = edges; }

    pub(crate) fn set_label_font_size(&mut self, font_size: f64)
    { self.label_font_size = font_size; }

    pub(crate) fn set_button_margin_edges(&mut self, edges: Edges<i32>)
    { self.button_margin_edges = edges; }

    pub(crate) fn set_button_padding_edges(&mut self, edges: Edges<i32>)
    { self.button_padding_edges = edges; }

    pub(crate) fn set_button_sep_width(&mut self, width: i32)
    { self.button_sep_width = width; }

    pub(crate) fn set_button_font_size(&mut self, font_size: f64)
    { self.button_font_size = font_size; }

    pub(crate) fn set_button_icon_size(&mut self, size: Size<i32>)
    { self.button_icon_size = size; }

    pub(crate) fn set_check_margin_edges(&mut self, edges: Edges<i32>)
    { self.check_margin_edges = edges; }

    pub(crate) fn set_check_padding_edges(&mut self, edges: Edges<i32>)
    { self.check_padding_edges = edges; }
    
    pub(crate) fn set_check_font_size(&mut self, font_size: f64)
    { self.check_font_size = font_size; }
    
    pub(crate) fn set_radio_margin_edges(&mut self, edges: Edges<i32>)
    { self.radio_margin_edges = edges; }

    pub(crate) fn set_radio_padding_edges(&mut self, edges: Edges<i32>)
    { self.radio_padding_edges = edges; }

    pub(crate) fn set_radio_font_size(&mut self, font_size: f64)
    { self.radio_font_size = font_size; }

    pub(crate) fn set_scroll_bar_margin_edges(&mut self, edges: Edges<i32>)
    { self.scroll_bar_margin_edges = edges; }
    
    pub(crate) fn set_scroll_bar_elems(&mut self, elems: ScrollBarElems)
    { self.scroll_bar_elems = elems; }
    
    pub(crate) fn set_scroll_bar_button_size(&mut self, size: Size<i32>)
    { self.scroll_bar_button_size = size; }
}

impl Theme for MockTheme
{
    fn set_cairo_context(&self, cairo_context: &CairoContext, _scale: i32) -> Result<(), CairoError>
    { 
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(self.font_size);
        Ok(())
    }

    fn toplevel_window_edges(&self) -> Edges<i32>
    { self.toplevel_window_edges }

    fn toplevel_window_corners(&self) -> Corners<i32>
    { self.toplevel_window_corners }
    
    fn draw_toplevel_window_title_bar_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_toplevel_window_content_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _is_focused_window: bool, _is_tool_bar: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_title_bar_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn title_margin_edges(&self) -> Edges<i32>
    { self.title_margin_edges }

    fn title_padding_edges(&self) -> Edges<i32>
    { self.title_padding_edges }

    fn draw_title_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_title_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>
    { 
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(self.title_font_size);
        Ok(())
    }
    
    fn draw_title_text(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _s: &str, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn title_button_margin_edges(&self) -> Edges<i32>
    { self.title_button_margin_edges }

    fn title_button_padding_edges(&self) -> Edges<i32>
    { self.title_button_padding_edges }

    fn draw_title_button_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn title_button_icon_size(&self) -> Size<i32>
    { self.title_button_icon_size }
    
    fn draw_title_button_icon(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _icon: TitleButtonIcon, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>   
    { Ok(()) }

    fn empty_margin_edges(&self) -> Edges<i32>
    { self.empty_margin_edges }
    
    fn label_margin_edges(&self) -> Edges<i32>
    { self.label_margin_edges }

    fn label_padding_edges(&self) -> Edges<i32>
    { self.label_padding_edges }
    
    fn draw_label_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_label_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>
    { 
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(self.label_font_size);
        Ok(())
    }
    
    fn draw_label_text(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _s: &str, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>    
    { Ok(()) }
    
    fn button_margin_edges(&self) -> Edges<i32>
    { self.button_margin_edges }

    fn button_padding_edges(&self) -> Edges<i32>
    { self.button_padding_edges }
    
    fn button_sep_width(&self) -> i32
    { self.button_sep_width }
    
    fn draw_button_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_button_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>
    {
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(self.button_font_size);
        Ok(())
    }
    
    fn draw_button_text(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _s: &str, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn button_icon_size(&self) -> Size<i32>
    { self.button_icon_size }
    
    fn draw_button_icon(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _icon: ButtonIcon, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn check_margin_edges(&self) -> Edges<i32>
    { self.check_margin_edges }

    fn check_padding_edges(&self) -> Edges<i32>
    { self.check_padding_edges }
    
    fn draw_check_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _is_checked: bool, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_check_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>
    {
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(self.check_font_size);
        Ok(())
    }
    
    fn draw_check_text(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _s: &str, _is_checked: bool, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn radio_margin_edges(&self) -> Edges<i32>
    { self.radio_margin_edges }

    fn radio_padding_edges(&self) -> Edges<i32>
    { self.radio_padding_edges }
    
    fn draw_radio_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _is_selected: bool, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_radio_font(&self, cairo_context: &CairoContext) -> Result<(), CairoError>
    { 
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(self.radio_font_size);
        Ok(())
    }
    
    fn draw_radio_text(&self, _cairo_context: &CairoContext, _pos: Pos<i32>, _s: &str, _is_selected: bool, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn draw_linear_layout_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_grid_layout_bg(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn scroll_bar_margin_edges(&self) -> Edges<i32>
    { self.scroll_bar_margin_edges }
    
    fn scroll_bar_elems(&self) -> ScrollBarElems
    { self.scroll_bar_elems }
    
    fn scroll_bar_button_size(&self) -> Size<i32>
    { self.scroll_bar_button_size }
    
    fn draw_sroll_bar_first_button(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_sroll_bar_second_button(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_sroll_bar_trough(&self, _cairo_context: &CairoContext, _bounds: Rect<i32>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn draw_sroll_bar_slider(&self, _cairo_context: &CairoContext, _bounds: Rect<f64>, _orient: Orient, _state: WidgetState, _is_enabled: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn set_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_white_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn set_red_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn set_green_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_blue_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_cyan_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
    
    fn set_purple_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }

    fn set_yellow_fg(&self, _cairo_context: &CairoContext, _state: WidgetState, _is_enabled: bool, _is_focused: bool, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
}
