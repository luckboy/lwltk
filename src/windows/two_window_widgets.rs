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
    pub title_bar: Option<Box<dyn Widget>>,
    pub content: Option<Box<dyn Widget>>,
}

impl TwoWindowWidgets
{
    pub fn new() -> Self
    { TwoWindowWidgets { title_bar: None, content: None, } }

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
                if area_size.width.is_none() {
                    let area_size3 = Size::new(Some(title_bar.margin_width()), Some(content.margin_height()));
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
        size_for_opt_size(size, area_size)
    }
}
