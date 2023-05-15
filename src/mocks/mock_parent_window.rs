//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::collections::btree_set;
use std::collections::BTreeSet;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use crate::as_any::*;
use crate::call_on::*;
use crate::client_context::*;
use crate::container::*;
use crate::draw::*;
use crate::events::*;
use crate::min_size::*;
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::types::*;
use crate::window::*;

struct MockWindowIter<'a>
{
    iter: btree_set::Iter<'a, WindowIndex>,
}

impl<'a> MockWindowIter<'a>
{
    fn new(child_indices: &'a BTreeSet<WindowIndex>) -> Self
    { MockWindowIter { iter: child_indices.iter(), } }
}

impl<'a> WindowIterator<'a> for MockWindowIter<'a>
{
    fn next(&mut self) -> Option<WindowIndex>
    { self.iter.next().map(|i| *i) }
}

pub(crate) struct MockParentWindow
{
    title: String,
    size: Size<i32>,
    padding_bounds: Rect<i32>,
    corners: Corners<i32>,
    is_visible: bool,
    is_focused: bool,
    change_flag_arc: Arc<AtomicBool>,
    min_size: Size<Option<i32>>,
    preferred_size: Size<Option<i32>>,
    child_indices: BTreeSet<WindowIndex>,
}

impl MockParentWindow
{
    pub(crate) fn new(title: &str) -> Self
    {
        MockParentWindow {
            title: String::from(title),
            size: Size::new(0, 0),
            padding_bounds: Rect::new(0, 0, 0, 0),
            corners: Corners::new(0, 0, 0, 0, 0, 0, 0, 0),
            is_visible: true,
            is_focused: false,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            min_size: Size::new(None, None),
            preferred_size: Size::new(None, None),
            child_indices: BTreeSet::new(),
        }
    }
    
    pub(crate) fn set_size(&mut self, size: Size<i32>)
    { self.size = size; }

    pub(crate) fn set_padding_bounds(&mut self, bounds: Rect<i32>)
    { self.padding_bounds = bounds; }
    
    pub(crate) fn set_visible(&mut self, is_visible: bool)
    { self.is_visible = is_visible; }
    
    pub(crate) fn set_change_flag(&mut self, is_changed: bool)
    { self.change_flag_arc.store(is_changed, Ordering::SeqCst); }
}

impl Window for MockParentWindow
{
    fn size(&self) -> Size<i32>
    { self.size }

    fn padding_bounds(&self) -> Rect<i32>
    { self.padding_bounds }

    fn corners(&self) -> Corners<i32>
    { self.corners }

    fn is_visible(&self) -> bool
    { self.is_visible }
    
    fn is_focused(&self) -> bool
    { self.is_focused }
    
    fn set_focus(&mut self, is_focused: bool)
    { self.is_focused = is_focused; }

    fn title(&self) -> Option<&str>
    { Some(self.title.as_str()) }
    
    fn is_changed(&self) -> bool
    { self.change_flag_arc.load(Ordering::SeqCst) }
    
    fn clear_change_flag(&mut self)
    { self.change_flag_arc.store(false, Ordering::SeqCst); }

    fn child_index_iter(&self) -> Option<Box<dyn WindowIterator + '_>>
    { Some(Box::new(MockWindowIter::new(&self.child_indices))) }

    #[allow(unused_variables)]
    fn add_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    {
        if self.child_indices.insert(idx.window_index()) {
            Some(())
        } else {
            None
        }
    }

    #[allow(unused_variables)]
    fn remove_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    {
        if self.child_indices.remove(&idx.window_index()) {
            Some(())
        } else {
            None
        }
    }
}

impl Container for MockParentWindow
{}

impl MinSize for MockParentWindow
{
    fn min_size(&self) -> Size<Option<i32>>
    { self.min_size }
    
    fn set_min_size(&mut self, size: Size<Option<i32>>)
    { self.min_size = size; }
}

impl PreferredSize for MockParentWindow
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    { self.preferred_size = size; }
}

impl Draw for MockParentWindow
{
    fn update_size(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_size: Size<Option<i32>>) -> Result<(), CairoError>
    { Ok(()) }
    
    fn update_pos(&mut self, _cairo_context: &CairoContext, _theme: &dyn Theme, _area_bounds: Rect<i32>) -> Result<(), CairoError>
    { Ok(()) }

    fn draw(&self, _cairo_context: &CairoContext, _theme: &dyn Theme, _is_focused_window: bool) -> Result<(), CairoError>
    { Ok(()) }
}

impl CallOn for MockParentWindow
{
    fn call_on(&mut self, _client_context: &mut ClientContext, _queue_context: &mut QueueContext, _event: &Event) -> Option<Option<Event>>
    { Some(None) }
}

impl AsAny for MockParentWindow
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}
