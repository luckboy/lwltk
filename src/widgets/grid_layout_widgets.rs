//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::Ordering;
use std::cmp::max;
use std::collections::BTreeMap;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;

pub struct GridLayoutWidgetPair
{
    pub count: usize,
    pub max_width: i32,
}

impl GridLayoutWidgetPair
{
    pub fn new() -> Self
    { GridLayoutWidgetPair { count: 1, max_width: 0, } }
}

pub struct GridLayoutWidgets
{
    pub max_column_count: usize,
    pub widgets: Vec<Vec<Box<dyn Widget>>>,
    pub zero_weight_pairs: BTreeMap<u32, GridLayoutWidgetPair>,
    pub zero_weight_width_sum: i32,
    pub weight_sum: u32,
    pub weight_width: i32,
    pub weight_width_rem: i32,
    pub row_height: i32,
    pub row_height_rem: i32,
    pub start_y: i32,
}

fn weight_and_weight_index(widget: &dyn Widget, zero_weight_pairs: &BTreeMap<u32, GridLayoutWidgetPair>, weight_idx: u32) -> (u32, u32)
{
    let weight = widget.weight();
    if weight > 0 {
        (weight, weight_idx + weight)
    } else {
        if zero_weight_pairs.contains_key(&weight_idx) {
            (0, weight_idx + 1)
        } else {
            (1, weight_idx + 1)
        }
    }
}

impl GridLayoutWidgets
{
    pub fn new(max_column_count: usize) -> Self
    {
        GridLayoutWidgets {
            max_column_count,
            widgets: Vec::new(),
            zero_weight_pairs: BTreeMap::new(),
            zero_weight_width_sum: 0,
            weight_sum: 0,
            weight_width: 0,
            weight_width_rem: 0,
            row_height: 0,
            row_height_rem: 0,
            start_y: 0,
        }
    }

    pub fn add_dyn(&mut self, widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        let i = self.widgets.len();
        let last_row = match self.widgets.last_mut() {
            Some(row) => {
                if row.len() < self.max_column_count {
                    Some(row)
                } else {
                    None
                }
            },
            None => None,
        };
        match last_row {
            Some(last_row) => {
                let j = last_row.len();
                last_row.push(widget);
                Some(WidgetIndexPair(i - 1, j))
            },
            None => {
                self.widgets.push(vec![widget]);
                Some(WidgetIndexPair(i, 0))
            },
        }
    }
    
    pub fn add_empty_row(&mut self) -> bool
    {
        self.widgets.push(Vec::new());
        true
    }
    
    pub fn insert_dyn(&mut self, idx_pair: WidgetIndexPair, widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        let i = idx_pair.0;
        let j = idx_pair.1;
        if i == self.widgets.len() && j == 0 {
            self.widgets.push(vec![widget]);
            Some(WidgetIndexPair(i, 0))
        } else if i < self.widgets.len() {
            match self.widgets.get_mut(i) {
                Some(row) => {
                    if j <= row.len() {
                        row.insert(j, widget);
                        Some(WidgetIndexPair(i, j))
                    } else {
                        None
                    }
                },
                None => None,
            }
        } else {
            None
        }
    }

    pub fn remove(&mut self, idx_pair: WidgetIndexPair) -> Option<Box<dyn Widget>>
    {
        let i = idx_pair.0;
        let j = idx_pair.1;
        let len = self.widgets.len();
        if i < len {
            match self.widgets.get_mut(i) {
                Some(row) => {
                    if j < row.len() {
                        let widget = row.remove(j);
                        if i == len - 1 && row.is_empty() {
                            self.widgets.pop();
                        }
                        Some(widget)
                    } else {
                        None
                    }
                },
                None => None,
            }
        } else {
            None
        }
    }
    
    pub fn remove_last(&mut self) -> Option<Box<dyn Widget>>
    {
        match self.widgets.last_mut() {
            Some(row) => {
                match row.pop() {
                   Some(widget) => {
                       if row.is_empty() {
                           self.widgets.pop();
                       }
                       Some(widget)
                   }
                   None => None,
                }
            },
            None => None,
        }
    }

