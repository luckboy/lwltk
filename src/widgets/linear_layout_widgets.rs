//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::Ordering;
use std::cmp::max;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;

pub struct LinearLayoutWidgets
{
    pub widgets: Vec<Box<dyn Widget>>,
    pub zero_weight_width_sum: i32,
    pub weight_sum: u32,
    pub weight_width: i32,
    pub weight_width_rem: i32,
}

impl LinearLayoutWidgets
{
    pub fn new() -> Self
    {
        LinearLayoutWidgets {
            widgets: Vec::new(),
            zero_weight_width_sum: 0,
            weight_sum: 0,
            weight_width: 0,
            weight_width_rem: 0,
        }
    }
    
    pub fn add_dyn(&mut self, widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        let i = self.widgets.len();
        self.widgets.push(widget);
        Some(WidgetIndexPair(i, 0))
    }
    
    pub fn insert_dyn(&mut self, idx_pair: WidgetIndexPair, widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        if idx_pair.1 == 0 {
            let i = idx_pair.0;
            if i <= self.widgets.len() {
                self.widgets.insert(i, widget);
                Some(idx_pair)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn remove(&mut self, idx_pair: WidgetIndexPair) -> Option<Box<dyn Widget>>
    {
        if idx_pair.1 == 0 {
            let i = idx_pair.0;
            if i < self.widgets.len() {
                Some(self.widgets.remove(i))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn remove_last(&mut self) -> Option<Box<dyn Widget>>
    { self.widgets.pop() }
    
    pub fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if !self.widgets.is_empty() {
                    Some(WidgetIndexPair(self.widgets.len() - 1, 0))
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(i, 0)) => {
                match i.checked_sub(1) {
                    Some(j) if j < self.widgets.len() => Some(WidgetIndexPair(j, 0)),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if !self.widgets.is_empty() {
                    Some(WidgetIndexPair(0, 0))
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(i, 0)) => {
                match i.checked_add(1) {
                    Some(j) if j < self.widgets.len() => Some(WidgetIndexPair(j, 0)),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub fn dyn_widget(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, 0) => {
                match self.widgets.get(i) {
                    Some(widget) => Some(&**widget),
                    None => None,
                }
            },
            _ => None,
        }
    }

    pub fn dyn_widget_mut(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, 0) => {
                match self.widgets.get_mut(i) {
                    Some(widget) => Some(&mut **widget),
                    None => None,
                }
            },
            _ => None,
        }
    }
    
    pub fn point(&self, pos: Pos<f64>, orient: Orient) -> Option<WidgetIndexPair>
    {
        let res = self.widgets.binary_search_by(|w| {
                let x = orient_pos_x(pos, orient);
                let widget_x = orient_pos_x(w.pos(), orient) as f64;
                let widget_width = orient_size_width(w.size(), orient) as f64;
                if widget_x + widget_width <= x  {
                    Ordering::Less
                } else if widget_x > x {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
        });
        match res {
            Ok(i) => {
                match self.widgets.get(i) {
                    Some(widget) => {
                        
                        if widget.bounds().to_f64_rect().contains(pos) {
                            Some(WidgetIndexPair(i, 0))
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            },
            Err(_) => None,
        }
    }
    
    pub fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>, orient: Orient, h_align: HAlign, v_align: VAlign, preferred_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        let area_size2 = min_opt_size_for_opt_size(area_size, preferred_size);
        let mut widget_area_size = area_size2;
        self.zero_weight_width_sum = 0;
        self.weight_sum = 0;
        for widget in &mut self.widgets {
            let widget_weight = widget.weight();
            if widget_weight > 0 {
                self.weight_sum += widget_weight;
            } else {
                widget.update_size(cairo_context, theme, widget_area_size)?;
                let widget_width = orient_size_width(widget.margin_size(), orient);
                self.zero_weight_width_sum += widget_width;
                match orient_size_width(widget_area_size, orient) {
                    Some(widget_area_width) => set_orient_size_width(&mut widget_area_size, Some(widget_area_width - widget_width), orient),
                    None => (),
                }
            }
        }
        let mut is_weight_width = false;
        match orient_size_width(widget_area_size, orient) {
            Some(widget_area_width) => {
                if self.weight_sum > 0 {
                    self.weight_width = widget_area_width / (self.weight_sum as i32);
                    self.weight_width_rem = widget_area_width % (self.weight_sum as i32);
                    is_weight_width = true;
                }
            },
            None => (),
        }
        let mut rem_count = 0;
        let mut max_weight_width = 0;
        for widget in &mut self.widgets {
            let widget_weight = widget.weight();
            if widget_weight > 0 {
                if is_weight_width {
                    widget_area_size = orient_size(Some(self.weight_width * (widget_weight as i32)), orient_size_height(widget_area_size, orient), orient);
                    match orient_size_width(widget_area_size, orient) {
                        Some(widget_area_width) => {
                            if rem_count + (widget_weight as i32) <= self.weight_width_rem {
                                set_orient_size_width(&mut widget_area_size, Some(widget_area_width + (widget_weight as i32)), orient);
                            } else if rem_count < self.weight_width_rem {
                                set_orient_size_width(&mut widget_area_size, Some(widget_area_width + self.weight_width_rem - rem_count), orient);
                            }
                        },
                        None => (),
                    }
                } else {
                    widget_area_size = orient_size(None, orient_size_height(area_size2, orient), orient);
                }
                widget.update_size(cairo_context, theme, widget_area_size)?;
                let widget_width = orient_size_width(widget.margin_size(), orient);
                max_weight_width = max(max_weight_width, widget_width / (widget.weight() as i32));
                rem_count += widget.weight() as i32;
            }
        }
        if is_weight_width {
            match orient {
                Orient::Horizontal => {
                    match h_align {
                        HAlign::Fill => (),
                        _ => {
                            if preferred_size.width.is_none() {
                                if max_weight_width <= self.weight_width {
                                    self.weight_width = max_weight_width;
                                    self.weight_width_rem = 0;
                                }
                            }
                        },
                    }
                },
                Orient::Vertical => {
                    match v_align {
                        VAlign::Fill => (),
                        _ => {
                            if preferred_size.height.is_none() {
                                if max_weight_width <= self.weight_width {
                                    self.weight_width = max_weight_width;
                                    self.weight_width_rem = 0;
                                }
                            }
                        },
                    }
                },
            }
        } else {
            self.weight_width = max_weight_width;
            self.weight_width_rem = 0;
        }
        if area_size2.width.is_none() || area_size2.height.is_none() {
            rem_count = 0;
            let max_widget_height = self.max_widget_height(orient);
            for widget in &mut self.widgets {
                let widget_weight = widget.weight();
                if widget_weight > 0 {
                    widget_area_size = orient_size(Some(self.weight_width * (widget_weight as i32)), Some(max_widget_height), orient);
                    match orient_size_width(widget_area_size, orient) {
                        Some(widget_area_width) => {
                            if rem_count + (widget_weight as i32) <= self.weight_width_rem {
                                set_orient_size_width(&mut widget_area_size, Some(widget_area_width + (widget_weight as i32)), orient);
                            } else if rem_count < self.weight_width_rem {
                                set_orient_size_width(&mut widget_area_size, Some(widget_area_width + self.weight_width_rem - rem_count), orient);
                            }
                        },
                        None => (),
                    }
                    rem_count += widget.weight() as i32;
                } else {
                    widget_area_size = orient_size(Some(orient_size_width(widget.margin_size(), orient)), Some(max_widget_height), orient);
                }
                let mut is_updating = false;
                match widget.h_align() {
                    HAlign::Fill => is_updating |= area_size2.width.is_none(),
                    _ => (),
                }
                match widget.v_align() {
                    VAlign::Fill => is_updating |= area_size2.height.is_none(),
                    _ => (),
                }
                if is_updating {
                    widget.update_size(cairo_context, theme, widget_area_size)?;
                }
            }
        }
        Ok(())
    }
    
    pub fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>, orient: Orient, h_align: HAlign, v_align: VAlign, preferred_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        let size = self.size(Size::new(Some(area_bounds.width), Some(area_bounds.height)), orient, h_align, v_align, preferred_size);
        let mut pos = pos_for_h_align_and_v_align(size, area_bounds, h_align, v_align);
        let mut rem_count = 0;
        for widget in &mut self.widgets {
            let widget_weight = widget.weight();
            if widget_weight > 0 {
                let mut widget_area_bounds = orient_rect(orient_pos_x(pos, orient), orient_pos_y(pos, orient), self.weight_width * (widget_weight as i32), orient_size_height(size, orient), orient);
                let widget_area_width = orient_rect_width(widget_area_bounds, orient);
                if rem_count + (widget_weight as i32) <= self.weight_width_rem {
                    set_orient_rect_width(&mut widget_area_bounds, widget_area_width + (widget_weight as i32), orient);
                } else if rem_count < self.weight_width_rem {
                    set_orient_rect_width(&mut widget_area_bounds, widget_area_width + self.weight_width_rem - rem_count, orient);
                }
                widget.update_pos(cairo_context, theme, widget_area_bounds)?;
                let x = orient_pos_x(pos, orient);
                set_orient_pos_x(&mut pos, x + orient_rect_width(widget_area_bounds, orient), orient);
                rem_count += widget.weight() as i32;
            } else {
                let widget_area_bounds = orient_rect(orient_pos_x(pos, orient), orient_pos_y(pos, orient), orient_size_width(widget.margin_size(), orient), orient_size_height(size, orient), orient);
                widget.update_pos(cairo_context, theme, widget_area_bounds)?;
                let x = orient_pos_x(pos, orient);
                set_orient_pos_x(&mut pos, x + orient_rect_width(widget_area_bounds, orient), orient);
            }
        }
        Ok(())
    }
    
    pub fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    {
        for widget in &self.widgets {
            widget.draw(cairo_context, theme, is_focused_window)?;
        }
        Ok(())
    }
    
    pub fn max_widget_height(&self, orient: Orient) -> i32
    { self.widgets.iter().fold(0, |w, w2| max(w, orient_size_height(w2.margin_size(), orient))) }
    
    pub fn size(&self, area_size: Size<Option<i32>>, orient: Orient, h_align: HAlign, v_align: VAlign, preferred_size: Size<Option<i32>>) -> Size<i32>
    {
        let width_sum = self.zero_weight_width_sum + self.weight_width * (self.weight_sum as i32) + self.weight_width_rem;
        let width = match orient {
            Orient::Horizontal => {
                let area_width2 = min_opt_width_for_opt_width(area_size.width, preferred_size.width);
                if preferred_size.width.is_none() {
                    width_for_h_align(width_sum, area_width2, h_align)
                } else {
                    area_width2.unwrap_or(width_sum)
                }
            },
            Orient::Vertical => {
                let area_height2 = min_opt_height_for_opt_height(area_size.height, preferred_size.height);
                if preferred_size.height.is_none() {
                    height_for_v_align(width_sum, area_height2, v_align)
                } else {
                    area_height2.unwrap_or(width_sum)
                }
            },
        };
        let max_height = self.max_widget_height(orient);
        let height = match orient {
            Orient::Horizontal => {
                let area_height2 = min_opt_height_for_opt_height(area_size.height, preferred_size.height);
                if preferred_size.height.is_none() {
                    height_for_v_align(max_height, area_height2, v_align)
                } else {
                    area_height2.unwrap_or(max_height)
                }
            },
            Orient::Vertical => {
                let area_width2 = min_opt_width_for_opt_width(area_size.width, preferred_size.width);
                if preferred_size.width.is_none() {
                    width_for_h_align(max_height, area_width2, h_align)
                } else {
                    area_width2.unwrap_or(max_height)
                }
            },
        };
        orient_size(width, height, orient)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;
    use crate::widgets::button::*;
    use crate::preferred_size::*;

    #[test]
    fn test_linear_layout_widgets_add_widgets()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        assert_eq!(3, widgets.widgets.len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[2]).map(|b: &Button| b.text()));
    }
    
    #[test]
    fn test_linear_layout_widgets_insert_widgets()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.insert_dyn(WidgetIndexPair(1, 0), Box::new(Button::new("B4"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.insert_dyn(WidgetIndexPair(4, 0), Box::new(Button::new("B5"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(4, 0), idx_pair),
            None => assert!(false),
        }
        assert_eq!(5, widgets.widgets.len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[2]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[3]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[4]).map(|b: &Button| b.text()));
    }
    
    #[test]
    fn test_linear_layout_widgets_do_not_insert_widget_for_too_large_index()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.insert_dyn(WidgetIndexPair(4, 0), Box::new(Button::new("B4"))) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(3, widgets.widgets.len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[2]).map(|b: &Button| b.text()));
    }    

    #[test]
    fn test_linear_layout_widgets_remove_widget()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.remove(WidgetIndexPair(1, 0)) {
            Some(widget) => assert_eq!(Some("B2"), dyn_widget_as_widget(&*widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[1]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_linear_layout_widgets_do_not_remove_widget_for_too_large_index()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.remove(WidgetIndexPair(3, 0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(3, widgets.widgets.len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[2]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_linear_layout_widgets_remove_last_widget()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.remove_last() {
            Some(widget) => assert_eq!(Some("B3"), dyn_widget_as_widget(&*widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[1]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_linear_layout_widgets_do_not_remove_last_widget_for_no_widgets()
    {
        let mut widgets = LinearLayoutWidgets::new();
        match widgets.remove_last() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(0, widgets.widgets.len());
    }

    #[test]
    fn test_linear_layout_widgets_give_previous_widget_index_pairs()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(2, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_linear_layout_widgets_do_not_give_previous_widget_index_pair_for_bad_index()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        let idx_pair = widgets.prev(Some(WidgetIndexPair(0, 1)));
        assert_eq!(None, idx_pair);
    }    
    
    #[test]
    fn test_linear_layout_widgets_give_next_widget_index_pairs()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(2, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_linear_layout_widgets_do_not_give_next_widget_index_pair_for_bad_index()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        let idx_pair = widgets.next(Some(WidgetIndexPair(0, 1)));
        assert_eq!(None, idx_pair);
    }

    #[test]
    fn test_linear_layout_widgets_give_widget()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        match widgets.dyn_widget(WidgetIndexPair(1, 0)) {
            Some(widget) => assert_eq!(Some("B2"), dyn_widget_as_widget(widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_linear_layout_widgets_do_not_give_widget_for_bad_index()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        match widgets.dyn_widget(WidgetIndexPair(0, 1)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_linear_layout_widgets_give_mutable_widget()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        match widgets.dyn_widget_mut(WidgetIndexPair(1, 0)) {
            Some(widget) => assert_eq!(Some("B2"), dyn_widget_mut_as_widget_mut(widget).map(|b: &mut Button| b.text())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_linear_layout_widgets_do_not_give_mutable_widget_for_bad_index()
    {
        let mut widgets = LinearLayoutWidgets::new();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        match widgets.dyn_widget_mut(WidgetIndexPair(0, 1)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
    }    
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_v_align(VAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 54), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 50), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 54), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 50), widgets.widgets[2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(94, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(90, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 47, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 47 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 141, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 141 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(47, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(43, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(94, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(90, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 47, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 47 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 141, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 141 + 2, 10 + 2), widgets.widgets[3].pos());
    }    
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 50 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(94, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(90, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 47, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 47 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 141, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 47 + 54 + 141 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_and_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 50 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(51, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(47, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(151, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(147, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(100, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(96, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151 + 2, 10 + 2), widgets.widgets[3].pos());
    }    
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 50 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(94, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(90, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_and_area_width_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 50 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(51, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(47, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(151, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(147, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(100, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(96, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151 + 2, 10 + 2), widgets.widgets[3].pos());
    }
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 50 * (1 + 3 + 2) + 4;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(94, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(90, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 153, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 153 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_and_area_width_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 50 * (1 + 3 + 2) + 4;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(51, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(47, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(153, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(149, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(100, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(96, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 153, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 153 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width_and_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 50 + 4 + 50 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(Some(preferred_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(94, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(90, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_and_area_width_and_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 50 + 4 + 50 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(Some(preferred_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(51, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(47, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1].size());
        assert_eq!(Size::new(151, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(147, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(100, 34), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(96, 30), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 51, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 51 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151, 10), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 51 + 54 + 151 + 2, 10 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_horizontal_orientation_and_no_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 0;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
    }    
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 40 + 4 + 30 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 54 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 44 + 2), widgets.widgets[2].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_h_align(HAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 40 + 4 + 30 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(64, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(60, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(64, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(60, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 54 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 44 + 2), widgets.widgets[2].pos());
    }    
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 37;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 74), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 70), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 37), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 37 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 37 + 44 + 111), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 44 + 111 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 37;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 37), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 33), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 74), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 70), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 37), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 37 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 37 + 44 + 111), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 44 + 111 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 40 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 37;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 74), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 70), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 37), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 37 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 37 + 44 + 111), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 37 + 44 + 111 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 40 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 41), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 37), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 121), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 117), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 80), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 76), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 121), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 121 + 2), widgets.widgets[3].pos());
    }    
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 40 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 74), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 70), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 121), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 121 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 40 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 41), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 37), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 121), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 117), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 80), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 76), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 121), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 121 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 40 * (1 + 3 + 2) + 4;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 74), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 70), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 123), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 123 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 40 * (1 + 3 + 2) + 4;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 41), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 37), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 123), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 119), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 80), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 76), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 123), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 123 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height_and_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_height = 40 + 4 + 40 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(None, Some(preferred_height));
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 74), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 70), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 121), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 121 + 2), widgets.widgets[3].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height_and_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_height = 40 + 4 + 40 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(None, Some(preferred_height));
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 40;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 41), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 37), widgets.widgets[0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(44, 121), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(40, 117), widgets.widgets[2].size());
        assert_eq!(Size::new(44, 80), widgets.widgets[3].margin_size());
        assert_eq!(Size::new(40, 76), widgets.widgets[3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20, 10 + 41), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 2), widgets.widgets[2].pos());
        assert_eq!(Pos::new(20, 10 + 41 + 44 + 121), widgets.widgets[3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 41 + 44 + 121 + 2), widgets.widgets[3].pos());
    }
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_vertical_orientation_and_no_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 0;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_left_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
    }    

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_center_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Center;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20 + 5, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2 + 5, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44 + 5, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2 + 5, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 5, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2 + 5, 10 + 2), widgets.widgets[2].pos());
    }    

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_right_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Right;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20 + 10, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2 + 10, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44 + 10, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2 + 10, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 10, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2 + 10, 10 + 2), widgets.widgets[2].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_fill_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
    }
    
    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_top_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
    }        

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_center_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Center;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10 + 5), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2 + 5), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 5), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2 + 5), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10 + 5), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2 + 5), widgets.widgets[2].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_bottom_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Bottom;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10 + 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2 + 10), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2 + 10), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10 + 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2 + 10), widgets.widgets[2].pos());
    }

    #[test]
    fn test_linear_layout_widgets_update_size_and_position_for_fill_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 44 + 54 + 64 + 10;
        let area_height = 54 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        assert_eq!(Pos::new(20, 10), widgets.widgets[0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1].pos());
        assert_eq!(Pos::new(20 + 44 + 54, 10), widgets.widgets[2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 54 + 2, 10 + 2), widgets.widgets[2].pos());
    }
    
    #[test]
    fn test_linear_layout_wigets_give_size_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 40 + 4 + 50 + 4 + 60 + 4;
        let expected_height = 50 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_wigets_give_size_for_horizontal_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(90), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 50 + 4 + 47 + 141 + 94;
        let expected_height = 30 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_wigets_give_size_for_horizontal_orientation_and_area_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let area_height = 50 + 4 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 40 + 4 + 50 + 4 + 60 + 4;
        let expected_height = 50 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_wigets_give_size_for_horizontal_orientation_and_area_size_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let area_height = 50 + 4 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let expected_height = 50 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_wigets_give_size_for_horizontal_orientation_and_area_size_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 40 + 4 + 50 + 4 + 60 + 4 + 20;
        let area_height = 50 + 4 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let preferred_height = 50 + 4 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let expected_height = 50 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_wigets_give_size_for_horizontal_orientation_and_area_size_and_fill_alignment_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 40 + 4 + 50 + 4 + 60 + 4 + 20;
        let area_height = 50 + 4 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let preferred_height = 50 + 4 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 40 + 4 + 50 + 4 + 60 + 4 + 10;
        let expected_height = 50 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }
    
    #[test]
    fn test_linear_layout_widgets_give_size_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4;
        let expected_height = 50 + 4 + 40 + 4 + 30 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }
    
    #[test]
    fn test_linear_layout_widgets_give_size_for_vertical_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_weight(1);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_weight(3);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_weight(2);
        button4.set_preferred_size(Size::new(Some(40), Some(70)));
        widgets.add_dyn(Box::new(button4));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 40 + 4;
        let expected_height = 40 + 4 + 37 + 111 + 74;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_widgets_give_size_for_vertical_orientation_and_area_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 60 + 4 + 10;
        let area_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4;
        let expected_height = 50 + 4 + 40 + 4 + 30 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_widgets_give_size_for_vertical_orientation_and_area_size_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 60 + 4 + 10;
        let area_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 10;
        let expected_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_widgets_give_size_for_vertical_orientation_and_area_size_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 60 + 4 + 20;
        let area_height = 50 + 4 + 40 + 4 + 30 + 4 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 60 + 4 + 10;
        let preferred_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 10;
        let expected_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_linear_layout_widgets_give_size_for_vertical_orientation_and_area_size_and_fill_alignment_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_width = 60 + 4 + 20;
        let area_height = 50 + 4 + 40 + 4 + 30 + 4 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_width = 60 + 4 + 10;
        let preferred_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 10;
        let expected_height = 50 + 4 + 40 + 4 + 30 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }
    
    #[test]
    fn test_linear_layout_widgets_point_widgets_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size).unwrap();
        match widgets.point(Pos::new(25.0, 15.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(70.0, 45.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(120.0, 12.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
    }

    #[test]
    fn test_linear_layout_widgets_do_not_point_widget_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size).unwrap();
        match widgets.point(Pos::new(10.0, 15.0), orient) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_linear_layout_widgets_point_widgets_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size).unwrap();
        match widgets.point(Pos::new(25.0, 15.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(50.0, 75.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(22.0, 110.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
    }

    #[test]
    fn test_linear_layout_widgets_do_not_point_widget_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = LinearLayoutWidgets::new();
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size).unwrap();
        match widgets.point(Pos::new(25.0, 10.0), orient) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
