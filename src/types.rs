//
// Copyright (c) 2022 Łukasz Szpakowski
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum WidgetState
{
    None,
    Hover,
    Active,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum HAlign
{
    Left,
    Center,
    Right,
    Fill,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VAlign
{
    Top,
    Center,
    Bottom,
    Fill,
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
        let x2 = if self.x + self.width < rect.x + self.width { self.x + self.width } else { rect.x + rect.width };
        let y2 = if self.y + self.height < rect.y + self.height { self.y + self.height } else { rect.y + rect.height };
        if x1 <= x2 && y1 <= y2 {
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