    fn skip_empty_rows_for_prev(&self, i: usize) -> Option<usize>
    {
        if !self.widgets.is_empty() {
            let mut j = i;
            loop {
                match self.widgets.get(j) {
                    Some(row) if row.is_empty() => (),
                    Some(_) => return Some(j),
                    None => break,
                }
                match j.checked_sub(1) {
                    Some(k) if k < self.widgets.len() => j = k,
                    _ => break,
                }
            }
        }
        None
    }
    
    pub fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if !self.widgets.is_empty() {
                    match self.skip_empty_rows_for_prev(self.widgets.len() - 1) {
                        Some(i) => {
                            match self.widgets.get(i) {
                                Some(row) => Some(WidgetIndexPair(i, row.len() - 1)),
                                None => None,
                            }
                        },
                        None => None,
                    }
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(i, j)) => {
                match self.widgets.get(i) {
                    Some(row) => {
                        match j.checked_sub(1) {
                            Some(k) if k < row.len() => Some(WidgetIndexPair(i, k)),
                            _ => {
                                match i.checked_sub(1) {
                                    Some(k) if k < self.widgets.len() => {
                                        match self.skip_empty_rows_for_prev(k) {
                                            Some(l) => {
                                                match self.widgets.get(l) {
                                                    Some(row) => Some(WidgetIndexPair(l, row.len() - 1)),
                                                    None => None,
                                                }
                                            },
                                            None => None,
                                        }
                                    },
                                    _ => None,
                                }
                            },
                        }
                    },
                    None => None,
                }
            },
        }
    }


    fn skip_empty_rows_for_next(&self, i: usize) -> Option<usize>
    {
        if !self.widgets.is_empty() {
            let mut j = i;
            loop {
                match self.widgets.get(j) {
                    Some(row) if row.is_empty() => (),
                    Some(_) => return Some(j),
                    None => break,
                }
                match j.checked_add(1) {
                    Some(k) if k < self.widgets.len() => j = k,
                    _ => break,
                }
            }
        }
        None
    }
    
    pub fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if !self.widgets.is_empty() {
                    match self.skip_empty_rows_for_next(0) {
                        Some(i) => Some(WidgetIndexPair(i, 0)),
                        None => None,
                    }
                } else {
                    None
                }
            },
            Some(WidgetIndexPair(i, j)) => {
                match self.widgets.get(i) {
                    Some(row) => {
                        match j.checked_add(1) {
                            Some(k) if k < row.len() => Some(WidgetIndexPair(i, k)),
                            _ => {
                                match i.checked_add(1) {
                                    Some(k) if k < self.widgets.len() => {
                                        match self.skip_empty_rows_for_next(k) {
                                            Some(l) => Some(WidgetIndexPair(l, 0)),
                                            None => None,
                                        }
                                    },
                                    _ => None,
                                }
                            },
                        }
                    },
                    None => None,
                }
            },
        }
    }
    
    pub fn dyn_widget(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, j) => {
                match self.widgets.get(i) {
                    Some(row) => {
                        match row.get(j) {
                            Some(widget) => Some(&**widget),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
        }
    }
    
    pub fn dyn_widget_mut(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    {
        match idx_pair {
            WidgetIndexPair(i, j) => {
                match self.widgets.get_mut(i) {
                    Some(row) => {
                        match row.get_mut(j) {
                            Some(widget) => Some(&mut **widget),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
        }
    }
    
    pub fn point(&self, pos: Pos<f64>, orient: Orient) -> Option<WidgetIndexPair>
    {
        let start_y = self.start_y as f64;
        let rem_y = start_y + (((self.row_height + 1) * self.row_height_rem) as f64);
        let end_y = rem_y + ((self.row_height * ((self.widgets.len() as i32) - self.row_height_rem)) as f64);
        let y = orient_pos_y(pos, orient);
        let i = if start_y <= y && rem_y > y {
            Some(((y - start_y) / ((self.row_height + 1) as f64)).floor() as usize)
        } else if rem_y <= y && end_y > y {
            if self.row_height != 0 {
                Some(((((y - rem_y) / (self.row_height as f64)).floor() as i32) + self.row_height_rem) as usize)
            } else {
                None
            }
        } else {
            None
        };
        match i {
            Some(i) => {
                match self.widgets.get(i) {
                    Some(row) => {
                        let res = row.binary_search_by(|w| {
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
                            Ok(j) => {
                                match row.get(j) {
                                    Some(widget) => {
                                        if widget.bounds().to_f64_rect().contains(pos) {
                                            Some(WidgetIndexPair(i, j))
                                        } else {
                                            None
                                        }
                                    },
                                    None => None,
                                }
                            },
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
            },
            None => None,
        }
    }
        
    pub fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>, orient: Orient, h_align: HAlign, v_align: VAlign, preferred_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        self.zero_weight_pairs.clear();
        if !self.widgets.is_empty() {
            let mut weight_idx: u32;
            let mut weight_end = 0;
            for row in &self.widgets {
                weight_idx = 0;
                for widget in row {
                    let widget_weight = widget.weight();
                    let weight_inc = if widget_weight > 0 {
                        widget_weight
                    } else {
                        1
                    };
                    weight_idx += weight_inc;
                }
                weight_end = max(weight_end, weight_idx);
            }
            weight_idx = 0;
            for widget in &self.widgets[0] {
                let widget_weight = widget.weight();
                let weight_inc = if widget_weight > 0 {
                    widget_weight
                } else {
                    self.zero_weight_pairs.insert(weight_idx, GridLayoutWidgetPair::new());
                    1
                };
                weight_idx += weight_inc;
            }
            for _ in weight_idx..weight_end {
                self.zero_weight_pairs.insert(weight_idx, GridLayoutWidgetPair::new());
                weight_idx += 1;
            }
            for row in &self.widgets[1..] {
                weight_idx = 0;
                for widget in row {
                    let (widget_weight, tmp_weight_idx) = weight_and_weight_index(&**widget, &self.zero_weight_pairs, weight_idx);
                    if widget_weight <= 0 {
                        match self.zero_weight_pairs.get_mut(&weight_idx) {
                            Some(pair) => pair.count += 1,
                            None => (),
                        }
                    }
                    weight_idx = tmp_weight_idx;
                }
                for _ in weight_idx..weight_end {
                    match self.zero_weight_pairs.get_mut(&weight_idx) {
                        Some(pair) => pair.count += 1,
                        None => (),
                    }
                    weight_idx += 1;
                }
            }
            let weight_pair_keys: Vec<u32> = self.zero_weight_pairs.keys().map(|k| *k).collect();
            for key in &weight_pair_keys {
                match self.zero_weight_pairs.get(key) {
                    Some(pair) => {
                        if pair.count < self.widgets.len() {
                            self.zero_weight_pairs.remove(key);
                        }
                    },
                    None => (),
                }
            }
        }
        let area_size2 = min_opt_size_for_opt_size(area_size, preferred_size);
        let mut widget_area_size = area_size2;
        self.weight_sum = 0;
        let mut is_row_height = false;
        match orient_size_height(widget_area_size, orient) {
            Some(widget_area_height) => {
                if self.widgets.len() > 0 {
                    self.row_height = widget_area_height / (self.widgets.len() as i32);
                    self.row_height_rem = widget_area_height % (self.widgets.len() as i32);
                    is_row_height = true;
                }
            }
            None => (),
        }
        let mut row_rem_count = 0;
        let mut max_row_height = 0;
        for row in &mut self.widgets {
            let mut tmp_weight_sum = 0;
            if is_row_height {
                widget_area_size = orient_size(orient_size_width(area_size2, orient), Some(self.row_height), orient);
                match orient_size_height(widget_area_size, orient) {
                    Some(widget_area_height) => {
                        if row_rem_count < self.row_height_rem {
                            set_orient_size_height(&mut widget_area_size, Some(widget_area_height + 1), orient);
                        }
                    },
                    None => (),
                }
            } else {
                widget_area_size = orient_size(orient_size_width(widget_area_size, orient), None, orient);
            }
            let mut weight_idx = 0;
            for widget in row {
                let (widget_weight, tmp_weight_idx) = weight_and_weight_index(&**widget, &self.zero_weight_pairs, weight_idx);
                if widget_weight > 0 {
                    tmp_weight_sum += widget_weight;
                } else {
                    widget.update_size(cairo_context, theme, widget_area_size)?;
                    let widget_width = orient_size_width(widget.margin_size(), orient);
                    let widget_height = orient_size_height(widget.margin_size(), orient);
                    match self.zero_weight_pairs.get_mut(&weight_idx) {
                        Some(pair) => pair.max_width = max(pair.max_width, widget_width),
                        None => (),
                    }
                    max_row_height = max(max_row_height, widget_height);
                    match orient_size_width(widget_area_size, orient) {
                        Some(widget_area_width) => set_orient_size_width(&mut widget_area_size, Some(widget_area_width - widget_width), orient),
                        None => (),
                    }
                }
                weight_idx = tmp_weight_idx;
            }
            self.weight_sum = max(tmp_weight_sum, self.weight_sum);
            row_rem_count += 1;
        }
        self.zero_weight_width_sum = 0;
        for pair in self.zero_weight_pairs.values() {
            self.zero_weight_width_sum += pair.max_width;
        }
        let mut is_weight_width = false;
        match orient_size_width(area_size2, orient) {
            Some(area_width2) => {
                if self.weight_sum > 0 {
                    let widget_area_width = area_width2 - self.zero_weight_width_sum;
                    self.weight_width = widget_area_width / (self.weight_sum as i32);
                    self.weight_width_rem = widget_area_width % (self.weight_sum as i32);
                    is_weight_width = true;
                }
            },
            None => (),
        }
        let mut max_weight_width = 0;
        for row in &mut self.widgets {
            if is_row_height {
                widget_area_size = orient_size(None, Some(self.row_height), orient);
                match orient_size_height(widget_area_size, orient) {
                    Some(widget_area_height) => {
                        if row_rem_count < self.row_height_rem {
                            set_orient_size_height(&mut widget_area_size, Some(widget_area_height + 1), orient);
                        }
                    },
                    None => (),
                }
            } else {
                widget_area_size = orient_size(None, None, orient);
            }
            let mut weight_idx = 0;
            let mut rem_count = 0;
            for widget in row {
                let (widget_weight, tmp_weight_idx) = weight_and_weight_index(&**widget, &self.zero_weight_pairs, weight_idx);
                if widget_weight > 0 {
                    if is_weight_width {
                        set_orient_size_width(&mut widget_area_size, Some(self.weight_width * (widget_weight as i32)), orient);
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
                    let widget_height = orient_size_height(widget.margin_size(), orient);
                    max_weight_width = max(max_weight_width, widget_width / (widget.weight() as i32));
                    max_row_height = max(max_row_height, widget_height);
                    rem_count += widget.weight() as i32;
                }
                weight_idx = tmp_weight_idx;
            }
            row_rem_count += 1;
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
        if is_row_height {
            match orient {
                Orient::Horizontal => {
                    match v_align {
                        VAlign::Fill => (),
                        _ => {
                            if preferred_size.height.is_none() {
                                self.row_height = max_row_height;
                                self.row_height_rem = 0;
                            }
                        },
                    }
                },
                Orient::Vertical => {
                    match h_align {
                        HAlign::Fill => (),
                        _ => {
                            if preferred_size.width.is_none() {
                                self.row_height = max_row_height;
                                self.row_height_rem = 0;
                            }
                        },
                    }
                },
            }
        } else {
            self.row_height = max_row_height;
            self.row_height_rem = 0;
        }
        if area_size2.width.is_none() || area_size2.height.is_none() {
            for row in &mut self.widgets {
                widget_area_size = orient_size(None, Some(self.row_height), orient);
                match orient_size_height(widget_area_size, orient) {
                    Some(widget_area_height) => {
                        if row_rem_count < self.row_height_rem {
                            set_orient_size_height(&mut widget_area_size, Some(widget_area_height + 1), orient);
                        }
                    },
                    None => (),
                }
                let mut weight_idx = 0;
                let mut rem_count = 0;
                for widget in row {
                    let (widget_weight, tmp_weight_idx) = weight_and_weight_index(&**widget, &self.zero_weight_pairs, weight_idx);
                    if widget_weight > 0 {
                        set_orient_size_width(&mut widget_area_size, Some(self.weight_width * (widget_weight as i32)), orient);
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
                        set_orient_size_width(&mut widget_area_size, Some(orient_size_width(widget.margin_size(), orient)), orient);
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
                    weight_idx = tmp_weight_idx;
                }
                row_rem_count += 1;
            }
        }
        Ok(())
    }
    
    pub fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>, orient: Orient, h_align: HAlign, v_align: VAlign, preferred_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        let size = self.size(Size::new(Some(area_bounds.width), Some(area_bounds.height)), orient, h_align, v_align, preferred_size);
        let mut pos = pos_for_h_align_and_v_align(size, area_bounds, h_align, v_align);
        self.start_y = pos.y;
        let mut row_rem_count = 0;
        for row in &mut self.widgets {
            let mut widget_area_bounds = orient_rect(0, orient_pos_y(pos, orient), 0, self.row_height, orient);
            let widget_area_height = orient_rect_height(widget_area_bounds, orient);
            if row_rem_count < self.row_height_rem {
                set_orient_rect_height(&mut widget_area_bounds, widget_area_height + 1, orient);
            }
            let mut weight_idx = 0;
            let mut rem_count = 0;
            let mut tmp_pos = pos;
            for widget in row {
                let (widget_weight, tmp_weight_idx) = weight_and_weight_index(&**widget, &self.zero_weight_pairs, weight_idx);
                if widget_weight > 0 {
                    set_orient_rect_x(&mut widget_area_bounds, orient_pos_x(tmp_pos, orient), orient);
                    set_orient_rect_width(&mut widget_area_bounds, self.weight_width * (widget_weight as i32), orient);
                    let widget_area_width = orient_rect_width(widget_area_bounds, orient);
                    if rem_count + (widget_weight as i32) <= self.weight_width_rem {
                        set_orient_rect_width(&mut widget_area_bounds, widget_area_width + (widget_weight as i32), orient);
                    } else if rem_count < self.weight_width_rem {
                        set_orient_rect_width(&mut widget_area_bounds, widget_area_width + self.weight_width_rem - rem_count, orient);
                    }
                    widget.update_pos(cairo_context, theme, widget_area_bounds)?;
                    let x = orient_pos_x(tmp_pos, orient);
                    set_orient_pos_x(&mut tmp_pos, x + orient_rect_width(widget_area_bounds, orient), orient);
                    rem_count += widget.weight() as i32;
                } else {
                    set_orient_rect_x(&mut widget_area_bounds, orient_pos_x(tmp_pos, orient), orient);
                    set_orient_rect_width(&mut widget_area_bounds, orient_size_width(widget.margin_size(), orient), orient);
                    widget.update_pos(cairo_context, theme, widget_area_bounds)?;
                    let x = orient_pos_x(tmp_pos, orient);
                    set_orient_pos_x(&mut tmp_pos, x + orient_rect_width(widget_area_bounds, orient), orient);
                }
                weight_idx = tmp_weight_idx;
            }
            let y = orient_pos_y(pos, orient);
            set_orient_pos_y(&mut pos, y + orient_rect_height(widget_area_bounds, orient), orient);
            row_rem_count += 1;
        }
        Ok(())
    }
    
    pub fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    {
        for row in &self.widgets {
            for widget in row {
                widget.draw(cairo_context, theme, is_focused_window)?;
            }
        }
        Ok(())
    }

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
        let height_sum = self.row_height * (self.widgets.len() as i32) + self.row_height_rem;
        let height = match orient {
            Orient::Horizontal => {
                let area_height2 = min_opt_height_for_opt_height(area_size.height, preferred_size.height);
                if preferred_size.width.is_none() {
                    height_for_v_align(height_sum, area_height2, v_align)
                } else {
                    area_height2.unwrap_or(height_sum)
                }
            },
            Orient::Vertical => {
                let area_width2 = min_opt_width_for_opt_width(area_size.width, preferred_size.width);
                if preferred_size.height.is_none() {
                    width_for_h_align(height_sum, area_width2, h_align)
                } else {
                    area_width2.unwrap_or(height_sum)
                }
            },
        };
        orient_size(width, height, orient)
    }
}
