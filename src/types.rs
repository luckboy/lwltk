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

/// A type of cairo context.
pub type CairoContext = cairo::Context;

/// A type of cairo error.
pub type CairoError = cairo::Error;

/// An integer type for a widget client.
#[cfg(target_pointer_width = "16")]
pub type ClientInt = i64;
/// An integer type for a widget client.
#[cfg(target_pointer_width = "32")]
pub type ClientInt = i64;
/// An integer type for a widget client.
#[cfg(target_pointer_width = "64")]
pub type ClientInt = i128;

/// An enumeration of widget state.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WidgetState
{
    /// A none.
    None,
    /// A hover.
    Hover,
    /// A active.
    Active,
}

/// An enumeration of horizontal alignment.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HAlign
{
    /// An alignment to left.
    Left,
    /// An alignment to center.
    Center,
    /// An alignment to right.
    Right,
    /// An alignment to fill.
    Fill,
}

/// An enumeration of vertical alignment.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VAlign
{
    /// An alignment to top.
    Top,
    /// An alignment to center.
    Center,
    /// An alignment to bottom.
    Bottom,
    /// An alignment to fill.
    Fill,
}

/// An enumeration of text alignment.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TextAlign
{
    /// An alignment to left.
    Left,
    /// An alignment to center.
    Center,
    /// An alignment to right.
    Right,
}

/// An orientation enumeration.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Orient
{
    /// A horizontal orientation.
    Horizontal,
    /// A vertical orientation.
    Vertical,
}

/// A color structure.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color
{
    /// A field of red component.
    pub red: f64,
    /// A field of green component.
    pub green: f64,
    /// A field of blue component.
    pub blue: f64,
    /// A field of alpha component.
    pub alpha: f64,
}

impl Color
{
    /// Creates a color for the red component, the green component, the blue component, and the
    /// alpha component.
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self
    { Color { red, green, blue, alpha, } }

    /// Creates a color for the red component, the green component, and the blue component.
    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self
    { Self::new(red, green, blue, 1.0) }

    /// Creates a color from 32-bit unsigned integer number for the alpha component, the red
    /// component, the green component, and the blue component.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Color;
    ///
    /// let color = Color::new_from_argb_u32(0x80402010);
    /// assert_eq!(0.25, color.red);
    /// assert_eq!(0.125, color.green);
    /// assert_eq!(0.0625, color.blue);
    /// assert_eq!(0.50, color.alpha);
    /// ```
    pub fn new_from_argb_u32(argb: u32) -> Self
    {
        let red = (((argb >> 16) & 0xff) as f64) / 256.0;
        let green = (((argb >> 8) & 0xff) as f64) / 256.0;
        let blue = ((argb & 0xff) as f64) / 256.0;
        let alpha = (((argb >> 24) & 0xff) as f64) / 256.0;
        Self::new(red, green, blue, alpha)
    }

    /// Creates a color from 32-bit unsigned integer number for the red component, the green 
    /// component, and the blue component.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Color;
    ///
    /// let color = Color::new_from_rgb_u32(0x804020);
    /// assert_eq!(0.50, color.red);
    /// assert_eq!(0.25, color.green);
    /// assert_eq!(0.125, color.blue);
    /// assert_eq!(1.0, color.alpha);
    /// ```
    pub fn new_from_rgb_u32(rgb: u32) -> Self
    {
        let red = (((rgb >> 16) & 0xff) as f64) / 256.0;
        let green = (((rgb >> 8) & 0xff) as f64) / 256.0;
        let blue = ((rgb & 0xff) as f64) / 256.0;
        Self::new(red, green, blue, 1.0)
    }
}

/// A position structure.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Pos<T>
{
    /// A field of X coordinate.
    pub x: T,
    /// A field of Y coordinate.
    pub y: T,
}

impl<T> Pos<T>
{
    /// Creates a position.
    pub fn new(x: T, y: T) -> Self
    { Pos { x, y, } }
}

