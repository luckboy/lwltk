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

/// A widget trait.
///
/// The widgets are for example buttons, labels, and text fields. Child widgets can be in the widget
/// if the widget is for example a linear layout. The widget draws with descendant widgets on a
/// window and reacts on events. If the widget is disabled, the widget should be unfocusable and
/// unclickable.
pub trait Widget: Container + PreferredSize
{
    /// Returns the margin bounds of the widget.
    fn margin_bounds(&self) -> Rect<i32>;
    
    /// Returns the bounds of the widget.
    fn bounds(&self) -> Rect<i32>;

    /// Returns the weight of the widget.
    fn weight(&self) -> u32;

    /// Returns the horizontal alignment of the widget.
    fn h_align(&self) -> HAlign;
    
    /// Returns the vertical alignment of the widget.
    fn v_align(&self) -> VAlign;

    /// Returns the state of the widget.
    fn state(&self) -> WidgetState;
    
    /// Sets the state of the widget.
    fn set_state(&mut self, state: WidgetState);

    /// Returns `true` if the widget is enabled, otherwise `false`.
    fn is_enabled(&self) -> bool;
    
    /// Returns `true` if the widget is focusable, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_focusable(&self) -> bool
    { false }
    
    /// Returns `true` if the widget is focused, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_focused(&self) -> bool
    { false }
    
    /// Sets the focus of the widget if the widget is focusable.
    ///
    /// This method should return `true` if the widget is focusable, otherwise `false`. This method
    /// defaultly returns `false` and doesn't set the focus of the widget.
    #[allow(unused_variables)]
    fn set_focus(&mut self, is_focused: bool) -> bool
    { false }
    
    /// Returns `true` if the widget is clickable, otherewise `false`.
    ///
    /// This method defaultly returns a result of the `is_focusable` method.
    fn is_clickable(&self) -> bool
    { self.is_focusable() }

    /// Returns `true` if the widget is clickable by key, otherewise `false`.
    ///
    /// This method defaultly returns a result of the `is_clickable` method.
    fn is_clickable_by_key(&self) -> bool
    { self.is_clickable() }    
    
    /// Returns the viewport size of the widget.
    ///
    /// This method defaultly returns the specified size.
    fn viewport_size(&self, size: Size<i32>) -> Size<i32>
    { size }
    
    /// Sets the viewport size of the widget.
    ///
    /// This method defaultly doesn't set the viewport of the widget.
    #[allow(unused_variables)]
    fn set_viewport(&mut self, size: Size<i32>)
    {}
    
    /// Returns the X offset of the horizontal scroll slider.
    fn h_scroll_bar_slider_x(&self, viewport_width: i32, trough_width: i32) -> f64;

    /// Returns the width of the horizontal scroll slider.
    fn h_scroll_bar_slider_width(&self, viewport_width: i32, trough_width: i32) -> f64;

    /// Sets the X offset of the widget client.
    fn set_client_x(&mut self, viewport_width: i32, slider_x: f64, trough_width: i32);
    
    /// Updates the X offset of the widget client.
    fn update_client_x(&mut self, viewport_width: i32) -> bool;
    
    /// Returns the Y offset of the vertical scroll slider.
    fn v_scroll_bar_slider_y(&self, viewport_height: i32, trough_height: i32) -> f64;
    
    /// Returns the height of the vertical scroll slider.
    fn v_scroll_bar_slider_height(&self, viewport_height: i32, trough_height: i32) -> f64;

    /// Sets the Y offset of the widget client.
    fn set_client_y(&mut self, viewport_height: i32, slider_y: f64, trough_height: i32);

    /// Updates the Y offset of the widget client.
    fn update_client_y(&mut self, viewport_height: i32) -> bool;
    
    /// Sets only the reference-counting pointer to the change flag.
    ///
    /// This method doesn't set the referernce-counting pointer to the change flag for descendant
    /// widgets. This method shouldn't be direclty used by an application. 
    fn set_only_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>);
    
    /// Returns the cursor of the widget.
    ///
    /// This method defaultly returns the default corsor if the specified flag of the wait cursor is
    /// `false`, otherwise the wait cursor.
    #[allow(unused_variables)]
    fn cursor(&self, pos: Pos<f64>, is_wait_cursor: bool) -> Cursor
    {
        if !is_wait_cursor {
            Cursor::Default
        } else {
            Cursor::Wait
        }
    }

    /// Returns the margin position of the widget.
    fn margin_pos(&self) -> Pos<i32>
    { self.margin_bounds().pos() }

    /// Returns the margin size of the widget.
    fn margin_size(&self) -> Size<i32>
    { self.margin_bounds().size() }

    /// Returns the margin X coordinate of the widget.
    fn margin_x(&self) -> i32
    { self.margin_bounds().x }

    /// Returns the margin Y coordinate of the widget.
    fn margin_y(&self) -> i32
    { self.margin_bounds().y }

    /// Returns the margin width of the widget.
    fn margin_width(&self) -> i32
    { self.margin_bounds().width }

    /// Returns the margin height of the widget.
    fn margin_height(&self) -> i32
    { self.margin_bounds().height }

    /// Returns the position of the widget.
    fn pos(&self) -> Pos<i32>
    { self.bounds().pos() }

    /// Returns the size of the widget.
    fn size(&self) -> Size<i32>
    { self.bounds().size() }

    /// Returns the X coordinate of the widget.
    fn x(&self) -> i32
    { self.bounds().x }

    /// Returns the Y coordinate of the widget.
    fn y(&self) -> i32
    { self.bounds().y }

    /// Returns the width of the widget.
    fn width(&self) -> i32
    { self.bounds().width }

    /// Returns the height of the widget.
    fn height(&self) -> i32
    { self.bounds().height }

    /// Sets the reference-counting pointer to the change flag.
    ///
    /// This method sets the referernce-counting pointer to the change flag for descendant widgets.
    /// The change flag is used to checks whether the window should be redrawn. This method
    /// shouldn't be direclty used by an application instead the
    /// [`set_only_change_flag_arc`](Self::set_only_change_flag_arc) method.
    fn set_change_flag_arc(&mut self, flag_arc: Arc<AtomicBool>)
    {
        self.set_only_change_flag_arc(flag_arc.clone());
        self.set_descendant_change_flag_arcs(flag_arc);
    }
}

/// Returns a reference to the widget for the reference to the dynamic widget or `None`.
pub fn dyn_widget_as_widget<T: Any>(widget: &dyn Widget) -> Option<&T>
{ widget.as_any().downcast_ref::<T>() }

/// Returns a mutable reference to the widget for the mutable reference to the dynamic widget or
/// `None`.
pub fn dyn_widget_mut_as_widget_mut<T: Any>(widget: &mut dyn Widget) -> Option<&mut T>
{ widget.as_any_mut().downcast_mut::<T>() }
