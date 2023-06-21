//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use crate::container::*;
use crate::cursors::*;
use crate::preferred_size::*;
use crate::types::*;

pub trait Widget: Container + PreferredSize
{
    fn margin_bounds(&self) -> Rect<i32>;
    
    fn bounds(&self) -> Rect<i32>;

    fn weight(&self) -> u32;

    fn h_align(&self) -> HAlign;
    
    fn v_align(&self) -> VAlign;

    fn state(&self) -> WidgetState;
    
    fn set_state(&mut self, state: WidgetState);

    fn is_enabled(&self) -> bool;

    fn is_focusable(&self) -> bool
    { false }
    
    fn is_focused(&self) -> bool
    { false }
    
    #[allow(unused_variables)]
    fn set_focus(&mut self, is_focused: bool) -> bool
    { false }
    
    fn is_clickable(&self) -> bool
    { self.is_focusable() }

    fn is_clickable_by_key(&self) -> bool
    { self.is_clickable() }    
    
    fn viewport_size(&self, size: Size<i32>) -> Size<i32>
    { size }
    
    #[allow(unused_variables)]
    fn set_viewport(&mut self, size: Size<i32>)
    {}
    
    fn h_scroll_bar_slider_x(&self, viewport_width: i32, trough_width: i32) -> f64;

    fn h_scroll_bar_slider_width(&self, viewport_width: i32, trough_width: i32) -> f64;

    fn set_client_x(&mut self, viewport_width: i32, slider_x: f64, trough_width: i32);
    
    fn update_client_x(&mut self, viewport_width: i32) -> bool;
    
    fn v_scroll_bar_slider_y(&self, viewport_height: i32, trough_height: i32) -> f64;
    
    fn v_scroll_bar_slider_height(&self, viewport_height: i32, trough_height: i32) -> f64;

    fn set_client_y(&mut self, viewport_height: i32, slider_y: f64, trough_height: i32);

    fn update_client_y(&mut self, viewport_height: i32) -> bool;
    
    fn set_only_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>);
    
    #[allow(unused_variables)]
    fn cursor(&self, pos: Pos<f64>, is_wait_cursor: bool) -> Cursor
    {
        if !is_wait_cursor {
            Cursor::Default
        } else {
            Cursor::Wait
        }
    }

    fn margin_pos(&self) -> Pos<i32>
    { self.margin_bounds().pos() }

    fn margin_size(&self) -> Size<i32>
    { self.margin_bounds().size() }

    fn margin_x(&self) -> i32
    { self.margin_bounds().x }

    fn margin_y(&self) -> i32
    { self.margin_bounds().y }

    fn margin_width(&self) -> i32
    { self.margin_bounds().width }

    fn margin_height(&self) -> i32
    { self.margin_bounds().height }

    fn pos(&self) -> Pos<i32>
    { self.bounds().pos() }

    fn size(&self) -> Size<i32>
    { self.bounds().size() }

    fn x(&self) -> i32
    { self.bounds().x }

    fn y(&self) -> i32
    { self.bounds().y }

    fn width(&self) -> i32
    { self.bounds().width }

    fn height(&self) -> i32
    { self.bounds().height }

    fn set_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>)
    {
        self.set_only_change_flag_arc(flag_arc.clone());
        self.set_descendant_change_flag_arcs(flag_arc);
    }
}

pub fn dyn_widget_as_widget<T: Any>(widget: &dyn Widget) -> Option<&T>
{ widget.as_any().downcast_ref::<T>() }

pub fn dyn_widget_mut_as_widget_mut<T: Any>(widget: &mut dyn Widget) -> Option<&mut T>
{ widget.as_any_mut().downcast_mut::<T>() }