impl Pos<i32>
{
    /// Converts the integer position to a floating point position.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Pos;
    /// 
    /// let pos = Pos::new(1, 2);
    /// assert_eq!(Pos::new(1.0, 2.0), pos.to_f64_pos());
    /// ```
    pub fn to_f64_pos(&self) -> Pos<f64>
    { Pos { x: self.x as f64, y: self.y as f64, } }
}

impl Pos<f64>
{
    /// Converts the floating point position to an integer position.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Pos;
    /// 
    /// let pos = Pos::new(1.5, 2.0);
    /// assert_eq!(Pos::new(1, 2), pos.to_i32_pos());
    /// ```
    pub fn to_i32_pos(&self) -> Pos<i32>
    { Pos { x: self.x as i32, y: self.y as i32, } }
}

/// A size structure.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Size<T>
{
    /// A width field.
    pub width: T,
    /// A height field.
    pub height: T,
}

impl<T> Size<T>
{
    /// Creates a size.
    pub fn new(width: T, height: T) -> Self
    { Size { width, height, } }
}

impl Size<i32>
{
    /// Converts the integer size to a floating point size.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Size;
    /// 
    /// let size = Size::new(1, 2);
    /// assert_eq!(Size::new(1.0, 2.0), size.to_f64_size());
    /// ```
    pub fn to_f64_size(&self) -> Size<f64>
    { Size { width: self.width as f64, height: self.height as f64, } }
}

impl Size<f64>
{
    /// Converts the floating point size to an integer size.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Size;
    /// 
    /// let size = Size::new(1.5, 2.0);
    /// assert_eq!(Size::new(1, 2), size.to_i32_size());
    /// ```
    pub fn to_i32_size(&self) -> Size<i32>
    { Size { width: self.width as i32, height: self.height as i32, } }
}

/// A rectangle structure.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Rect<T>
{
    /// A field of X coordinate.
    pub x: T,
    /// A field of Y coordinate.
    pub y: T,
    /// A width field.
    pub width: T,
    /// A height field.
    pub height: T,
}

impl<T> Rect<T>
{
    /// Creates a rectangle.
    pub fn new(x: T, y: T, width: T, height: T) -> Self
    { Rect { x, y, width, height, } }
}

impl<T: Copy + PartialOrd + Add<Output = T>> Rect<T>
{
    /// Returns `true` if the rectangle contains the point, otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Pos;
    /// use lwltk::Rect;
    ///
    /// let rect = Rect::new(1, 2, 3, 4);
    /// assert_eq!(true, rect.contains(Pos::new(2, 3)));
    /// assert_eq!(false, rect.contains(Pos::new(4, 6)));
    /// ```
    pub fn contains(&self, point: Pos<T>) -> bool
    {
        point.x >= self.x && point.y >= self.y &&
        point.x < self.x + self.width && point.y < self.y + self.height
    }
}

impl<T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T>> Rect<T>
{
    /// Returns an intersection of two rectangles if the intersection of two rectangles isn't empty,
    /// otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Rect;
    ///
    /// let rect1 = Rect::new(1, 2, 3, 4);
    /// let rect2 = Rect::new(2, 4, 5, 6);
    /// let rect3 = Rect::new(4, 6, 3, 4);
    /// assert_eq!(Some(Rect::new(2, 4, 2, 2)), rect1.intersection(rect2));
    /// assert_eq!(None, rect1.intersection(rect3));
    /// ```
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
    /// Returns the rectangle position.
    pub fn pos(&self) -> Pos<T>
    { Pos::new(self.x, self.y) }

    /// Returns the rectangle size.
    pub fn size(&self) -> Size<T>
    { Size::new(self.width, self.height) }
    
    /// Sets the rectangle position.
    pub fn set_pos(&mut self, pos: Pos<T>)
    {
        self.x = pos.x;
        self.y = pos.y;
    }

    /// Sets the rectangle size. 
    pub fn set_size(&mut self, size: Size<T>)
    {
        self.width = size.width;
        self.height = size.height;
    }
}

