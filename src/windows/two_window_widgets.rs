//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::max;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;

pub struct TwoWindowWidgets
{
    pub has_trimmed_width: bool,
    pub has_trimmed_height: bool,
    pub title_bar: Option<Box<dyn Widget>>,
    pub content: Option<Box<dyn Widget>>,
}

impl TwoWindowWidgets
{
    pub fn new() -> Self
    {
        TwoWindowWidgets {
            has_trimmed_width: false,
            has_trimmed_height: false,
            title_bar: None,
            content: None,
        }
    }

    pub fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if self.content.is_some() {
                    Some(WidgetIndexPair(1, 0))
                } else if self.title_bar.is_some() {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(1, 0)) => {
                if self.content.is_some() && self.title_bar.is_some() {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            _ => None, 
        }
    }
    
    pub fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if self.title_bar.is_some() {
                    Some(WidgetIndexPair(0, 0))
                } else if self.content.is_some() {
                    Some(WidgetIndexPair(1, 0))
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(0, 0)) => {
                if self.title_bar.is_some() && self.content.is_some() {
                    Some(WidgetIndexPair(1, 0))
                } else {
                    None
                }
            },
            _ => None, 
        }
    }
    
    pub fn dyn_widget(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(0, 0) => {
                match &self.title_bar {
                    Some(title_bar) => Some(&**title_bar),
                    None => None,
                }
            },
            WidgetIndexPair(1, 0) => {
                match &self.content {
                    Some(content) => Some(&**content),
                    None => None,
                }
            },
            _ => None,
        }
    }
    
    pub fn dyn_widget_mut(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(0, 0) => {
                match &mut self.title_bar {
                    Some(title_bar) => Some(&mut **title_bar),
                    None => None,
                }
            },
            WidgetIndexPair(1, 0) => {
                match &mut self.content {
                    Some(content) => Some(&mut **content),
                    None => None,
                }
            },
            _ => None,
        }
    }

    pub fn point(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    {
        let idx_pair = match &self.title_bar {
            Some(title_bar) => {
                if title_bar.bounds().to_f64_rect().contains(pos) {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            None => None,
        };
        if idx_pair.is_none() {
            match &self.content {
                Some(content) => {
                    if content.bounds().to_f64_rect().contains(pos) {
                        Some(WidgetIndexPair(1, 0))
                    } else {
                        None
                    }
                },
                None => None,
            }
        } else {
            idx_pair
        }
    }
    
    pub fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        let mut area_size2 = area_size;
        match &mut self.title_bar {
            Some(title_bar) => {
                title_bar.update_size(cairo_context, theme, area_size)?;
                match area_size2.height {
                    Some(area_height2) => {
                        if area_height2 >= title_bar.margin_height() {
                            area_size2.height = Some(area_height2 - title_bar.margin_height());
                        } else {
                            area_size2.height = Some(0);
                        }
                    },
                    None => (),
                }
            },
            None => (),
        }
        match &mut self.content {
            Some(content) => content.update_size(cairo_context, theme, area_size2)?,
            None => (),
        }
        match (&mut self.title_bar, &self.content) {
            (Some(title_bar), Some(content)) => {
                if area_size.width.is_none() || self.has_trimmed_width {
                    let area_size3 = Size::new(Some(content.margin_width()), Some(title_bar.margin_height()));
                    title_bar.update_size(cairo_context, theme, area_size3)?;
                }
            },
            (Some(title_bar), None) => {
                if self.has_trimmed_width {
                    let area_size3 = Size::new(Some(0), Some(title_bar.margin_height()));
                    title_bar.update_size(cairo_context, theme, area_size3)?;
                }
            },
            _ => (),
        }
        Ok(())
    }
    
    pub fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>
    {
        let mut area_bounds2 = area_bounds;
        match &mut self.title_bar {
            Some(title_bar) => {
                let area_bounds3 = Rect::new(area_bounds.x, area_bounds.y, area_bounds.width, title_bar.margin_height());
                title_bar.update_pos(cairo_context, theme, area_bounds3)?;
                area_bounds2.y += title_bar.margin_height();
                area_bounds2.height -= title_bar.margin_height();
            },
            None => (),
        }
        match &mut self.content {
            Some(content) => content.update_pos(cairo_context, theme, area_bounds2)?,
            None => (),
        }
        Ok(())
    }
    
    pub fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    {
        match &self.title_bar {
            Some(title_bar) => title_bar.draw(cairo_context, theme, is_focused_window)?,
            None => (),
        }
        match &self.content {
            Some(content) => content.draw(cairo_context, theme, is_focused_window)?,
            None => (),
        }
        Ok(())
    }
    
    pub fn padding_size(&self, area_size: Size<Option<i32>>) -> Size<i32>
    {
        let size = match (&self.title_bar, &self.content) {
            (Some(title_bar), Some(content)) => Size::new(max(title_bar.margin_width(), content.margin_width()), title_bar.margin_height() + content.margin_height()),
            (Some(title_bar), None) => Size::new(title_bar.margin_width(), title_bar.margin_height()),
            (None, Some(content)) => Size::new(content.margin_width(), content.margin_height()),
            (None, None) => Size::new(0, 0),
        };
        let width = if self.has_trimmed_width {
            size.width
        } else {
            width_for_opt_width(size.width, area_size.width)
        };
        let height = if self.has_trimmed_height {
            size.height
        } else {
            height_for_opt_height(size.height, area_size.height)
        };
        Size::new(width, height)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;
    use crate::image::*;
    use crate::preferred_size::*;
    use crate::widgets::*;

    #[test]
    fn test_two_window_widgets_give_previous_widget_index_pairs()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_previous_widget_index_pairs_for_no_content()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_previous_widget_index_pairs_for_no_title_bar()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.content = Some(Box::new(Button::new("B")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_previous_widget_index_pairs_for_no_title_bar_and_no_content()
    {
        let widgets = TwoWindowWidgets::new();
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_previous_widget_index_pairs_for_bad_index()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        let idx_pair = widgets.prev(Some(WidgetIndexPair(0, 1)));
        assert_eq!(None, idx_pair);
    }
    
    #[test]
    fn test_two_window_widgets_give_next_widget_index_pairs()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_next_widget_index_pairs_for_no_content()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_next_widget_index_pairs_for_no_title_bar()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.content = Some(Box::new(Button::new("B")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_next_widget_index_pairs_for_no_title_bar_and_no_content()
    {
        let widgets = TwoWindowWidgets::new();
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_next_widget_index_pairs_for_bad_index()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        let idx_pair = widgets.next(Some(WidgetIndexPair(0, 1)));
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_two_window_widgets_give_title_bar()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        match widgets.dyn_widget(WidgetIndexPair(0, 0)) {
            Some(widget) => assert_eq!(Some(()), dyn_widget_as_widget(widget).map(|_: &TitleBar| ())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_two_window_widgets_give_content()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        match widgets.dyn_widget(WidgetIndexPair(1, 0)) {
            Some(widget) => assert_eq!(Some("B"), dyn_widget_as_widget(widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_two_window_widgets_do_not_give_widget_for_bad_index()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        match widgets.dyn_widget(WidgetIndexPair(0, 1)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_two_window_widgets_give_mutable_title_bar()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        match widgets.dyn_widget_mut(WidgetIndexPair(0, 0)) {
            Some(widget) => assert_eq!(Some(()), dyn_widget_mut_as_widget_mut(widget).map(|_: &mut TitleBar| ())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_two_window_widgets_give_mutable_content()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        match widgets.dyn_widget_mut(WidgetIndexPair(1, 0)) {
            Some(widget) => assert_eq!(Some("B"), dyn_widget_mut_as_widget_mut(widget).map(|b: &mut Button| b.text())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_two_window_widgets_do_not_give_mutable_widget_for_bad_index()
    {
        let mut widgets = TwoWindowWidgets::new();
        widgets.title_bar = Some(Box::new(TitleBar::new()));
        widgets.content = Some(Box::new(Button::new("B")));
        match widgets.dyn_widget_mut(WidgetIndexPair(0, 1)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_filled_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_h_align(HAlign::Fill);
        button.set_v_align(VAlign::Fill);
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_small_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(44, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(44, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(44, 34), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(40, 30), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_filled_small_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_h_align(HAlign::Fill);
        button.set_v_align(VAlign::Fill);
        button.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(44, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(44, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(44, 34), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(40, 30), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 120 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(134, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(134, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_filled_content_and_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_h_align(HAlign::Fill);
        button.set_v_align(VAlign::Fill);
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 120 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(134, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(134, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(134, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(130, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_area_width_and_trimmed_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        widgets.has_trimmed_width = true;
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 120 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_filled_content_and_area_width_and_trimmed_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        widgets.has_trimmed_width = true;
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_h_align(HAlign::Fill);
        button.set_v_align(VAlign::Fill);
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 120 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(134, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(134, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(134, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(130, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = (font_height.ceil() as i32) + 8 + 60 + 4 + 10;
        let area_size = Size::new(None, Some(area_height));
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_filled_content_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_h_align(HAlign::Fill);
        button.set_v_align(VAlign::Fill);
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = (font_height.ceil() as i32) + 8 + 60 + 4 + 10;
        let area_size = Size::new(None, Some(area_height));
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 74), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 70), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_area_height_and_trimmed_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        widgets.has_trimmed_height = true;
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = (font_height.ceil() as i32) + 8 + 60 + 4 + 10;
        let area_size = Size::new(None, Some(area_height));
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_filled_content_and_area_height_and_trimmed_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        widgets.has_trimmed_height = true;
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_h_align(HAlign::Fill);
        button.set_v_align(VAlign::Fill);
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = (font_height.ceil() as i32) + 8 + 60 + 4 + 10;
        let area_size = Size::new(None, Some(area_height));
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 74), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 70), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(20, 10 + (font_height.ceil() as i32) + 8), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + (font_height.ceil() as i32) + 8 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_no_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        theme.set_title_font(&cairo_context).unwrap();
        let t = cairo_context.text_extents("T").unwrap().x_advance;
        let text_width = t;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(24 * 3 + (text_width.ceil() as i32) + 4, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(24 * 3 + (text_width.ceil() as i32) + 4, (font_height.ceil() as i32) + 8), widgets.title_bar.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20, 10), widgets.title_bar.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_no_title_bar()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Size::new(124, 64), widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), widgets.content.as_ref().unwrap().size());
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.content.as_ref().unwrap().pos());
    }

    #[test]
    fn test_two_window_widgets_update_size_and_position_for_no_title_bar_and_no_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_two_window_widgets_give_padding_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(124, (font_height.ceil() as i32) + 8 + 64);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_give_padding_size_for_small_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(44, (font_height.ceil() as i32) + 8 + 34);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_give_padding_size_for_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 120 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(134, (font_height.ceil() as i32) + 8 + 64);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_give_padding_size_for_area_width_and_trimmed_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        widgets.has_trimmed_width = true;
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_width = 120 + 4 + 10;
        let area_size = Size::new(Some(area_width), None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(124, (font_height.ceil() as i32) + 8 + 64);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_give_padding_size_for_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = (font_height.ceil() as i32) + 8 + 60 + 4 + 10;
        let area_size = Size::new(None, Some(area_height));
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(124, (font_height.ceil() as i32) + 8 + 74);
        assert_eq!(expected_padding_size, padding_size);
    }


    #[test]
    fn test_two_window_widgets_give_padding_size_for_area_height_and_trimmed_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        widgets.has_trimmed_height = true;
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_height = (font_height.ceil() as i32) + 8 + 60 + 4 + 10;
        let area_size = Size::new(None, Some(area_height));
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(124, (font_height.ceil() as i32) + 8 + 64);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_give_padding_size_for_no_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        theme.set_title_font(&cairo_context).unwrap();
        let t = cairo_context.text_extents("T").unwrap().x_advance;
        let text_width = t;
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(24 * 3 + (text_width.ceil() as i32) + 4, (font_height.ceil() as i32) + 8);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_give_padding_size_for_no_title_bar()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(124, 64);
        assert_eq!(expected_padding_size, padding_size);
    }


    #[test]
    fn test_two_window_widgets_give_padding_size_for_no_title_bar_and_no_content()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let expected_padding_size = Size::new(0, 0);
        assert_eq!(expected_padding_size, padding_size);
    }

    #[test]
    fn test_two_window_widgets_point_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        widgets.update_pos(&cairo_context, &theme, area_bounds).unwrap();
        match widgets.point(Pos::new(25.0, 15.0)) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(50.0, 65.0)) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
    }

    #[test]
    fn test_two_window_widgets_do_not_point_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = TwoWindowWidgets::new();
        let mut title_bar = TitleBar::new();
        title_bar.add(TitleButton::new(TitleButtonIcon::Menu));
        title_bar.add(Title::new("T"));
        title_bar.add(TitleButton::new(TitleButtonIcon::Maximize));
        title_bar.add(TitleButton::new(TitleButtonIcon::Close));
        widgets.title_bar = Some(Box::new(title_bar));
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        widgets.content = Some(Box::new(button));
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size).unwrap();
        let padding_size = widgets.padding_size(area_size);
        let area_bounds = Rect::new(20, 10, padding_size.width, padding_size.height);
        widgets.update_pos(&cairo_context, &theme, area_bounds).unwrap();
        match widgets.point(Pos::new(15.0, 25.0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
