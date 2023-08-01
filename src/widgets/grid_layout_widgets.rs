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
    
    pub fn add_empty_row(&mut self) -> Option<()>
    {
        self.widgets.push(Vec::new());
        Some(())
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
                        if j < row.len() {
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
                        } else {
                            None
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
                        if j < row.len() {
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
                        } else {
                            None
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
                    max_weight_width = max(max_weight_width, widget_width / (widget_weight as i32));
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
        if is_row_height {
            match orient {
                Orient::Horizontal => {
                    match v_align {
                        VAlign::Fill => (),
                        _ => {
                            if preferred_size.height.is_none() {
                                if max_row_height <= self.row_height {
                                    self.row_height = max_row_height;
                                    self.row_height_rem = 0;
                                }
                            }
                        },
                    }
                },
                Orient::Vertical => {
                    match h_align {
                        HAlign::Fill => (),
                        _ => {
                            if preferred_size.width.is_none() {
                                if max_row_height <= self.row_height {
                                    self.row_height = max_row_height;
                                    self.row_height_rem = 0;
                                }
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
                    let max_width = match self.zero_weight_pairs.get(&weight_idx) {
                        Some(pair) => pair.max_width,
                        None => orient_size_width(widget.margin_size(), orient),
                    };
                    set_orient_rect_x(&mut widget_area_bounds, orient_pos_x(tmp_pos, orient), orient);
                    set_orient_rect_width(&mut widget_area_bounds, max_width, orient);
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

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;
    use crate::widgets::button::*;
    use crate::preferred_size::*;

    #[test]
    fn test_grid_layout_widgets_add_widgets()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        match widgets.add_dyn(Box::new(Button::new("B1"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B2"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B3"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 2), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B4"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B5"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.add_dyn(Box::new(Button::new("B6"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 2), idx_pair),
            None => assert!(false),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(3, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(3, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B6"), dyn_widget_as_widget(&*widgets.widgets[1][2]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_grid_layout_widgets_add_empty_row()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.add_empty_row() {
            Some(()) => assert!(true),
            None => assert!(false),
        }
        assert_eq!(3, widgets.widgets.len());
        assert_eq!(3, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(3, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B6"), dyn_widget_as_widget(&*widgets.widgets[1][2]).map(|b: &Button| b.text()));
        assert_eq!(0, widgets.widgets[2].len());
    }    
    
    #[test]
    fn test_grid_layout_widgets_insert_widgets()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.insert_dyn(WidgetIndexPair(0, 1), Box::new(Button::new("B7"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.insert_dyn(WidgetIndexPair(1, 3), Box::new(Button::new("B8"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 3), idx_pair),
            None => assert!(false),
        }
        match widgets.insert_dyn(WidgetIndexPair(2, 0), Box::new(Button::new("B9"))) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(2, 0), idx_pair),
            None => assert!(false),
        }
        assert_eq!(3, widgets.widgets.len());
        assert_eq!(4, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B7"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][3]).map(|b: &Button| b.text()));
        assert_eq!(4, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B6"), dyn_widget_as_widget(&*widgets.widgets[1][2]).map(|b: &Button| b.text()));
        assert_eq!(Some("B8"), dyn_widget_as_widget(&*widgets.widgets[1][3]).map(|b: &Button| b.text()));
        assert_eq!(1, widgets.widgets[2].len());
        assert_eq!(Some("B9"), dyn_widget_as_widget(&*widgets.widgets[2][0]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_grid_layout_widgets_do_not_widgets_for_too_large_index()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.insert_dyn(WidgetIndexPair(0, 4), Box::new(Button::new("B7"))) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match widgets.insert_dyn(WidgetIndexPair(3, 0), Box::new(Button::new("B8"))) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(3, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(3, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B6"), dyn_widget_as_widget(&*widgets.widgets[1][2]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_grid_layout_widgets_remove_widget()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.remove(WidgetIndexPair(1, 1)) {
            Some(widget) => assert_eq!(Some("B5"), dyn_widget_as_widget(&*widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(3, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(2, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B6"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_grid_layout_widgets_do_not_remove_widgets_for_too_large_index()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.remove(WidgetIndexPair(1, 3)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match widgets.remove(WidgetIndexPair(2, 0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(3, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(3, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B6"), dyn_widget_as_widget(&*widgets.widgets[1][2]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_grid_layout_widgets_remove_last_widget()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.remove_last() {
            Some(widget) => assert_eq!(Some("B6"), dyn_widget_as_widget(&*widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
        assert_eq!(2, widgets.widgets.len());
        assert_eq!(3, widgets.widgets[0].len());
        assert_eq!(Some("B1"), dyn_widget_as_widget(&*widgets.widgets[0][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B2"), dyn_widget_as_widget(&*widgets.widgets[0][1]).map(|b: &Button| b.text()));
        assert_eq!(Some("B3"), dyn_widget_as_widget(&*widgets.widgets[0][2]).map(|b: &Button| b.text()));
        assert_eq!(2, widgets.widgets[1].len());
        assert_eq!(Some("B4"), dyn_widget_as_widget(&*widgets.widgets[1][0]).map(|b: &Button| b.text()));
        assert_eq!(Some("B5"), dyn_widget_as_widget(&*widgets.widgets[1][1]).map(|b: &Button| b.text()));
    }

    #[test]
    fn test_grid_layout_widgets_do_not_remove_last_widget_for_no_widgets()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        match widgets.remove_last() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        assert_eq!(0, widgets.widgets.len());
    }

    #[test]
    fn test_grid_layout_widgets_give_previous_widget_index_pairs()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 1)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 2)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 1)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);        
    }

    #[test]
    fn test_grid_layout_widgets_give_previous_widget_index_pairs_for_empty_rows()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_empty_row();
        widgets.add_empty_row();
        widgets.add_empty_row();
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(3, 1)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(3, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 2)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 1)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);        
    }    

    #[test]
    fn test_grid_layout_widgets_give_previous_widget_index_pairs_for_last_empty_rows()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_empty_row();
        widgets.add_empty_row();
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 1)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 2)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 1)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.prev(idx_pair);
        assert_eq!(None, idx_pair);        
    }    
    
    #[test]
    fn test_grid_layout_widgets_do_not_give_previous_widget_index_pairs_for_bad_index()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let idx_pair = widgets.prev(Some(WidgetIndexPair(1, 3)));
        assert_eq!(None, idx_pair);
        let idx_pair2 = widgets.prev(Some(WidgetIndexPair(3, 0)));
        assert_eq!(None, idx_pair2);
    }

    #[test]
    fn test_grid_layout_widgets_give_next_widget_index_pairs()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 1)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 2)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(1, 1)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);        
    }

    #[test]
    fn test_grid_layout_widgets_give_next_widget_index_pairs_for_empty_rows()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_empty_row();
        widgets.add_empty_row();
        widgets.add_empty_row();
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 1)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(0, 2)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(3, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(3, 1)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);        
    }    

    #[test]
    fn test_grid_layout_widgets_give_next_widget_index_pairs_for_first_empty_rows()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_empty_row();
        widgets.add_empty_row();
        widgets.add_empty_row();
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let mut idx_pair: Option<WidgetIndexPair> = None;
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(2, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(2, 1)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(2, 2)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(3, 0)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(Some(WidgetIndexPair(3, 1)), idx_pair);
        idx_pair = widgets.next(idx_pair);
        assert_eq!(None, idx_pair);        
    }    
    
    #[test]
    fn test_grid_layout_widgets_do_not_give_next_widget_index_pairs_for_bad_index()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        let idx_pair = widgets.next(Some(WidgetIndexPair(1, 3)));
        assert_eq!(None, idx_pair);
        let idx_pair2 = widgets.next(Some(WidgetIndexPair(3, 0)));
        assert_eq!(None, idx_pair2);
    }

    #[test]
    fn test_grid_layout_widgets_give_widget()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.dyn_widget(WidgetIndexPair(1, 1)) {
            Some(widget) => assert_eq!(Some("B5"), dyn_widget_as_widget(widget).map(|b: &Button| b.text())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_grid_layout_widgets_do_not_give_widgets_for_bad_index()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.dyn_widget(WidgetIndexPair(1, 3)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match widgets.dyn_widget(WidgetIndexPair(2, 0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_grid_layout_widgets_give_mutable_widget()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.dyn_widget_mut(WidgetIndexPair(1, 1)) {
            Some(widget) => assert_eq!(Some("B5"), dyn_widget_mut_as_widget_mut(widget).map(|b: &mut Button| b.text())),
            None => assert!(false),
        }
    }

    #[test]
    fn test_grid_layout_widgets_do_not_give_mutable_widgets_for_bad_index()
    {
        let mut widgets = GridLayoutWidgets::new(3);
        widgets.add_dyn(Box::new(Button::new("B1")));
        widgets.add_dyn(Box::new(Button::new("B2")));
        widgets.add_dyn(Box::new(Button::new("B3")));
        widgets.add_dyn(Box::new(Button::new("B4")));
        widgets.add_dyn(Box::new(Button::new("B5")));
        widgets.add_dyn(Box::new(Button::new("B6")));
        match widgets.dyn_widget_mut(WidgetIndexPair(1, 3)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match widgets.dyn_widget_mut(WidgetIndexPair(2, 0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
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
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_v_align(VAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 54), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 50), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 54), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 50), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 54), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 50), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 54), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 50), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
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
        let expected_weight_width = 57;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 57, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 57, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 114, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 114 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_h_align(HAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_h_align(HAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
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
        let expected_weight_width = 57;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(57, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(53, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(171, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(167, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(57, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(53, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(171, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(167, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 57, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 57, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 114, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 114 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_with_unweighted_widget_and_row_with_one_widget()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let mut button9 = Button::new("B9");
        button9.set_weight(1);
        button9.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button9));
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
        let expected_weight_width = 57;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[1][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][0].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 57, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 57, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 57, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
        assert_eq!(Pos::new(20, 10 + 34 * 2), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 * 2 + 2), widgets.widgets[2][0].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_with_filled_unweighted_widget_and_row_with_one_widget()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_h_align(HAlign::Fill);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_h_align(HAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let mut button9 = Button::new("B9");
        button9.set_h_align(HAlign::Fill);
        button9.set_weight(1);
        button9.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button9));
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
        let expected_weight_width = 57;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(57, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(53, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(171, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(167, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(57, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(53, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(57, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(53, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(171, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(167, 30), widgets.widgets[1][3].size());
        assert_eq!(Size::new(57, 34), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(53, 30), widgets.widgets[2][0].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 57, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 57, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 57, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
        assert_eq!(Pos::new(20, 10 + 34 * 2), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 * 2 + 2), widgets.widgets[2][0].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 57;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 57, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 171 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 57, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 57 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 114, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 57 + 54 + 114 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_h_align(HAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_h_align(HAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(181, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(177, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(120, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(116, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(121, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(117, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(180, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(176, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_area_width_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_h_align(HAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_h_align(HAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(181, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(177, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(120, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(116, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(121, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(117, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(180, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(176, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 4;
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
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 183, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 183 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 122, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 122 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_area_width_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_h_align(HAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_h_align(HAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 4;
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
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(183, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(179, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(120, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(116, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(122, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(118, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(181, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(177, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 183, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 183 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 122, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 122 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_weighted_widgets_and_area_width_and_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 70 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(Some(preferred_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(114, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(110, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(141, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(137, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_weighted_widgets_area_width_and_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_h_align(HAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_h_align(HAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 50 + 4 + 60 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(Some(preferred_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 60;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 34;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[0][1].size());
        assert_eq!(Size::new(181, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(177, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(120, 34), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(116, 30), widgets.widgets[0][3].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(121, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(117, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(180, 34), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(176, 30), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181, 10), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 181 + 2, 10 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20, 10 + 34), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 34 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 34), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 34 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 54, 10 + 34), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 2, 10 + 34 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121, 10 + 34), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 54 + 121 + 2, 10 + 34 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_height = 50 * 3 + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20, 10 + 44 + 44), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 44 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 44 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 44 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_widgets_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_v_align(VAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_v_align(VAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_height = 50 * 3 + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 50;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 51), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 47), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 50), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 46), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 50), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 46), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20, 10 + 51 + 51), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51 + 51), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_area_height_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_height = 50 * 3 + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 50;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20, 10 + 51 + 51), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51 + 51), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_widgets_and_area_height_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_v_align(VAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_v_align(VAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_height = 50 * 3 + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 50;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 51), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 47), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 50), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 46), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 50), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 46), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20, 10 + 51 + 51), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51 + 51), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_area_height_and_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_height = 60 * 3 + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_height = 50 * 3 + 2;
        let preferred_size = Size::new(None, Some(preferred_height));
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 50;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20, 10 + 51 + 51), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51 + 51), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_and_filled_widgets_and_area_height_and_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_v_align(VAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_v_align(VAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_v_align(VAlign::Fill);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_v_align(VAlign::Fill);
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_v_align(VAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_height = 60 * 3 + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_height = 50 * 3 + 2;
        let preferred_size = Size::new(None, Some(preferred_height));
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 50;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 51), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 47), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 50), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 46), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 50), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 46), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20, 10 + 51 + 51), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 51 + 51), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 51 + 51 + 2), widgets.widgets[2][1].pos());
    }
    
    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_horizontal_orientation_for_no_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
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
        let expected_row_height = 0;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
    }
    
    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 40 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 64;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 54 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 44 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
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
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_h_align(HAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 50 + 4 + 40 + 4 + 50 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 64;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(64, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(60, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(64, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(60, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(64, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(60, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(64, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(60, 50), widgets.widgets[1][2].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 54 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 44 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
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
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 47), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44 + 141), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 141 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44 + 94), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 94 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_v_align(VAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_v_align(VAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
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
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 47), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 43), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 141), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 137), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 47), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 43), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 141), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 137), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 47), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44 + 141), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 141 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44 + 94), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 94 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_with_unweighted_widget_and_row_with_one_widget()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let mut button9 = Button::new("B9");
        button9.set_weight(1);
        button9.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button9));
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
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[1][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][0].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 47), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44 + 141), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 141 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44 + 47), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 47 + 2), widgets.widgets[1][3].pos());
        assert_eq!(Pos::new(20 + 44 + 44, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 44 + 2, 10 + 2), widgets.widgets[2][0].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_with_unweighted_widget_and_row_with_one_widget()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_v_align(VAlign::Fill);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_v_align(VAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let mut button9 = Button::new("B9");
        button9.set_v_align(VAlign::Fill);
        button9.set_weight(1);
        button9.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button9));
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
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 47), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 43), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 141), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 137), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 47), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 43), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 47), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 43), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 141), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 137), widgets.widgets[1][3].size());
        assert_eq!(Size::new(44, 47), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(40, 43), widgets.widgets[2][0].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 47), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44 + 141), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 141 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44 + 47), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 47 + 2), widgets.widgets[1][3].pos());
        assert_eq!(Pos::new(20 + 44 + 44, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 44 + 2, 10 + 2), widgets.widgets[2][0].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 47;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 47), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 47 + 44 + 141), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 47 + 44 + 141 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 47 + 44 + 94), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 47 + 44 + 94 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_v_align(VAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_v_align(VAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 151), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 147), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 100), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 96), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 101), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 97), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 150), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 146), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 151), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 151 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 101), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 101 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 151), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 151 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 101), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 101 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_v_align(VAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_v_align(VAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
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
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 151), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 147), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 100), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 96), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 101), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 97), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 150), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 146), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 151), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 151 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 101), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 101 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 4;
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
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 153), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 153 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 102), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 102 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height_and_fill_alignment_and_second_remainder()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_v_align(VAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_v_align(VAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 50 * (1 + 3 + 2) + 4;
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
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 4;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 153), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 149), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 100), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 96), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 102), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 98), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 151), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 147), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 153), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 153 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 102), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 102 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_weighted_widgets_and_area_height_and_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 60 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(None, Some(preferred_height));
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 94), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 90), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 111), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 107), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 151), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 151 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 101), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 101 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_weighted_widgets_and_area_height_and_preferred_height()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_v_align(VAlign::Fill);
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_v_align(VAlign::Fill);
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_v_align(VAlign::Fill);
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_height = 40 + 4 + 60 * (1 + 3 + 2) + 2;
        let area_size = Size::new(None, Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_height = 40 + 4 + 50 * (1 + 3 + 2) + 2;
        let preferred_size = Size::new(None, Some(preferred_height));
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 6;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 50;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 2;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 44;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 51), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[0][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 151), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(40, 147), widgets.widgets[0][2].size());
        assert_eq!(Size::new(44, 100), widgets.widgets[0][3].margin_size());
        assert_eq!(Size::new(40, 96), widgets.widgets[0][3].size());
        assert_eq!(Size::new(44, 51), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 47), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 101), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 97), widgets.widgets[1][2].size());
        assert_eq!(Size::new(44, 150), widgets.widgets[1][3].margin_size());
        assert_eq!(Size::new(40, 146), widgets.widgets[1][3].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 51), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 , 10 + 51 + 44 + 151), widgets.widgets[0][3].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 51 + 44 + 151 + 2), widgets.widgets[0][3].pos());
        assert_eq!(Pos::new(20 + 44, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 2), widgets.widgets[1][2].pos());
        assert_eq!(Pos::new(20 + 44, 10 + 51 + 44 + 101), widgets.widgets[1][3].margin_pos());
        assert_eq!(Pos::new(20 + 44 + 2, 10 + 51 + 44 + 101 + 2), widgets.widgets[1][3].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 * 3 + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 54, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 54, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 54 + 54, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 54 + 2, 10 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 54 + 54, 10 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 54 + 54 + 2, 10 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_widgets_and_area_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_h_align(HAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_h_align(HAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 * 3 + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 60;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(61, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(57, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(60, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(56, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(60, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(56, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_area_width_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 * 3 + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 60;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_widgets_and_area_width_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_h_align(HAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_h_align(HAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 * 3 + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 60;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(61, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(57, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(60, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(56, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(60, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(56, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_area_width_and_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 70 * 3 + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 60 * 3 + 2;
        let preferred_size = Size::new(Some(preferred_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 60;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(44, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(40, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_and_filled_widgets_and_area_width_and_preferred_width()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(2);
        let mut button1 = Button::new("B1");
        button1.set_h_align(HAlign::Fill);
        button1.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_h_align(HAlign::Fill);
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_h_align(HAlign::Fill);
        button3.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_h_align(HAlign::Fill);
        button4.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_h_align(HAlign::Fill);
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_h_align(HAlign::Fill);
        button6.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 70 * 3 + 2;
        let area_size = Size::new(Some(area_width), None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 60 * 3 + 2;
        let preferred_size = Size::new(Some(preferred_width), None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 40 + 4 + 40 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 60;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 2;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(61, 34), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[0][0].size());
        assert_eq!(Size::new(61, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(57, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(61, 34), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(57, 30), widgets.widgets[1][1].size());
        assert_eq!(Size::new(60, 44), widgets.widgets[2][0].margin_size());
        assert_eq!(Size::new(56, 40), widgets.widgets[2][0].size());
        assert_eq!(Size::new(60, 34), widgets.widgets[2][1].margin_size());
        assert_eq!(Size::new(56, 30), widgets.widgets[2][1].size());
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, area_width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20, 10 + 44), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 44 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 61, 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 61, 10 + 44), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 2, 10 + 44 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10), widgets.widgets[2][0].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 2), widgets.widgets[2][0].pos());
        assert_eq!(Pos::new(20 + 61 + 61, 10 + 44), widgets.widgets[2][1].margin_pos());
        assert_eq!(Pos::new(20 + 61 + 61 + 2, 10 + 44 + 2), widgets.widgets[2][1].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_vertical_orientation_for_no_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
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
        let expected_row_height = 0;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let area_bounds = Rect::new(20, 10, size.width, size.height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_left_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_center_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Center;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20 + 5, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2 + 5, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64 + 5, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2 + 5, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 5, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2 + 5, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 5, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2 + 5, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64 + 5, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2 + 5, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 5, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2 + 5, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_right_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Right;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20 + 10, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2 + 10, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64 + 10, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2 + 10, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 10, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2 + 10, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20 + 10, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2 + 10, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64 + 10, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2 + 10, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 10, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2 + 10, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_fill_horizontal_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_top_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_center_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Center;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10 + 5;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10 + 5), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2 + 5), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 5), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2 + 5), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 5), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2 + 5), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54 + 5), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2 + 5), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54 + 5), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2 + 5), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54 + 5), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2 + 5), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_bottom_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Bottom;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 54;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10 + 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10 + 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2 + 10), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2 + 10), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2 + 10), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 54 + 10), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 54 + 2 + 10), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 54 + 10), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 54 + 2 + 10), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 54 + 10), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 54 + 2 + 10), widgets.widgets[1][2].pos());
    }

    #[test]
    fn test_grid_layout_widgets_update_size_and_position_for_fill_vertical_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 64 + 54 + 64 + 10;
        let area_height = 54 * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        match widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_zero_weight_width_sum = 60 + 4 + 50 + 4 + 60 + 4;
        assert_eq!(expected_zero_weight_width_sum, widgets.zero_weight_width_sum);
        let expected_weight_sum = 0;
        assert_eq!(expected_weight_sum, widgets.weight_sum);
        let expected_weight_width = 0;
        assert_eq!(expected_weight_width, widgets.weight_width);
        let expected_weight_width_rem = 0;
        assert_eq!(expected_weight_width_rem, widgets.weight_width_rem);
        let expected_row_height = 59;
        assert_eq!(expected_row_height, widgets.row_height);
        let expected_row_height_rem = 0;
        assert_eq!(expected_row_height_rem, widgets.row_height_rem);
        assert_eq!(Size::new(44, 54), widgets.widgets[0][0].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[0][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[0][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[0][1].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[0][2].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[0][2].size());
        assert_eq!(Size::new(64, 34), widgets.widgets[1][0].margin_size());
        assert_eq!(Size::new(60, 30), widgets.widgets[1][0].size());
        assert_eq!(Size::new(54, 44), widgets.widgets[1][1].margin_size());
        assert_eq!(Size::new(50, 40), widgets.widgets[1][1].size());
        assert_eq!(Size::new(44, 54), widgets.widgets[1][2].margin_size());
        assert_eq!(Size::new(40, 50), widgets.widgets[1][2].size());
        let area_bounds = Rect::new(20, 10, area_width, area_height);
        match widgets.update_pos(&cairo_context, &theme, area_bounds, orient, h_align, v_align, preferred_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_start_y = 10;
        assert_eq!(expected_start_y, widgets.start_y);
        assert_eq!(Pos::new(20, 10), widgets.widgets[0][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 2), widgets.widgets[0][0].pos());
        assert_eq!(Pos::new(20 + 64, 10), widgets.widgets[0][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 2), widgets.widgets[0][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10), widgets.widgets[0][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 2), widgets.widgets[0][2].pos());
        assert_eq!(Pos::new(20, 10 + 59), widgets.widgets[1][0].margin_pos());
        assert_eq!(Pos::new(20 + 2, 10 + 59 + 2), widgets.widgets[1][0].pos());
        assert_eq!(Pos::new(20 + 64, 10 + 59), widgets.widgets[1][1].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 2, 10 + 59 + 2), widgets.widgets[1][1].pos());
        assert_eq!(Pos::new(20 + 64 + 54, 10 + 59), widgets.widgets[1][2].margin_pos());
        assert_eq!(Pos::new(20 + 64 + 54 + 2, 10 + 59 + 2), widgets.widgets[1][2].pos());
    }
    
    #[test]
    fn test_grid_layout_widgets_give_size_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 50 + 4 + 60 + 4;
        let expected_height = (50 + 4) * 2;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }    
    
    #[test]
    fn test_grid_layout_widgets_give_size_for_horizontal_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(110), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(50), Some(30)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(137), Some(30)));
        widgets.add_dyn(Box::new(button8));
        let area_size = Size::new(None, None);
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 50 + 4 + 57 + 171 + 114;
        let expected_height = (30 + 4) * 2;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_horizontal_orientation_and_area_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let area_height = (50 + 4) * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 50 + 4 + 60 + 4;
        let expected_height = (50 + 4) * 2;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_horizontal_orientation_and_area_size_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let area_height = (50 + 4) * 2 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let expected_height = (50 + 4) * 2 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_horizontal_orientation_and_area_size_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 + 4 + 50 + 4 + 60 + 4 + 20;
        let area_height = (50 + 4) * 2 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let preferred_height = (50 + 4) * 2 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let expected_height = (50 + 4) * 2 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_horizontal_orientation_and_area_size_and_fill_alignment_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = 60 + 4 + 50 + 4 + 60 + 4 + 20;
        let area_height = (50 + 4) * 2 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Horizontal;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let preferred_height = (50 + 4) * 2 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = 60 + 4 + 50 + 4 + 60 + 4 + 10;
        let expected_height = (50 + 4) * 2 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = (60 + 4) * 2;
        let expected_height = 50 + 4 + 40 + 4 + 50 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }
    
    #[test]
    fn test_grid_layout_widgets_give_size_for_vertical_orientation_and_weighted_widgets()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(4);
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
        button4.set_preferred_size(Size::new(Some(40), Some(90)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_weight(1);
        button5.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(40)));
        widgets.add_dyn(Box::new(button6));
        let mut button7 = Button::new("B7");
        button7.set_weight(2);
        button7.set_preferred_size(Size::new(Some(40), Some(30)));
        widgets.add_dyn(Box::new(button7));
        let mut button8 = Button::new("B8");
        button8.set_weight(3);
        button8.set_preferred_size(Size::new(Some(40), Some(107)));
        widgets.add_dyn(Box::new(button8));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = (40 + 4) * 2;
        let expected_height = 40 + 4 + 47 + 141 + 94;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_vertical_orientation_and_area_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = (60 + 4) * 2 + 10;
        let area_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = (60 + 4) * 2;
        let expected_height = 50 + 4 + 40 + 4 + 50 + 4;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_vertical_orientation_and_area_size_and_fill_alignment()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = (60 + 4) * 2 + 10;
        let area_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_size = Size::new(None, None);
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = (60 + 4) * 2 + 10;
        let expected_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_vertical_orientation_and_area_size_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = (60 + 4) * 2 + 20;
        let area_height = 50 + 4 + 40 + 4 + 50 + 4 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let preferred_width = (60 + 4) * 2 + 10;
        let preferred_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = (60 + 4) * 2 + 10;
        let expected_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }

    #[test]
    fn test_grid_layout_widgets_give_size_for_vertical_orientation_and_area_size_and_fill_alignment_and_preferred_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_width = (60 + 4) * 2 + 20;
        let area_height = 50 + 4 + 40 + 4 + 50 + 4 + 20;
        let area_size = Size::new(Some(area_width), Some(area_height));
        let orient = Orient::Vertical;
        let h_align = HAlign::Fill;
        let v_align = VAlign::Fill;
        let preferred_width = (60 + 4) * 2 + 10;
        let preferred_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        let preferred_size = Size::new(Some(preferred_width), Some(preferred_height));
        widgets.update_size(&cairo_context, &theme, area_size, orient, h_align, v_align, preferred_size).unwrap();
        let size = widgets.size(area_size, orient, h_align, v_align, preferred_size);
        let expected_width = (60 + 4) * 2 + 10;
        let expected_height = 50 + 4 + 40 + 4 + 50 + 4 + 10;
        assert_eq!(Size::new(expected_width, expected_height), size);
    }
    
    #[test]
    fn test_grid_layout_widgets_point_widgets_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
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
        match widgets.point(Pos::new(90.0, 45.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(140.0, 12.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 2), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(25.0, 70.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(86.0, 68.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(160.0, 80.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 2), idx_pair),
            None => assert!(false),
        }
    }

    #[test]
    fn test_grid_layout_widgets_do_not_point_widgets_for_horizontal_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
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
        match widgets.point(Pos::new(30.0, 64.0), orient) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_grid_layout_widgets_point_widgets_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
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
        match widgets.point(Pos::new(50.0, 95.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(22.0, 130.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(0, 2), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(90.0, 15.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 0), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(86.0, 76.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 1), idx_pair),
            None => assert!(false),
        }
        match widgets.point(Pos::new(100.0, 140.0), orient) {
            Some(idx_pair) => assert_eq!(WidgetIndexPair(1, 2), idx_pair),
            None => assert!(false),
        }
    }

    #[test]
    fn test_grid_layout_widgets_do_not_point_widgets_for_vertical_orientation()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut widgets = GridLayoutWidgets::new(3);
        let mut button1 = Button::new("B1");
        button1.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button1));
        let mut button2 = Button::new("B2");
        button2.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button2));
        let mut button3 = Button::new("B3");
        button3.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button3));
        let mut button4 = Button::new("B4");
        button4.set_preferred_size(Size::new(Some(60), Some(30)));
        widgets.add_dyn(Box::new(button4));
        let mut button5 = Button::new("B5");
        button5.set_preferred_size(Size::new(Some(50), Some(40)));
        widgets.add_dyn(Box::new(button5));
        let mut button6 = Button::new("B6");
        button6.set_preferred_size(Size::new(Some(40), Some(50)));
        widgets.add_dyn(Box::new(button6));
        let area_size = Size::new(None, None);
        let orient = Orient::Vertical;
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
        match widgets.point(Pos::new(84.0, 20.0), orient) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