impl Rect<i32>
{
    /// Converts the integer rectangle to a floating point rectangle.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Rect;
    /// 
    /// let rect = Rect::new(1, 2, 3, 4);
    /// assert_eq!(Rect::new(1.0, 2.0, 3.0, 4.0), rect.to_f64_rect());
    /// ```
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
    /// Converts the floating point rectangle to an integer rectangle.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Rect;
    /// 
    /// let rect = Rect::new(1.5, 2.0, 3.5, 4.0);
    /// assert_eq!(Rect::new(1, 2, 3, 4), rect.to_i32_rect());
    /// ```
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

/// A structure of edges.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Edges<T>
{
    /// A top field.
    pub top: T,
    /// A bottom field.
    pub bottom: T,
    /// A left field.
    pub left: T,
    /// A right field.
    pub right: T,
}

impl<T> Edges<T>
{
    /// Creates edges.
    pub fn new(top: T, bottom: T, left: T, right: T) -> Self
    { Edges { top, bottom, left, right, } }
}

impl Edges<i32>
{
    /// Converts the integer edges to floating point edges.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Edges;
    /// 
    /// let edges = Edges::new(1, 2, 3, 4);
    /// assert_eq!(Edges::new(1.0, 2.0, 3.0, 4.0), edges.to_f64_edges());
    /// ```
    pub fn to_f64_edges(&self) -> Edges<f64>
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
    /// Converts the floating point edges to integer edges.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Edges;
    /// 
    /// let edges = Edges::new(1.5, 2.0, 3.5, 4.0);
    /// assert_eq!(Edges::new(1, 2, 3, 4), edges.to_i32_edges());
    /// ```
    pub fn to_i32_edges(&self) -> Edges<i32>
    {
        Edges {
            top: self.top as i32,
            bottom: self.bottom as i32,
            left: self.left as i32,
            right: self.right as i32,
        }
    }
}

/// A structure of corners.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Corners<T>
{
    /// A field of top left width.
    pub top_left_width: T,
    /// A field of top left height.
    pub top_left_height: T,
    /// A field of top right width.
    pub top_right_width: T,
    /// A field of top right height.
    pub top_right_height: T,
    /// A field of bottom left width.
    pub bottom_left_width: T,
    /// A field of bottom left height.
    pub bottom_left_height: T,
    /// A field of bottom right width.
    pub bottom_right_width: T,
    /// A field of bottom right height.
    pub bottom_right_height: T,
}

impl<T> Corners<T>
{
    /// Creates corners.
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
    /// Converts the integer corners to floating point corners.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Corners;
    /// 
    /// let corners = Corners::new(1, 2, 3, 4, 5, 6, 7, 8);
    /// assert_eq!(Corners::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0), corners.to_f64_corners());
    /// ```
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
    /// Converts the floating point corners to integer corners.
    ///
    /// # Examples
    /// ```
    /// use lwltk::Corners;
    /// 
    /// let corners = Corners::new(1.5, 2.0, 3.5, 4.0, 5.5, 6.0, 7.5, 8.0);
    /// assert_eq!(Corners::new(1, 2, 3, 4, 5, 6, 7, 8), corners.to_i32_corners());
    /// ```
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

/// A structure of window index.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WindowIndex(pub usize);

/// A structure of pairs of widget indices.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WidgetIndexPair(pub usize, pub usize);

/// A widget path iterator that iterates over pairs of widget indices.
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

/// A structure of relative widget path.
///
/// The relative widget path refers to the widget for the container. This wigdet path doesn't
/// contain the window index.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RelWidgetPath
{
    widget_index_pairs: Vec<WidgetIndexPair>,
}

impl RelWidgetPath
{
    /// Creates a relative widget path.
    pub fn new(widget_idx_pair: WidgetIndexPair) -> Self
    { RelWidgetPath { widget_index_pairs: vec![widget_idx_pair], } }
    
