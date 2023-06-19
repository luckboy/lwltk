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

    pub fn dyn_widget_mut(&mut self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, 0) => {
                match self.widgets.get_mut(i) {
                    Some(widget) => Some(&**widget),
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
                if x < widget_x {
                    Ordering::Less
                } else if x >= widget_x + widget_width {
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
            if widget.weight() > 0 {
                self.weight_sum += widget.weight();
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
            }
            None => (),
        }
        let mut rem_count = 0;
        let mut max_weight_width = 0;
        for widget in &mut self.widgets {
            if widget.weight() > 0 {
                if is_weight_width {
                    widget_area_size = orient_size(Some(self.weight_width * (widget.weight() as i32)), orient_size_height(widget_area_size, orient), orient);
                    match orient_size_width(widget_area_size, orient) {
                        Some(widget_area_width) => {
                            if rem_count + (widget.weight() as i32) <= self.weight_width_rem {
                                set_orient_size_width(&mut widget_area_size, Some(widget_area_width + (widget.weight() as i32)), orient);
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
                let width = orient_size_width(widget.margin_size(), orient);
                max_weight_width = max(max_weight_width, width / (widget.weight() as i32));
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
                                self.weight_width = max_weight_width;
                                self.weight_width_rem = 0;
                            }
                        },
                    }
                },
                Orient::Vertical => {
                    match v_align {
                        VAlign::Fill => (),
                        _ => {
                            if preferred_size.height.is_none() {
                                self.weight_width = max_weight_width;
                                self.weight_width_rem = 0;
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
                if widget.weight() > 0 {
                    widget_area_size = orient_size(Some(self.weight_width * (widget.weight() as i32)), Some(max_widget_height), orient);
                    match orient_size_width(widget_area_size, orient) {
                        Some(widget_area_width) => {
                            if rem_count + (widget.weight() as i32) <= self.weight_width_rem {
                                set_orient_size_width(&mut widget_area_size, Some(widget_area_width + (widget.weight() as i32)), orient);
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
                match h_align {
                    HAlign::Fill => is_updating |= area_size2.width.is_none(),
                    _ => (),
                }
                match v_align {
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
    
    pub fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>, orient: Orient, h_align: HAlign, v_align: VAlign) -> Result<(), CairoError>
    {
        let size = self.size(orient);
        let mut pos = pos_for_h_align_and_v_align(size, area_bounds, h_align, v_align);
        let mut rem_count = 0;
        for widget in &mut self.widgets {
            if widget.weight() > 0 {
                let mut widget_area_bounds = orient_rect(orient_pos_x(pos, orient), orient_pos_y(pos, orient), self.weight_width * (widget.weight() as i32), orient_size_height(size, orient), orient);
                let widget_area_width = orient_rect_width(widget_area_bounds, orient);
                if rem_count + (widget.weight() as i32) <= self.weight_width_rem {
                    set_orient_rect_width(&mut widget_area_bounds, widget_area_width + (widget.weight() as i32), orient);
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
    { self.widgets.iter().fold(0, |w, w2| max(w, orient_size_width(w2.size(), orient))) }
    
    pub fn size(&self, orient: Orient) -> Size<i32>
    { orient_size(self.zero_weight_width_sum + self.weight_width * (self.weight_sum as i32) + self.weight_width_rem, self.max_widget_height(orient), orient) }
}
