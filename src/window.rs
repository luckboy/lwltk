//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use crate::container::*;
use crate::preferred_size::*;
use crate::types::*;

#[derive(Copy, Clone, Debug)]
pub struct ParentWindowIndex(WindowIndex);

impl ParentWindowIndex
{
    pub(crate) fn new(idx: WindowIndex) -> ParentWindowIndex
    { ParentWindowIndex(idx) }
    
    pub fn window_index(&self) -> WindowIndex
    { self.0 }
}

#[derive(Copy, Clone, Debug)]
pub struct ChildWindowIndex(WindowIndex);

impl ChildWindowIndex
{
    pub(crate) fn new(idx: WindowIndex) -> ChildWindowIndex
    { ChildWindowIndex(idx) }

    pub fn window_index(&self) -> WindowIndex
    { self.0 }
}

pub trait Window: Container + PreferredSize
{
    fn size(&self) -> Size<i32>;

    fn padding_bounds(&self) -> Rect<i32>;

    fn is_visible(&self) -> bool;
    
    fn is_focused(&self) -> bool;
    
    fn set_focus(&self, is_focused: bool);

    fn is_popup(&self) -> bool
    { false }
    
    fn parent(&self) -> Option<WindowIndex>
    { None }
    
    #[allow(unused_variables)]
    fn set_parent(&mut self, idx: Option<ParentWindowIndex>) -> bool
    { false }

    fn child_index_iter(&self) -> Option<Box<dyn ChildWindowIndexIterator>>;
    
    #[allow(unused_variables)]
    fn add_child(&mut self, idx: ChildWindowIndex) -> bool
    { false }

    #[allow(unused_variables)]
    fn remove_child(&mut self, idx: ChildWindowIndex) -> bool
    { false }

    fn child_indices(&self) -> ChildWindowIndices<'_>
    { ChildWindowIndices::new(self.child_index_iter()) }
    
    fn width(&self) -> i32
    { self.size().width }

    fn height(&self) -> i32
    { self.size().height }

    fn padding_pos(&self) -> Pos<i32>
    { self.padding_bounds().pos() }

    fn padding_size(&self) -> Size<i32>
    { self.padding_bounds().size() }

    fn padding_x(&self) -> i32
    { self.padding_bounds().x }

    fn padding_y(&self) -> i32
    { self.padding_bounds().y }

    fn padding_width(&self) -> i32
    { self.padding_bounds().width }

    fn padding_height(&self) -> i32
    { self.padding_bounds().height }
}

pub trait ChildWindowIndexIterator<'a>
{
    fn next(&mut self) -> Option<WindowIndex>;
}

pub struct ChildWindowIndices<'a>
{
    iter: Option<Box<dyn ChildWindowIndexIterator<'a>>>,
}

impl<'a> ChildWindowIndices<'a>
{
    fn new(iter: Option<Box<dyn ChildWindowIndexIterator<'a>>>) -> Self
    { ChildWindowIndices { iter, } }
}

impl<'a> Iterator for ChildWindowIndices<'a>
{
    type Item = WindowIndex;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.as_mut().map(|i| i.next()).flatten() }
}

pub fn dyn_window_as_window<T: Any>(window: &dyn Window) -> Option<&T>
{ window.as_any().downcast_ref::<T>() }

pub fn dyn_window_mut_as_window_mut<T: Any>(window: &mut dyn Window) -> Option<&mut T>
{ window.as_any_mut().downcast_mut::<T>() }
