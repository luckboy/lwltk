//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::iter::FusedIterator;
use std::ops::Add;
use std::ops::Sub;
use std::slice::Iter;
use cairo;

pub type CairoContext = cairo::Context;

pub type CairoError = cairo::Error;

#[cfg(target_pointer_width = "16")]
pub type ClientInt = i64;
#[cfg(target_pointer_width = "32")]
pub type ClientInt = i64;
#[cfg(target_pointer_width = "64")]
pub type ClientInt = i128;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WidgetState
{
    None,
    Hover,
    Active,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HAlign
{
    Left,
    Center,
    Right,
    Fill,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VAlign
{
    Top,
    Center,
    Bottom,
    Fill,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TextAlign
{
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Orient
{
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color
{
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl Color
{
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self
    { Color { red, green, blue, alpha, } }

    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self
    { Self::new(red, green, blue, 1.0) }

    pub fn new_from_argb_u32(argb: u32) -> Self
    {
        let red = (((argb >> 16) & 0xff) as f64) / 256.0;
        let green = (((argb >> 8) & 0xff) as f64) / 256.0;
        let blue = ((argb & 0xff) as f64) / 256.0;
        let alpha = (((argb >> 24) & 0xff) as f64) / 256.0;
        Self::new(red, green, blue, alpha)
    }

    pub fn new_from_rgb_u32(rgb: u32) -> Self
    {
        let red = (((rgb >> 16) & 0xff) as f64) / 256.0;
        let green = (((rgb >> 8) & 0xff) as f64) / 256.0;
        let blue = ((rgb & 0xff) as f64) / 256.0;
        Self::new(red, green, blue, 1.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Pos<T>
{
    pub x: T,
    pub y: T,
}

impl<T> Pos<T>
{
    pub fn new(x: T, y: T) -> Self
    { Pos { x, y, } }
}

impl Pos<i32>
{
    pub fn to_f64_pos(&self) -> Pos<f64>
    { Pos { x: self.x as f64, y: self.y as f64, } }
}

impl Pos<f64>
{
    pub fn to_i32_pos(&self) -> Pos<i32>
    { Pos { x: self.x as i32, y: self.y as i32, } }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Size<T>
{
    pub width: T,
    pub height: T,
}

impl<T> Size<T>
{
    pub fn new(width: T, height: T) -> Self
    { Size { width, height, } }
}

impl Size<i32>
{
    pub fn to_f64_size(&self) -> Size<f64>
    { Size { width: self.width as f64, height: self.height as f64, } }
}

impl Size<f64>
{
    pub fn to_i32_size(&self) -> Size<i32>
    { Size { width: self.width as i32, height: self.height as i32, } }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Rect<T>
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self
    { Rect { x, y, width, height, } }
}

impl<T: Copy + PartialOrd + Add<Output = T>> Rect<T>
{
    pub fn contains(&self, point: Pos<T>) -> bool
    {
        point.x >= self.x && point.y >= self.y &&
        point.x < self.x + self.width && point.y < self.y + self.height
    }
}

impl<T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T>> Rect<T>
{
    pub fn intersection(&self, rect: Rect<T>) -> Option<Rect<T>>
    {
        let x1 = if self.x > rect.x { self.x } else { rect.x };
        let y1 = if self.y > rect.y { self.y } else { rect.y };
        let x2 = if self.x + self.width < rect.x + rect.width { self.x + self.width } else { rect.x + rect.width };
        let y2 = if self.y + self.height < rect.y + rect.height { self.y + self.height } else { rect.y + rect.height };
        if x1 < x2 && y1 < y2 {
            Some(Rect::new(x1, y1, x2 - x1, y2 - y1))
        } else {
            None
        }
    }
}

impl<T: Copy> Rect<T>
{
    pub fn pos(&self) -> Pos<T>
    { Pos::new(self.x, self.y) }

    pub fn size(&self) -> Size<T>
    { Size::new(self.width, self.height) }
    
    pub fn set_pos(&mut self, pos: Pos<T>)
    {
        self.x = pos.x;
        self.y = pos.y;
    }

    pub fn set_size(&mut self, size: Size<T>)
    {
        self.width = size.width;
        self.height = size.height;
    }
}

impl Rect<i32>
{
    pub fn to_f64_rect(&self) -> Rect<f64>
    {
        Rect {
            x: self.x as f64,
            y: self.y as f64,
            width: self.width as f64,
            height: self.height as f64,
        }
    }
}

impl Rect<f64>
{
    pub fn to_i32_rect(&self) -> Rect<i32>
    {
        Rect {
            x: self.x as i32,
            y: self.y as i32,
            width: self.width as i32,
            height: self.height as i32,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Edges<T>
{
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
}

impl<T> Edges<T>
{
    pub fn new(top: T, bottom: T, left: T, right: T) -> Self
    { Edges { top, bottom, left, right, } }
}

impl Edges<i32>
{
    pub fn to_f64_egdes(&self) -> Edges<f64>
    {
        Edges {
            top: self.top as f64,
            bottom: self.bottom as f64,
            left: self.left as f64,
            right: self.right as f64,
        }
    }
}

impl Edges<f64>
{
    pub fn to_i32_egdes(&self) -> Edges<i32>
    {
        Edges {
            top: self.top as i32,
            bottom: self.bottom as i32,
            left: self.left as i32,
            right: self.right as i32,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Corners<T>
{
    pub top_left_width: T,
    pub top_left_height: T,
    pub top_right_width: T,
    pub top_right_height: T,
    pub bottom_left_width: T,
    pub bottom_left_height: T,
    pub bottom_right_width: T,
    pub bottom_right_height: T,
}

impl<T> Corners<T>
{
    pub fn new(top_left_width: T, top_left_height: T, top_right_width: T, top_right_height: T, bottom_left_width: T, bottom_left_height: T, bottom_right_width: T, bottom_right_height: T) -> Self
    {
        Corners {
            top_left_width,
            top_left_height,
            top_right_width,
            top_right_height,
            bottom_left_width,
            bottom_left_height,
            bottom_right_width,
            bottom_right_height,
        }
    }
}

impl Corners<i32>
{
    pub fn to_f64_corners(&self) -> Corners<f64>
    {
        Corners {
            top_left_width: self.top_left_width as f64,
            top_left_height: self.top_left_height as f64,
            top_right_width: self.top_right_width as f64,
            top_right_height: self.top_right_height as f64,
            bottom_left_width: self.bottom_left_width as f64,
            bottom_left_height: self.bottom_left_height as f64,
            bottom_right_width: self.bottom_right_width as f64,
            bottom_right_height: self.bottom_right_height as f64,
        }
    }
}

impl Corners<f64>
{
    pub fn to_i32_corners(&self) -> Corners<i32>
    {
        Corners {
            top_left_width: self.top_left_width as i32,
            top_left_height: self.top_left_height as i32,
            top_right_width: self.top_right_width as i32,
            top_right_height: self.top_right_height as i32,
            bottom_left_width: self.bottom_left_width as i32,
            bottom_left_height: self.bottom_left_height as i32,
            bottom_right_width: self.bottom_right_width as i32,
            bottom_right_height: self.bottom_right_height as i32,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WindowIndex(pub usize);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WidgetIndexPair(pub usize, pub usize);

#[derive(Clone)]
pub struct WidgetPathIter<'a>
{
    iter: Iter<'a, WidgetIndexPair>,
}

impl<'a> WidgetPathIter<'a>
{
    fn new(slice: &'a [WidgetIndexPair]) -> Self
    { WidgetPathIter { iter: slice.iter(), } }
}

impl<'a> ExactSizeIterator for WidgetPathIter<'a>
{}

impl<'a> FusedIterator for WidgetPathIter<'a>
{}

impl<'a> DoubleEndedIterator for WidgetPathIter<'a>
{
    fn next_back(&mut self) -> Option<Self::Item>
    { self.iter.next_back().map(|ip| *ip) }
}

impl<'a> Iterator for WidgetPathIter<'a>
{
    type Item = WidgetIndexPair;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.next().map(|x| *x) }
    
    fn size_hint(&self) -> (usize, Option<usize>)
    { self.iter.size_hint() }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RelWidgetPath
{
    widget_index_pairs: Vec<WidgetIndexPair>,
}

impl RelWidgetPath
{
    pub fn new(widget_idx_pair: WidgetIndexPair) -> Self
    { RelWidgetPath { widget_index_pairs: vec![widget_idx_pair], } }
    
    pub fn widget_index_pairs(&self) -> WidgetPathIter<'_>
    { WidgetPathIter::new(self.widget_index_pairs.as_slice()) }

    pub fn iter(&self) -> WidgetPathIter<'_>
    { self.widget_index_pairs() }

    pub fn push(&mut self, widget_idx_pair: WidgetIndexPair)
    { self.widget_index_pairs.push(widget_idx_pair); }
    
    pub fn pop(&mut self) -> Option<WidgetIndexPair>
    {
        if self.widget_index_pairs.len() > 1 {
            self.widget_index_pairs.pop()
        } else {
            None
        }
    }

    pub fn to_abs_widget_path(&self, window_idx: WindowIndex) -> AbsWidgetPath
    { 
        AbsWidgetPath {
            window_index: window_idx,
            rel_widget_path: self.clone(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AbsWidgetPath
{
    window_index: WindowIndex,
    rel_widget_path: RelWidgetPath,
}

impl AbsWidgetPath
{
    pub fn new(window_idx: WindowIndex, widget_idx_pair: WidgetIndexPair) -> Self
    {
        AbsWidgetPath {
            window_index: window_idx,
            rel_widget_path: RelWidgetPath::new(widget_idx_pair),
        }
    }

    pub fn window_index(&self) -> WindowIndex
    { self.window_index }

    pub fn widget_index_pairs(&self) -> WidgetPathIter<'_>
    { self.rel_widget_path.widget_index_pairs() }
    
    pub fn as_rel_widget_path(&self) -> &RelWidgetPath
    { &self.rel_widget_path }

    pub fn iter(&self) -> WidgetPathIter<'_>
    { self.widget_index_pairs() }

    pub fn push(&mut self, widget_idx_pair: WidgetIndexPair)
    { self.rel_widget_path.push(widget_idx_pair); }

    pub fn pop(&mut self) -> Option<WidgetIndexPair>
    { self.rel_widget_path.pop() }
}
