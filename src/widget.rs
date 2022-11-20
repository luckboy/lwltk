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

pub trait Widget: Container + PreferredSize
{
    fn margin_bounds(&self) -> Rect<i32>;
    
    fn bounds(&self) -> Rect<i32>;

    fn h_align(&self) -> HAlign;
    
    fn v_align(&self) -> VAlign;

    fn state(&self) -> WidgetState;
    
    fn set_state(&mut self, state: WidgetState);
    
    fn is_focusable(&self) -> bool
    { false }
    
    fn is_focused(&self) -> bool
    { false }
    
    #[allow(unused_variables)]
    fn set_focus(&self, is_focused: bool) -> bool
    { false }

    #[allow(unused_variables)]
    fn set_viewport(&mut self, size: Size<i32>, is_focused_window: bool)
    {}
    
    fn scroll_slider_x(&self, trough_width: i32);

    fn scroll_slider_width(&self, viewport_width: i32, trough_width: i32);

    fn set_client_x(&mut self, slider_x: f64, trough_width: i32);
    
    fn scroll_slider_y(&self, trough_height: i32);
    
    fn scroll_slider_height(&self, viewport_height: i32, trough_width: i32);

    fn set_client_y(&mut self, slider_y: f64, trough_height: i32);
    
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
}

pub fn dyn_widget_as_widget<T: Any>(widget: &dyn Widget) -> Option<&T>
{ widget.as_any().downcast_ref::<T>() }

pub fn dyn_widget_mut_as_widget_mut<T: Any>(widget: &mut dyn Widget) -> Option<&mut T>
{ widget.as_any_mut().downcast_mut::<T>() }