    /// Returns an iterator that iterates over the pairs of widget indices.
    ///
    /// # Examples
    /// ```
    /// use lwltk::RelWidgetPath;
    /// use lwltk::WidgetIndexPair;
    ///
    /// let mut path = RelWidgetPath::new(WidgetIndexPair(1, 2));
    /// path.push(WidgetIndexPair(3, 4));
    /// path.push(WidgetIndexPair(5, 6));
    /// let mut iter = path.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), iter.next());
    /// assert_eq!(Some(WidgetIndexPair(3, 4)), iter.next());
    /// assert_eq!(Some(WidgetIndexPair(5, 6)), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn widget_index_pairs(&self) -> WidgetPathIter<'_>
    { WidgetPathIter::new(self.widget_index_pairs.as_slice()) }

    /// See [`widget_index_pairs`](Self::widget_index_pairs).
    pub fn iter(&self) -> WidgetPathIter<'_>
    { self.widget_index_pairs() }

    /// Pushes a pair of widget indices to the relative widget path.
    ///
    /// # Examples
    /// ```
    /// use lwltk::RelWidgetPath;
    /// use lwltk::WidgetIndexPair;
    ///
    /// let mut path = RelWidgetPath::new(WidgetIndexPair(1, 2));
    /// path.push(WidgetIndexPair(3, 4));
    /// let mut iter = path.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), iter.next());
    /// assert_eq!(Some(WidgetIndexPair(3, 4)), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn push(&mut self, widget_idx_pair: WidgetIndexPair)
    { self.widget_index_pairs.push(widget_idx_pair); }
    
    /// Pops the pair of widget indices from the relative widget path.
    ///
    /// # Examples
    /// ```
    /// use lwltk::RelWidgetPath;
    /// use lwltk::WidgetIndexPair;
    ///
    /// let mut path1 = RelWidgetPath::new(WidgetIndexPair(1, 2));
    /// path1.push(WidgetIndexPair(3, 4));
    /// path1.push(WidgetIndexPair(5, 6));
    /// let idx_pair1 = path1.pop();
    /// assert_eq!(Some(WidgetIndexPair(5, 6)), idx_pair1);
    /// let mut iter1 = path1.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), iter1.next());
    /// assert_eq!(Some(WidgetIndexPair(3, 4)), iter1.next());
    /// assert_eq!(None, iter1.next());
    /// let mut path2 = RelWidgetPath::new(WidgetIndexPair(2, 3));
    /// let idx_pair2 = path2.pop();
    /// assert_eq!(None, idx_pair2);
    /// let mut iter2 = path2.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(2, 3)), iter2.next());
    /// assert_eq!(None, iter2.next());
    /// ```
    pub fn pop(&mut self) -> Option<WidgetIndexPair>
    {
        if self.widget_index_pairs.len() > 1 {
            self.widget_index_pairs.pop()
        } else {
            None
        }
    }

    /// Converts the relative widget path to an absolute widget path with the specified window index.
    ///
    /// This method returns the pair of widget indices if the pair of widget indices is popped,
    /// otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::RelWidgetPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let path = RelWidgetPath::new(WidgetIndexPair(1, 2));
    /// let abs_path = path.to_abs_widget_path(WindowIndex(1));
    /// assert_eq!(WindowIndex(1), abs_path.window_index());
    /// let mut abs_iter = abs_path.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), abs_iter.next());
    /// assert_eq!(None, abs_iter.next());
    /// ```
    pub fn to_abs_widget_path(&self, window_idx: WindowIndex) -> AbsWidgetPath
    { 
        AbsWidgetPath {
            window_index: window_idx,
            rel_widget_path: self.clone(),
        }
    }
}

/// A structure of absolute widget path.
///
/// The absolute widget path refers to the widget for the window container. This wigdet path
/// contains the window index.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AbsWidgetPath
{
    window_index: WindowIndex,
    rel_widget_path: RelWidgetPath,
}

impl AbsWidgetPath
{
    /// Creates an absolute widget path.
    pub fn new(window_idx: WindowIndex, widget_idx_pair: WidgetIndexPair) -> Self
    {
        AbsWidgetPath {
            window_index: window_idx,
            rel_widget_path: RelWidgetPath::new(widget_idx_pair),
        }
    }

    /// Returns the window inxdex.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let path = AbsWidgetPath::new(WindowIndex(1), WidgetIndexPair(1, 2));
    /// assert_eq!(WindowIndex(1), path.window_index());
    /// ```
    pub fn window_index(&self) -> WindowIndex
    { self.window_index }

    /// Returns an iterator that iterates over the pairs of widget indices.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let mut path = AbsWidgetPath::new(WindowIndex(1), WidgetIndexPair(1, 2));
    /// path.push(WidgetIndexPair(3, 4));
    /// path.push(WidgetIndexPair(5, 6));
    /// let mut iter = path.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), iter.next());
    /// assert_eq!(Some(WidgetIndexPair(3, 4)), iter.next());
    /// assert_eq!(Some(WidgetIndexPair(5, 6)), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn widget_index_pairs(&self) -> WidgetPathIter<'_>
    { self.rel_widget_path.widget_index_pairs() }

    /// Returns a reference to the relative widget path for the absolute widget path.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::RelWidgetPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let path = AbsWidgetPath::new(WindowIndex(1), WidgetIndexPair(1, 2));
    /// assert_eq!(&RelWidgetPath::new(WidgetIndexPair(1, 2)), path.as_rel_widget_path());
    /// ```
    pub fn as_rel_widget_path(&self) -> &RelWidgetPath
    { &self.rel_widget_path }

    /// See [`widget_index_pairs`](Self::widget_index_pairs).
    pub fn iter(&self) -> WidgetPathIter<'_>
    { self.widget_index_pairs() }

    /// Pushes a pair of widget indices to the absolute widget path.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let mut path = AbsWidgetPath::new(WindowIndex(1), WidgetIndexPair(1, 2));
    /// path.push(WidgetIndexPair(3, 4));
    /// assert_eq!(WindowIndex(1), path.window_index());
    /// let mut iter = path.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), iter.next());
    /// assert_eq!(Some(WidgetIndexPair(3, 4)), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn push(&mut self, widget_idx_pair: WidgetIndexPair)
    { self.rel_widget_path.push(widget_idx_pair); }

    /// Pops the pair of widget indices from the absolute widget path.
    ///
    /// This method returns the pair of widget indices if the pair of widget indices is popped,
    /// otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::AbsWidgetPath;
    /// use lwltk::WidgetIndexPair;
    /// use lwltk::WindowIndex;
    ///
    /// let mut path1 = AbsWidgetPath::new(WindowIndex(1), WidgetIndexPair(1, 2));
    /// path1.push(WidgetIndexPair(3, 4));
    /// path1.push(WidgetIndexPair(5, 6));
    /// let idx_pair1 = path1.pop();
    /// assert_eq!(Some(WidgetIndexPair(5, 6)), idx_pair1);
    /// assert_eq!(WindowIndex(1), path1.window_index());
    /// let mut iter1 = path1.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(1, 2)), iter1.next());
    /// assert_eq!(Some(WidgetIndexPair(3, 4)), iter1.next());
    /// assert_eq!(None, iter1.next());
    /// let mut path2 = AbsWidgetPath::new(WindowIndex(2), WidgetIndexPair(2, 3));
    /// let idx_pair2 = path2.pop();
    /// assert_eq!(None, idx_pair2);
    /// assert_eq!(WindowIndex(2), path2.window_index());
    /// let mut iter2 = path2.widget_index_pairs();
    /// assert_eq!(Some(WidgetIndexPair(2, 3)), iter2.next());
    /// assert_eq!(None, iter2.next());
    /// ```
    pub fn pop(&mut self) -> Option<WidgetIndexPair>
    { self.rel_widget_path.pop() }
}
