//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of utilities.
//!
//! The module of utilities contains one structure and miscullaneous functions. The functions of a
//! default event handlers are in this module. The default event handler is called before the event
//! handler and returns a default event.
use std::cmp::max;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::time::Duration;
use std::time::Instant;
use cairo::Format;
use cairo::ImageSurface;
use crate::client_context::*;
use crate::cursors::*;
use crate::event_queue::*;
use crate::events::*;
use crate::keys::*;
use crate::queue_context::*;
use crate::types::*;
use crate::widget::*;
use crate::window::*;

mod call_on_fun;

pub use call_on_fun::*;

/// Creates a dummy Cairo surface.
pub fn create_dummy_cairo_surface() -> Result<ImageSurface, CairoError>
{ ImageSurface::create(Format::ARgb32, 1, 1) }

/// Calls the closure with a Cairo context from the Cairo surface.
pub fn with_cairo_context<T, F>(cairo_surface: &ImageSurface, f: F) -> Result<T, CairoError>
    where F: FnOnce(&CairoContext) -> Result<T, CairoError>
{
    match CairoContext::new(&cairo_surface) {
        Ok(cairo_context) => f(&cairo_context),
        Err(err) => Err(err),
    }
}

/// Calls the closure with a Cairo context from a dummy Cairo surface.
pub fn with_dummy_cairo_context<T, F>(f: F) -> Result<T, CairoError>
    where F: FnOnce(&CairoContext) -> Result<T, CairoError>
{
    match create_dummy_cairo_surface() {
        Ok(cairo_surface) => with_cairo_context(&cairo_surface, f),
        Err(err) => Err(err),
    }
}

/// Sets the Cairo color.
pub fn set_cairo_color(cairo_context: &CairoContext, color: Color)
{ cairo_context.set_source_rgba(color.red, color.green, color.blue, color.alpha); }

/// Returns the X offset of the horizontal scroll slider.
pub fn h_scroll_bar_slider_x(client_x: i32, client_width: i32, viewport_width: i32, trough_width: i32) -> f64
{
    let max_width = max(viewport_width, client_width);
    if max_width > 0 {
        (client_x as f64) * (trough_width as f64) / (max_width as f64)
    } else {
        0.0
    }
}

/// Returns the width of the horizontal scroll slider.
pub fn h_scroll_bar_slider_width(client_width: i32, viewport_width: i32, trough_width: i32) -> f64
{
    let max_width = max(viewport_width, client_width);
    if max_width > 0 {
        (viewport_width as f64) * (trough_width as f64) / (max_width as f64)
    } else {
        trough_width as f64
    }
}

/// Sets the X offset of the widget client.
pub fn set_client_x(client_x: &mut i32, client_width: i32, viewport_width: i32, slider_x: f64, trough_width: i32)
{
    let max_width = max(viewport_width, client_width);
    if trough_width > 0 {
        *client_x = ((slider_x * (max_width as f64)) / (trough_width as f64)) as i32;
    } else {
        *client_x = 0;
    }
}

/// Updates the X offset of the widget client.
pub fn update_client_x(client_x: &mut i32, client_width: i32, viewport_width: i32) -> bool
{
    if client_width - *client_x < viewport_width {
        if client_width > viewport_width {
            *client_x = client_width - viewport_width;
            true
        } else {
            if *client_x != 0 {
                *client_x = 0;
                true
            } else {
                false
            }
        }
    } else {
        false
    }
}

/// Returns the Y offset of the vertical scroll slider.
pub fn v_scroll_bar_slider_y(client_y: i32, client_height: i32, viewport_height: i32, trough_height: i32) -> f64
{ h_scroll_bar_slider_x(client_y, client_height, viewport_height, trough_height) }

/// Returns the height of the vertical scroll slider.
pub fn v_scroll_bar_slider_height(client_height: i32, viewport_height: i32, trough_height: i32) -> f64
{ h_scroll_bar_slider_width(client_height, viewport_height, trough_height) }

/// Sets the Y offset of the widget client.
pub fn set_client_y(client_y: &mut i32, client_height: i32, viewport_height: i32, slider_y: f64, trough_height: i32)
{ set_client_x(client_y, client_height, viewport_height, slider_y, trough_height); }

/// Updates the Y offset of the widget client.
pub fn update_client_y(client_y: &mut i32, client_height: i32, viewport_height: i32) -> bool
{ update_client_x(client_y, client_height, viewport_height) }

/// Returns the X offset of the horizontal scroll slider for the client integer.
pub fn h_scroll_bar_slider_x_for_client_int(client_x: ClientInt, client_width: ClientInt, viewport_width: i32, trough_width: i32) -> f64
{
    let max_width = max(viewport_width as ClientInt, client_width);
    if max_width > 0 {
        (client_x as f64) * (trough_width as f64) / (max_width as f64)
    } else {
        0.0
    }
}

/// Returns the width of the horizontal scroll slider for the client integer.
pub fn h_scroll_bar_slider_width_for_client_int(client_width: ClientInt, viewport_width: i32, trough_width: i32) -> f64
{
    let max_width = max(viewport_width as ClientInt, client_width);
    if max_width > 0 {
        (viewport_width as f64) * (trough_width as f64) / (max_width as f64)
    } else {
        trough_width as f64
    }
}

/// Sets the X offset of the widget client for the client integer.
pub fn set_client_x_for_client_int(client_x: &mut ClientInt, client_width: ClientInt, viewport_width: i32, slider_x: f64, trough_width: i32)
{
    let max_width = max(viewport_width as ClientInt, client_width);
    if trough_width > 0 {
        *client_x = ((slider_x * (max_width as f64)) / (trough_width as f64)) as ClientInt;
    } else {
        *client_x = 0;
    }
}

/// Updates the X offset of the widget client for the client integer.
pub fn update_client_x_for_client_int(client_x: &mut ClientInt, client_width: ClientInt, viewport_width: i32) -> bool
{
    if client_width - *client_x < (viewport_width as ClientInt) {
        if client_width > (viewport_width as ClientInt) {
            *client_x = client_width - (viewport_width as ClientInt);
            true
        } else {
            if *client_x != 0 {
                *client_x = 0;
                true
            } else {
                false
            }
        }
    } else {
        false
    }
}

/// Returns the Y offset of the vertical scroll slider for the client integer.
pub fn v_scroll_bar_slider_y_for_client_int(client_y: ClientInt, client_height: ClientInt, viewport_height: i32, trough_height: i32) -> f64
{ h_scroll_bar_slider_x_for_client_int(client_y, client_height, viewport_height, trough_height) }

/// Returns the height of the vertical scroll slider for the client integer.
pub fn v_scroll_bar_slider_height_for_client_int(client_height: ClientInt, viewport_height: i32, trough_height: i32) -> f64
{ h_scroll_bar_slider_width_for_client_int(client_height, viewport_height, trough_height) }

/// Sets the Y offset of the widget client for the client integer.
pub fn set_client_y_for_client_int(client_y: &mut ClientInt, client_height: ClientInt, viewport_height: i32, slider_y: f64, trough_height: i32)
{ set_client_x_for_client_int(client_y, client_height, viewport_height, slider_y, trough_height); }

/// Updates the Y offset of the widget client for the client integer.
pub fn update_client_y_for_client_int(client_y: &mut ClientInt, client_height: ClientInt, viewport_height: i32) -> bool
{ update_client_x_for_client_int(client_y, client_height, viewport_height) }

/// A part of default event handler for the widget and the client pointer.
pub fn default_widget_on_for_client_pointer(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::PointerEnter(pos)) => {
            client_context.set_cursor(widget.cursor(*pos, queue_context.has_wait_cursor()));
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerLeave) => {
            match queue_context.motion_call_on_path(CallOnId::Pointer) {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                    queue_context.push_callback(move |_, window_context, queue_context| {
                            if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                            }
                            Some(())
                    });
                },
                _ => (),
            }
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.remove_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                widget.set_state(WidgetState::Hover);
            }
            queue_context.unset_motion_call_on_path(CallOnId::Pointer);
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerMotion(_, pos)) => {
            client_context.set_cursor(widget.cursor(*pos, queue_context.has_wait_cursor()));
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Pointer);
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                    window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            if queue_context.current_call_on_path() == queue_context.pressed_call_on_path(CallOnId::Pointer) {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                    widget.set_state(WidgetState::Active);
                }
            } else {
                widget.set_state(WidgetState::Hover);
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Pressed)) => {
            queue_context.set_pressed_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.set_pressed_instant(CallOnId::Pointer, Instant::now());
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            queue_context.set_long_click(false);
            if client_context.post_button_release_call_on_path().is_none() {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                    widget.set_state(WidgetState::Active);
                }
                queue_context.set_double_click(false);
            }
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(current_window_idx)?;
                    match window.point_focusable(current_pos) {
                        Some(focusable_widget) => {
                            window.update_focused_rel_widget_path();
                            window.set_focused_rel_widget_path(Some(focusable_widget));
                        },
                        None => (),
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Released)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Pointer);
            if pressed_call_on_path == queue_context.current_call_on_path() {
                let duration = Duration::from_millis(client_context.long_click_delay());
                if queue_context.pressed_instant(CallOnId::Pointer)?.elapsed() >= duration {
                    queue_context.set_long_click(true);
                }
                if client_context.post_button_release_call_on_path().is_some() {
                    queue_context.set_double_click(true);
                } else {
                    let current_call_on_path = queue_context.current_call_on_path()?.clone();
                    queue_context.push_callback(move |client_context, window_context, _| {
                            let tmp_call_on_path = current_call_on_path.clone();
                            client_context.send_after_button_release(tmp_call_on_path, window_context.current_pos()?);
                            Some(())
                    });
                }
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                    window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Pressed)) => {
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            let is_enabled = widget.is_enabled();
            queue_context.push_callback(move |_, window_context, queue_context| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(current_window_idx)?;
                    match window.point_focusable(current_pos) {
                        Some(focusable_widget) => {
                            window.update_focused_rel_widget_path();
                            window.set_focused_rel_widget_path(Some(focusable_widget));
                        },
                        None => (),
                    }
                    if is_enabled {
                        queue_context.event_queue_mut().push(EventPair::new(current_call_on_path.clone(), Event::PopupClick));
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Released)) => Some(Some(None)),
        Event::Client(ClientEvent::PostButtonRelease) => {
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.remove_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                widget.set_state(WidgetState::Hover);
            }
            if widget.is_enabled() {
                if queue_context.has_double_click() {
                    queue_context.push_event(Event::DoubleClick)?;
                } else if queue_context.has_long_click() {
                    queue_context.push_event(Event::LongClick)?;
                } else {
                    queue_context.push_event(Event::Click)?;
                }
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the widget, the client pointer, the scroll bar, and the
/// seek bar.
pub fn default_widget_on_for_client_pointer_and_scroll<F, G>(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event, mut f: F, mut g: G) -> Option<Option<Option<Event>>>
    where F: FnMut(&dyn Widget, &mut ClientContext, &mut QueueContext, Pos<f64>) -> Option<CallOnElem> + Send + Sync + 'static,
          G: FnMut(&mut dyn Widget, &mut ClientContext, &mut QueueContext, CallOnElem, Option<Pos<f64>>, Pos<f64>) -> Option<()>  + Send + Sync + 'static
{
    match event {
        Event::Client(ClientEvent::PointerEnter(pos)) => {
            client_context.set_cursor(widget.cursor(*pos, queue_context.has_wait_cursor()));
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerLeave) => {
            match queue_context.motion_call_on_path(CallOnId::Pointer) {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                    queue_context.push_callback(move |_, window_context, queue_context| {
                            if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                            }
                            Some(())
                    });
                },
                _ => (),
            }
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.remove_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                widget.set_state(WidgetState::Hover);
            }
            queue_context.unset_motion_call_on_path(CallOnId::Pointer);
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerMotion(_, pos)) => {
            client_context.set_cursor(widget.cursor(*pos, queue_context.has_wait_cursor()));
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Pointer);
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                    window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            if queue_context.current_call_on_path() == queue_context.pressed_call_on_path(CallOnId::Pointer) {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                    widget.set_state(WidgetState::Active);
                }
            } else {
                widget.set_state(WidgetState::Hover);
            }
            if queue_context.current_call_on_path() == queue_context.pressed_call_on_path(CallOnId::Pointer) {
                match queue_context.current_call_on_path() {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        queue_context.push_callback(move |client_context, window_context, queue_context| {
                                let current_pos = window_context.current_pos()?;
                                match (queue_context.pressed_call_on_elem(CallOnId::Pointer), queue_context.pressed_old_pos(CallOnId::Pointer)) {
                                    (Some(CallOnElem::ScrollBarElem(ScrollBarElem::Slider)), Some(old_pos)) => g(window_context.dyn_widget_mut(&tmp_abs_widget_path)?, client_context, queue_context, CallOnElem::ScrollBarElem(ScrollBarElem::Slider), Some(old_pos), current_pos)?,
                                    _ => (),
                                }
                                queue_context.set_pressed_old_pos(CallOnId::Pointer, current_pos);
                                Some(())
                        });
                    },
                    _ => return None,
                }
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Pressed)) => {
            client_context.set_first_pos(CallOnId::Pointer);
            queue_context.set_pressed_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            queue_context.set_long_click(false);
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                widget.set_state(WidgetState::Active);
            }
            queue_context.set_double_click(false);
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(current_window_idx)?;
                    match window.point_focusable(current_pos) {
                        Some(focusable_widget) => {
                            window.update_focused_rel_widget_path();
                            window.set_focused_rel_widget_path(Some(focusable_widget));
                        },
                        None => (),
                    }
                    Some(())
            });
            match queue_context.current_call_on_path() {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    queue_context.push_callback(move |client_context, window_context, queue_context| {
                            let current_pos = window_context.current_pos()?;
                            let call_on_elem = f(window_context.dyn_widget(&tmp_abs_widget_path)?, client_context, queue_context, current_pos)?;
                            queue_context.set_pressed_call_on_elem(CallOnId::Pointer, call_on_elem);
                            g(window_context.dyn_widget_mut(&tmp_abs_widget_path)?, client_context, queue_context, call_on_elem, None, current_pos)?;
                            match call_on_elem {
                                CallOnElem::Trough => queue_context.set_pressed_call_on_elem(CallOnId::Pointer, CallOnElem::ScrollBarElem(ScrollBarElem::Slider)),
                                _ => (),
                            }
                            queue_context.set_pressed_old_pos(CallOnId::Pointer, current_pos);
                            Some(())
                    });
                },
                _ => return None,
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Released)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Pointer);
            if pressed_call_on_path == queue_context.current_call_on_path() {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.remove_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                    if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                        widget.set_state(WidgetState::Hover);
                    } else {
                        widget.set_state(WidgetState::None);
                    }
                }
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                    window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Pressed)) => {
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(current_window_idx)?;
                    match window.point_focusable(current_pos) {
                        Some(focusable_widget) => {
                            window.update_focused_rel_widget_path();
                            window.set_focused_rel_widget_path(Some(focusable_widget));
                        },
                        None => (),
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Released)) => Some(Some(None)),
        Event::Client(ClientEvent::RepeatedButton) => {
            match queue_context.current_call_on_path() {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    queue_context.push_callback(move |client_context, window_context, queue_context| {
                            let current_pos = window_context.current_pos()?;
                            match queue_context.pressed_call_on_elem(CallOnId::Pointer) {
                                Some(call_on_elem @ CallOnElem::ScrollBarElem(ScrollBarElem::FirstButton | ScrollBarElem::SecondButton)) => { 
                                    g(window_context.dyn_widget_mut(&tmp_abs_widget_path)?, client_context, queue_context, call_on_elem, None, current_pos)?;
                                },
                                _ => (),
                            }
                            Some(())
                    });
                },
                _ => return None,
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the widget and the client keyboard.
pub fn default_widget_on_for_client_keyboard(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::KeyboardEnter) => {
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardLeave) => {
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, s, ClientState::Pressed)) => {
            for key in keys {
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()))?;
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c))?;
            }
            if widget.is_clickable_by_key() {
                if keys.iter().any(|k| *k == VKey::Return || *k == VKey::Space) {
                    let current_call_on_path = queue_context.current_call_on_path()?.clone();
                    if keys.iter().any(|k| *k == VKey::Return) {
                        if queue_context.add_active_id(&current_call_on_path, ActiveId::ReturnKey) {
                            widget.set_state(WidgetState::Active);
                        }
                    }
                    if keys.iter().any(|k| *k == VKey::Space) {
                        if queue_context.add_active_id(&current_call_on_path, ActiveId::SpaceKey) {
                            widget.set_state(WidgetState::Active);
                        }
                    }
                }
            }
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, _, ClientState::Released)) => {
            if widget.is_clickable_by_key() {
                if keys.iter().any(|k| *k == VKey::Return || *k == VKey::Space) {
                    let current_call_on_path = queue_context.current_call_on_path()?.clone();
                    if keys.iter().any(|k| *k == VKey::Return) {
                        if queue_context.remove_active_id(&current_call_on_path, ActiveId::ReturnKey) {
                            if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                                widget.set_state(WidgetState::Hover);
                            } else {
                                widget.set_state(WidgetState::None);
                            }
                        }
                    }
                    if keys.iter().any(|k| *k == VKey::Space) {
                        if queue_context.remove_active_id(&current_call_on_path, ActiveId::SpaceKey) {
                            if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                                widget.set_state(WidgetState::Hover);
                            } else {
                                widget.set_state(WidgetState::None);
                            }
                        }
                    }
                    if widget.is_enabled() {
                        queue_context.push_event(Event::Click)?;
                    }
                }
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardModifiers(_)) => Some(Some(None)),
        Event::Client(ClientEvent::RepeatedKey(keys, s)) => {
            for key in keys {
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()))?;
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c))?;
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the widget and the client touch.
pub fn default_widget_on_for_client_touch(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::TouchDown(_, id, _)) => {
            queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Touch(*id));
            queue_context.set_pressed_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.set_pressed_instant(CallOnId::Touch(*id), Instant::now());
            queue_context.unset_pressed_call_on_elem(CallOnId::Touch(*id));
            queue_context.unset_pressed_old_pos(CallOnId::Touch(*id));
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Touch(*id))) {
                widget.set_state(WidgetState::Active);
            }
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(current_window_idx)?;
                    match window.point_focusable(current_pos) {
                        Some(focusable_widget) => {
                            window.update_focused_rel_widget_path();
                            window.set_focused_rel_widget_path(Some(focusable_widget));
                        },
                        None => (),
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchUp(_, id)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Touch(*id));
            if pressed_call_on_path == queue_context.current_call_on_path() {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.remove_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Touch(*id))) {
                    if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                        widget.set_state(WidgetState::Hover);
                    } else {
                        widget.set_state(WidgetState::None);
                    }
                }
                let duration = Duration::from_millis(client_context.long_click_delay());
                if widget.is_enabled() {
                    if queue_context.pressed_instant(CallOnId::Touch(*id))?.elapsed() >= duration {
                        queue_context.push_event(Event::LongClick)?;
                    } else {
                        queue_context.push_event(Event::Click)?;
                    }
                }
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        let tmp_id = *id; 
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Touch(tmp_id))) {
                                    if queue_context.motion_call_on_path(CallOnId::Pointer) == Some(&tmp_call_on_path) {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::Hover);
                                    } else {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                    }
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.unset_motion_call_on_path(CallOnId::Touch(*id));
            queue_context.unset_pressed_call_on_path(CallOnId::Touch(*id));
            queue_context.unset_pressed_instant(CallOnId::Touch(*id));
            queue_context.unset_pressed_call_on_elem(CallOnId::Touch(*id));
            queue_context.unset_pressed_old_pos(CallOnId::Touch(*id));
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchMotion(_, id, _)) => {
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Touch(*id));
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        let tmp_id = *id;
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Touch(tmp_id))) {
                                    if queue_context.motion_call_on_path(CallOnId::Pointer) == Some(&tmp_call_on_path) {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::Hover);
                                    } else {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                    }
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            if queue_context.current_call_on_path() == queue_context.pressed_call_on_path(CallOnId::Touch(*id)) {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Touch(*id))) {
                    widget.set_state(WidgetState::Active);
                }
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the widget, the client touch, the scroll bar, and the seek
/// bar.
pub fn default_widget_on_for_client_touch_and_scroll<F, G>(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event, mut f: F, mut g: G) -> Option<Option<Option<Event>>>
    where F: FnMut(&dyn Widget, &mut ClientContext, &mut QueueContext, Pos<f64>) -> Option<CallOnElem> + Send + Sync + 'static,
          G: FnMut(&mut dyn Widget, &mut ClientContext, &mut QueueContext, CallOnElem, Option<Pos<f64>>, Pos<f64>) -> Option<()>  + Send + Sync + 'static
{
    match event {
        Event::Client(ClientEvent::TouchDown(_, id, _)) => {
            client_context.set_first_pos(CallOnId::Touch(*id));
            queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Touch(*id));
            queue_context.set_pressed_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.unset_pressed_instant(CallOnId::Touch(*id));
            queue_context.unset_pressed_call_on_elem(CallOnId::Touch(*id));
            queue_context.unset_pressed_old_pos(CallOnId::Touch(*id));
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Touch(*id))) {
                widget.set_state(WidgetState::Active);
            }
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(current_window_idx)?;
                    match window.point_focusable(current_pos) {
                        Some(focusable_widget) => {
                            window.update_focused_rel_widget_path();
                            window.set_focused_rel_widget_path(Some(focusable_widget));
                        },
                        None => (),
                    }
                    Some(())
            });
            match queue_context.current_call_on_path() {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    let tmp_id = *id;
                    queue_context.push_callback(move |client_context, window_context, queue_context| {
                            let current_pos = window_context.current_pos()?;
                            let call_on_elem = f(window_context.dyn_widget(&tmp_abs_widget_path)?, client_context, queue_context, current_pos)?;
                            queue_context.set_pressed_call_on_elem(CallOnId::Touch(tmp_id), call_on_elem);
                            g(window_context.dyn_widget_mut(&tmp_abs_widget_path)?, client_context, queue_context, call_on_elem, None, current_pos)?;
                            match call_on_elem {
                                CallOnElem::Trough => queue_context.set_pressed_call_on_elem(CallOnId::Pointer, CallOnElem::ScrollBarElem(ScrollBarElem::Slider)),
                                _ => (),
                            }
                            queue_context.set_pressed_old_pos(CallOnId::Touch(tmp_id), current_pos);
                            Some(())
                    });
                },
                _ => return None,
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchUp(_, id)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Touch(*id));
            if pressed_call_on_path == queue_context.current_call_on_path() {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.remove_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Touch(*id))) {
                    if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                        widget.set_state(WidgetState::Hover);
                    } else {
                        widget.set_state(WidgetState::None);
                    }
                }
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        let tmp_id = *id; 
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Touch(tmp_id))) {
                                    if queue_context.motion_call_on_path(CallOnId::Pointer) == Some(&tmp_call_on_path) {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::Hover);
                                    } else {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                    }
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.unset_motion_call_on_path(CallOnId::Touch(*id));
            queue_context.unset_pressed_call_on_path(CallOnId::Touch(*id));
            queue_context.unset_pressed_instant(CallOnId::Touch(*id));
            queue_context.unset_pressed_call_on_elem(CallOnId::Touch(*id));
            queue_context.unset_pressed_old_pos(CallOnId::Touch(*id));
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchMotion(_, id, _)) => {
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Touch(*id));
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        let tmp_id = *id;
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Touch(tmp_id))) {
                                    if queue_context.motion_call_on_path(CallOnId::Pointer) == Some(&tmp_call_on_path) {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::Hover);
                                    } else {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                    }
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            if queue_context.current_call_on_path() == queue_context.pressed_call_on_path(CallOnId::Touch(*id)) {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.add_active_id(&current_call_on_path, ActiveId::CallOnId(CallOnId::Touch(*id))) {
                    widget.set_state(WidgetState::Active);
                }
            }
            if queue_context.current_call_on_path() == queue_context.pressed_call_on_path(CallOnId::Pointer) {
                match queue_context.current_call_on_path() {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_id = *id;
                        queue_context.push_callback(move |client_context, window_context, queue_context| {
                                let current_pos = window_context.current_pos()?;
                                match (queue_context.pressed_call_on_elem(CallOnId::Touch(tmp_id)), queue_context.pressed_old_pos(CallOnId::Pointer)) {
                                    (Some(CallOnElem::ScrollBarElem(ScrollBarElem::Slider)), Some(old_pos)) => g(window_context.dyn_widget_mut(&tmp_abs_widget_path)?, client_context, queue_context, CallOnElem::ScrollBarElem(ScrollBarElem::Slider), Some(old_pos), current_pos)?,
                                    _ => (),
                                }
                                queue_context.set_pressed_old_pos(CallOnId::Touch(tmp_id), current_pos);
                                Some(())
                        });
                    },
                    _ => return None,
                }
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::RepeatedTouch(id)) => {
            match queue_context.current_call_on_path() {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    let tmp_id = *id;
                    queue_context.push_callback(move |client_context, window_context, queue_context| {
                            let current_pos = window_context.current_pos()?;
                            match queue_context.pressed_call_on_elem(CallOnId::Touch(tmp_id)) {
                                Some(call_on_elem @ CallOnElem::ScrollBarElem(ScrollBarElem::FirstButton | ScrollBarElem::SecondButton)) => { 
                                    g(window_context.dyn_widget_mut(&tmp_abs_widget_path)?, client_context, queue_context, call_on_elem, None, current_pos)?;
                                },
                                _ => (),
                            }
                            Some(())
                    });
                },
                _ => return None,
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the widget and the clicks.
#[allow(unused_variables)]
pub fn default_widget_on_for_clicks(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Click | Event::DoubleClick | Event::LongClick | Event::PopupClick => {
            if widget.is_clickable() {
                Some(Some(None))
            } else {
                Some(Some(Some(event.clone())))
            }
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the widget, the key, and the character.
#[allow(unused_variables)]
pub fn default_widget_on_for_key_and_char(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Key(_, _) | Event::Char(_) => Some(Some(Some(event.clone()))),
        _ => Some(None),
    }
}

/// A part of default event handler for the widget and the window events.
#[allow(unused_variables)]
pub fn default_widget_on_for_window_events(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Menu | Event::Close | Event::Maximize=> Some(Some(Some(event.clone()))),
        _ => Some(None),
    }
}

/// A default event handler for the widget.
pub fn default_widget_on(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    if let Some(res) = default_widget_on_for_client_pointer(widget, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_widget_on_for_client_keyboard(widget, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_widget_on_for_client_touch(widget, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_widget_on_for_clicks(widget, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_widget_on_for_key_and_char(widget, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_widget_on_for_window_events(widget, client_context, queue_context, event)? {
        Some(Some(res))
    } else {
        Some(None)
    }
}

/// Returns a client resize for the position, the window egdes, and the window corners.
pub fn client_resize_for_pos(pos: Pos<f64>, size: Size<i32>, edges: Edges<i32>, corners: Corners<i32>) -> Option<ClientResize>
{
    let bottom = Rect::new(0, size.height - edges.bottom, size.width, edges.bottom).to_f64_rect();
    if bottom.contains(pos) {
        let bottom_right = Rect::new(size.width - corners.bottom_right_width, size.height - corners.bottom_right_height, corners.bottom_right_width, corners.bottom_right_height).to_f64_rect();
        if bottom_right.contains(pos) {
            return Some(ClientResize::BottomRight);
        }
        let bottom_left = Rect::new(0, size.height - corners.bottom_left_width, corners.bottom_left_width, corners.bottom_left_height).to_f64_rect();
        if bottom_left.contains(pos) {
            return Some(ClientResize::BottomLeft);
        }
        return Some(ClientResize::Bottom);
    }
    let right = Rect::new(size.width - edges.right, 0, edges.right, size.height).to_f64_rect();
    if right.contains(pos) {
        let bottom_right = Rect::new(size.width - corners.bottom_right_width, size.height - corners.bottom_right_height, corners.bottom_right_width, corners.bottom_right_height).to_f64_rect();
        if bottom_right.contains(pos) {
            return Some(ClientResize::BottomRight);
        }
        let top_right = Rect::new(size.width - corners.top_right_width, 0, corners.top_right_width, corners.top_right_height).to_f64_rect();
        if top_right.contains(pos) {
            return Some(ClientResize::TopRight);
        }
        return Some(ClientResize::Right);
    }
    let left = Rect::new(0, 0, edges.left, size.height).to_f64_rect();
    if left.contains(pos) {
        let bottom_left = Rect::new(0, size.height - corners.bottom_left_width, corners.bottom_left_width, corners.bottom_left_height).to_f64_rect();
        if bottom_left.contains(pos) {
            return Some(ClientResize::BottomLeft);
        }
        let top_left = Rect::new(0, 0, corners.top_left_width, corners.top_left_height).to_f64_rect();
        if top_left.contains(pos) {
            return Some(ClientResize::TopLeft);
        }
        return Some(ClientResize::Left);
    }
    let top = Rect::new(0, 0, size.width, edges.top).to_f64_rect();
    if top.contains(pos) {
        let top_right = Rect::new(size.width - corners.top_right_width, 0, corners.top_right_width, corners.top_right_height).to_f64_rect();
        if top_right.contains(pos) {
            return Some(ClientResize::TopRight);
        }
        let top_left = Rect::new(0, 0, corners.top_left_width, corners.top_left_height).to_f64_rect();
        if top_left.contains(pos) {
            return Some(ClientResize::TopLeft);
        }
        return Some(ClientResize::Top);
    }
    None
}

/// Returns for the client resize and the resizable flag. 
pub fn cursor_for_client_resize_and_resizable(_edges: Option<ClientResize>, _is_resizable: bool) -> Cursor
{
    //if is_resizable {
    //    match edges {
    //        Some(ClientResize::Top) => Cursor::TopSide,
    //        Some(ClientResize::Bottom) => Cursor::BottomSide,
    //        Some(ClientResize::Left) => Cursor::LeftSide,
    //        Some(ClientResize::Right) => Cursor::RightSide,
    //        Some(ClientResize::TopLeft) => Cursor::TopLeftCorner,
    //        Some(ClientResize::TopRight) => Cursor::TopRightCorner,
    //        Some(ClientResize::BottomLeft) => Cursor::BottomLeftCorner,
    //        Some(ClientResize::BottomRight) => Cursor::BottomRightCorner,
    //        _ => Cursor::Default,
    //    }
    //} else {
    //    Cursor::Default
    //}
    Cursor::Default
}

/// A part of default event handler for the window and the client shell surface.
#[allow(unused_variables)]
pub fn default_window_on_for_client_shell_surface(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::ShellSurfaceConfigure(_, size)) => {
            window.set_preferred_size(Size::new(Some(size.width), Some(size.height)));
            Some(Some(None))
        },
        Event::Client(ClientEvent::ShellSurfacePopupDone) => {
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    window_context.unset_parent_window(current_window_idx)?;
                    Some(())
            });
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the window and the client pointer.
pub fn default_window_on_for_client_pointer(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::PointerEnter(pos)) => {
            let resize_edges = client_resize_for_pos(*pos, window.size(), window.edges(), window.corners());
            client_context.set_cursor(cursor_for_client_resize_and_resizable(resize_edges, window.is_resizable()));
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            match resize_edges {
                Some(resize_edges) => queue_context.set_motion_resize_edges(CallOnId::Pointer, resize_edges),
                None => queue_context.unset_motion_resize_edges(CallOnId::Pointer),
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerLeave) => {
            match queue_context.motion_call_on_path(CallOnId::Pointer) {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                    queue_context.push_callback(move |_, window_context, queue_context| {
                            if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                            }
                            Some(())
                    });
                },
                _ => (),
            }
            queue_context.unset_motion_call_on_path(CallOnId::Pointer);
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
            queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerMotion(_, pos)) => {
            let resize_edges = client_resize_for_pos(*pos, window.size(), window.edges(), window.corners());
            client_context.set_cursor(cursor_for_client_resize_and_resizable(resize_edges, window.is_resizable()));
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Pointer);
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                    window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            match resize_edges {
                Some(resize_edges) => queue_context.set_motion_resize_edges(CallOnId::Pointer, resize_edges),
                None => queue_context.unset_motion_resize_edges(CallOnId::Pointer),
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Pressed)) => {
            match queue_context.motion_resize_edges(CallOnId::Pointer) {
                Some(resize_edges) => {
                    if window.is_resizable() {
                        window.resize(resize_edges);
                    }
                },
                None => {
                    queue_context.set_pressed_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
                    queue_context.set_pressed_instant(CallOnId::Pointer, Instant::now());
                    queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
                    queue_context.unset_pressed_old_pos(CallOnId::Pointer);
                    queue_context.set_long_click(false);
                    if client_context.post_button_release_call_on_path().is_none() {
                        queue_context.set_double_click(false);
                    }
                },
            }
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Released)) => {
            if queue_context.motion_resize_edges(CallOnId::Pointer).is_none() {
                let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Pointer);
                if pressed_call_on_path == queue_context.current_call_on_path() {
                    let duration = Duration::from_millis(client_context.long_click_delay());
                    if queue_context.pressed_instant(CallOnId::Pointer)?.elapsed() >= duration {
                        queue_context.set_long_click(true);
                    }
                    if client_context.post_button_release_call_on_path().is_some() {
                        queue_context.set_double_click(true);
                    } else {
                        let current_call_on_path = queue_context.current_call_on_path()?.clone();
                        queue_context.push_callback(move |client_context, window_context, _| {
                                let tmp_call_on_path = current_call_on_path.clone();
                                client_context.send_after_button_release(tmp_call_on_path, window_context.current_pos()?);
                                Some(())
                        });
                    }
                } else {
                    match pressed_call_on_path {
                        Some(CallOnPath::Widget(abs_widget_path)) => {
                            let tmp_abs_widget_path = abs_widget_path.clone();
                            let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                            queue_context.push_callback(move |_, window_context, queue_context| {
                                    if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Pointer)) {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                    }
                                    Some(())
                            });
                        },
                        _ => (),
                    }
                }
                queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
                queue_context.unset_pressed_instant(CallOnId::Pointer);
                queue_context.unset_pressed_call_on_elem(CallOnId::Pointer);
                queue_context.unset_pressed_old_pos(CallOnId::Pointer);
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Pressed)) => {
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            let are_motion_resize_edges = queue_context.motion_resize_edges(CallOnId::Pointer).is_some();
            queue_context.push_callback(move |_, window_context, queue_context| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    if !are_motion_resize_edges {
                        queue_context.event_queue_mut().push(EventPair::new(current_call_on_path.clone(), Event::PopupClick));
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Released)) => Some(Some(None)),
        Event::Client(ClientEvent::PostButtonRelease) => {
            if queue_context.has_double_click() {
                queue_context.push_event(Event::DoubleClick)?;
            } else if queue_context.has_long_click() {
                queue_context.push_event(Event::LongClick)?;
            } else {
                queue_context.push_event(Event::Click)?;
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the window and the client keyboard.
#[allow(unused_variables)]
pub fn default_window_on_for_client_keyboard(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::KeyboardEnter) => {
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardLeave) => {
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, s, ClientState::Pressed)) => {
            for key in keys {
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()))?;
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c))?;
            }
            queue_context.push_callback(move |_, window_context, _| {
                    let current_window_idx = window_context.current_window_index()?;
                    let window = window_context.dyn_window(current_window_idx)?;
                    if window.is_focusable() {
                        window_context.set_focused_window_index(Some(current_window_idx));
                    }
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, _, ClientState::Released)) => Some(Some(None)),
        Event::Client(ClientEvent::KeyboardModifiers(_)) => Some(Some(None)),
        Event::Client(ClientEvent::RepeatedKey(keys, s)) => {
            for key in keys {
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()))?;
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c))?;
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the window and the client touch.
pub fn default_window_on_for_client_touch(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::TouchDown(_, id, pos)) => {
            let resize_edges = client_resize_for_pos(*pos, window.size(), window.edges(), window.corners());
            match resize_edges {
                Some(resize_edges) => {
                    queue_context.set_motion_resize_edges(CallOnId::Touch(*id), resize_edges);
                    window.resize(resize_edges);
                },
                None => {
                    queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
                    queue_context.unset_motion_resize_edges(CallOnId::Touch(*id));
                    queue_context.set_pressed_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
                    queue_context.set_pressed_instant(CallOnId::Touch(*id), Instant::now());
                    queue_context.unset_pressed_call_on_elem(CallOnId::Touch(*id));
                    queue_context.unset_pressed_old_pos(CallOnId::Touch(*id));
                    queue_context.push_callback(move |_, window_context, _| {
                            let current_window_idx = window_context.current_window_index()?;
                            let window = window_context.dyn_window(current_window_idx)?;
                            if window.is_focusable() {
                                window_context.set_focused_window_index(Some(current_window_idx));
                            }
                           Some(())
                    });
                },
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchUp(_, id)) => {
            if queue_context.motion_resize_edges(CallOnId::Touch(*id)).is_none() {
                let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Touch(*id));
                if pressed_call_on_path == queue_context.current_call_on_path() {
                    let duration = Duration::from_millis(client_context.long_click_delay());
                    if queue_context.pressed_instant(CallOnId::Touch(*id))?.elapsed() >= duration {
                        queue_context.push_event(Event::LongClick)?;
                    } else {
                        queue_context.push_event(Event::Click)?;
                    }
                } else {
                    match pressed_call_on_path {
                        Some(CallOnPath::Widget(abs_widget_path)) => {
                            let tmp_abs_widget_path = abs_widget_path.clone();
                            let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                            let tmp_id = *id; 
                            queue_context.push_callback(move |_, window_context, queue_context| {
                                    if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Touch(tmp_id))) {
                                        if queue_context.motion_call_on_path(CallOnId::Pointer) == Some(&tmp_call_on_path) {
                                            window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::Hover);
                                        } else {
                                            window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                        }
                                    }
                                    Some(())
                            });
                        },
                        _ => (),
                    }
                }
                queue_context.unset_motion_call_on_path(CallOnId::Touch(*id));
                queue_context.unset_pressed_call_on_path(CallOnId::Touch(*id));
                queue_context.unset_pressed_instant(CallOnId::Touch(*id));
                queue_context.unset_pressed_call_on_elem(CallOnId::Touch(*id));
                queue_context.unset_pressed_old_pos(CallOnId::Touch(*id));
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchMotion(_, id, pos)) => {
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Touch(*id));
            let resize_edges = client_resize_for_pos(*pos, window.size(), window.edges(), window.corners());
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        let tmp_id = *id;
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.remove_active_id(&tmp_call_on_path, ActiveId::CallOnId(CallOnId::Touch(tmp_id))) {
                                    if queue_context.motion_call_on_path(CallOnId::Pointer) == Some(&tmp_call_on_path) {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::Hover);
                                    } else {
                                        window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                    }
                                }
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            match resize_edges {
                Some(resize_edges) => queue_context.set_motion_resize_edges(CallOnId::Pointer, resize_edges),
                None => queue_context.unset_motion_resize_edges(CallOnId::Pointer),
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the window and the clicks.
#[allow(unused_variables)]
pub fn default_window_on_for_clicks(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Click | Event::DoubleClick | Event::LongClick | Event::PopupClick => Some(Some(None)),
        _ => Some(None),
    }
}

/// A part of default event handler for the window and the key.
#[allow(unused_variables)]
pub fn default_window_on_for_key(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Key(key, modifiers) => {
            if *modifiers == KeyModifiers::EMPTY {
                match *key {
                    VKey::Escape | VKey::PageUp => {
                        window.update_focused_rel_widget_path();
                        window.up_focused_widget()?;
                        Some(Some(None))
                    },
                    VKey::PageDown => {
                        window.update_focused_rel_widget_path();
                        window.down_focused_widget()?;
                        Some(Some(None))
                    },
                    VKey::Up | VKey::Left => {
                        window.update_focused_rel_widget_path();
                        window.prev_focused_widget()?;
                        Some(Some(None))
                    },
                    VKey::Down | VKey::Right => {
                        window.update_focused_rel_widget_path();
                        window.next_focused_widget()?;
                        Some(Some(None))
                    },
                    _ => Some(None),
                }
            } else if *modifiers == KeyModifiers::ALT {
                match *key {
                    VKey::F4 => {
                        queue_context.push_event(Event::Close)?;
                        Some(Some(None))
                    },
                    _ => Some(None),
                }
            } else {
                Some(None)
            }
        },
        _ => Some(None),
    }
}

/// A part of default event handler for the window and the window maximization.
#[allow(unused_variables)]
pub fn default_window_on_for_maximize(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Maximize => {
            if window.is_maximizable() {
                if !window.is_maximized() {
                    window.maximize();
                } else {
                    window.unmaximize();
                }
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

/// A default event handler for the window.
pub fn default_window_on(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    if let Some(res) = default_window_on_for_client_shell_surface(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_window_on_for_client_pointer(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_window_on_for_client_keyboard(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_window_on_for_client_touch(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_window_on_for_clicks(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_window_on_for_key(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else if let Some(res) = default_window_on_for_maximize(window, client_context, queue_context, event)? {
        Some(Some(res))
    } else {
        Some(None)
    }
}

/// Returns `true` if the character is mark, otherwise `false`.
pub fn is_mark_char(c: char) -> bool
{
    (c >= '\u{0300}' && c <= '\u{036f}') || (c >= '\u{1ab0}' && c <= '\u{1ace}') ||
    (c >= '\u{1dc0}' && c <= '\u{1dff}') || (c >= '\u{20d0}' && c <= '\u{20f0}') ||
    (c >= '\u{fe20}' && c <= '\u{fe2f}') 
}

/// Returns `true` if the character is mark for two characters, otherwise `false`.
pub fn is_mark_char2(c: char) -> bool
{ c >= '\u{035c}' && c <= '\u{0362}' }

/// Returns a position of inner rectangle for the rectangle and the edges.
///
/// # Examples
/// ```
/// use lwltk::utils::inner_pos;
/// use lwltk::Edges;
/// use lwltk::Pos;
/// use lwltk::Rect;
///
/// let pos = inner_pos(Rect::new(1, 2, 8, 10), Edges::new(1, 1, 2, 2));
/// assert_eq!(Pos::new(3, 3), pos);
/// ```
pub fn inner_pos<T>(rect: Rect<T>, edges: Edges<T>) -> Pos<T>
    where T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32>
{
    let x = if rect.width >= edges.left + edges.right {
        rect.x + edges.left
    } else {
        let tmp_x = (rect.width - edges.right + edges.left) / T::from(2);
        if tmp_x < T::from(0) {
            rect.x
        } else if tmp_x > rect.width {
            rect.x + rect.width
        } else {
            rect.x + tmp_x
        }
    };
    let y = if rect.height >= edges.top + edges.bottom {
        rect.y + edges.top
    } else {
        let tmp_y = (rect.height - edges.bottom + edges.top) / T::from(2);
        if tmp_y < T::from(0) {
            rect.y
        } else if tmp_y > rect.height {
            rect.y + rect.height
        } else {
            rect.y + tmp_y
        }
    };
    Pos::new(x, y)
}

/// Returns a size of inner rectangle for the size and the edges.
///
/// # Examples
/// ```
/// use lwltk::utils::inner_size;
/// use lwltk::Edges;
/// use lwltk::Size;
///
/// let size = inner_size(Size::new(8, 10), Edges::new(1, 1, 2, 2));
/// assert_eq!(Size::new(4, 8), size);
/// ```
pub fn inner_size<T>(size: Size<T>, edges: Edges<T>) -> Size<T>
    where T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + From<i32>
{
    let width = if size.width >= edges.left + edges.right {
        size.width - edges.left - edges.right
    } else {
        T::from(0)
    };
    let height = if size.height >= edges.top + edges.bottom {
        size.height - edges.top - edges.bottom
    } else {
        T::from(0)
    };
    Size::new(width, height)
}

/// Returns an inner rectangle for the rectangle and the edges.
///
/// # Examples
/// ```
/// use lwltk::utils::inner_rect;
/// use lwltk::Edges;
/// use lwltk::Rect;
///
/// let rect = inner_rect(Rect::new(1, 2, 8, 10), Edges::new(1, 1, 2, 2));
/// assert_eq!(Rect::new(3, 3, 4, 8), rect);
/// ```
pub fn inner_rect<T>(rect: Rect<T>, edges: Edges<T>) -> Rect<T>
    where T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32>
{
    let pos = inner_pos(rect, edges);
    let size = inner_size(rect.size(), edges);
    Rect::new(pos.x, pos.y, size.width, size.height)
}

/// Returns an optional size of an inner rectangle for the optional size, the egdes.
///
/// # Examples
/// ```
/// use lwltk::utils::inner_opt_size;
/// use lwltk::Edges;
/// use lwltk::Size;
///
/// let size1 = inner_opt_size(Size::new(Some(8), None), Edges::new(1, 1, 2, 2));
/// let size2 = inner_opt_size(Size::new(None, Some(10)), Edges::new(1, 1, 2, 2));
/// assert_eq!(Size::new(Some(4), None), size1);
/// assert_eq!(Size::new(None, Some(8)), size2);
/// ```
pub fn inner_opt_size<T>(size: Size<Option<T>>, edges: Edges<T>) -> Size<Option<T>>
    where T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + From<i32>
{
    let width = match size.width {
        Some(tmp_width) => {
            if tmp_width >= edges.left + edges.right {
                Some(tmp_width - edges.left - edges.right)
            } else {
                Some(T::from(0))
            }
        },
        None => None,
    };
    let height = match size.height {
        Some(tmp_height) => {
            if tmp_height >= edges.top + edges.bottom {
                Some(tmp_height - edges.top - edges.bottom)
            } else {
                Some(T::from(0))
            }
        },
        None => None,
    };
    Size::new(width, height)
}

/// Returns a position of outer rectangle for the position and the edges.
///
/// # Examples
/// ```
/// use lwltk::utils::outer_pos;
/// use lwltk::Edges;
/// use lwltk::Pos;
///
/// let pos = outer_pos(Pos::new(3, 4), Edges::new(1, 1, 2, 2));
/// assert_eq!(Pos::new(1, 3), pos);
/// ```
pub fn outer_pos<T>(pos: Pos<T>, edges: Edges<T>) -> Pos<T>
    where T: Sub<Output = T>
{ Pos::new(pos.x - edges.left, pos.y - edges.top) }

/// Returns a size of outer rectangle for the size and the edges.
///
/// # Examples
/// ```
/// use lwltk::utils::outer_size;
/// use lwltk::Edges;
/// use lwltk::Size;
///
/// let size = outer_size(Size::new(4, 6), Edges::new(1, 1, 2, 2));
/// assert_eq!(Size::new(8, 8), size);
/// ```
pub fn outer_size<T>(size: Size<T>, edges: Edges<T>) -> Size<T>
    where T: Add<Output = T>
{ Size::new(size.width + edges.left + edges.right, size.height + edges.top + edges.bottom) }

/// Returns an outer rectangle for the rectangle and the edges.
///
/// # Examples
/// ```
/// use lwltk::utils::outer_rect;
/// use lwltk::Edges;
/// use lwltk::Rect;
///
/// let rect = outer_rect(Rect::new(3, 4, 4, 6), Edges::new(1, 1, 2, 2));
/// assert_eq!(Rect::new(1, 3, 8, 8), rect);
/// ```
pub fn outer_rect<T>(rect: Rect<T>, edges: Edges<T>) -> Rect<T>
    where T: Copy + Add<Output = T> + Sub<Output = T>
{
    let pos = outer_pos(rect.pos(), edges);
    let size = outer_size(rect.size(), edges);
    Rect::new(pos.x, pos.y, size.width, size.height)
}

/// Returns an optional size of an outer rectangle for the optional size, the egdes.
///
/// # Examples
/// ```
/// use lwltk::utils::outer_opt_size;
/// use lwltk::Edges;
/// use lwltk::Size;
///
/// let size1 = outer_opt_size(Size::new(Some(4), None), Edges::new(1, 1, 2, 2));
/// let size2 = outer_opt_size(Size::new(None, Some(6)), Edges::new(1, 1, 2, 2));
/// assert_eq!(Size::new(Some(8), None), size1);
/// assert_eq!(Size::new(None, Some(8)), size2);
/// ```
pub fn outer_opt_size<T>(size: Size<Option<T>>, edges: Edges<T>) -> Size<Option<T>>
    where T: Add<Output = T>
{
    let width = match size.width {
        Some(tmp_width) => Some(tmp_width + edges.left + edges.right),
        None => None,
    };
    let height = match size.height {
        Some(tmp_height) => Some(tmp_height + edges.top + edges.bottom),
        None => None,
    };
    Size::new(width, height)
}

/// Returns a maximal width for the width and the optional width.
///
/// # Examples
/// ```
/// use lwltk::utils::max_width_for_opt_width;
///
/// assert_eq!(2, max_width_for_opt_width(1, Some(2)));
/// assert_eq!(3, max_width_for_opt_width(3, Some(2)));
/// assert_eq!(4, max_width_for_opt_width(4, None));
/// ```
pub fn max_width_for_opt_width<T>(width1: T, width2: Option<T>) -> T
    where T: PartialOrd
{
    match width2 {
        Some(width) => if width > width1 { width } else { width1 },
        None => width1,
    }
}

/// Returns a maximal height for the height and the optional height.
///
/// # Examples
/// ```
/// use lwltk::utils::max_height_for_opt_height;
///
/// assert_eq!(2, max_height_for_opt_height(1, Some(2)));
/// assert_eq!(3, max_height_for_opt_height(3, Some(2)));
/// assert_eq!(4, max_height_for_opt_height(4, None));
/// ```
pub fn max_height_for_opt_height<T>(height1: T, height2: Option<T>) -> T
    where T: PartialOrd
{ max_width_for_opt_width(height1, height2) }

/// Returns a maximal size for the size and the optional size.
///
/// # Examples
/// ```
/// use lwltk::utils::max_size_for_opt_size;
/// use lwltk::Size;
///
/// assert_eq!(Size::new(2, 4), max_size_for_opt_size(Size::new(1, 4), Size::new(Some(2), Some(3))));
/// assert_eq!(Size::new(4, 5), max_size_for_opt_size(Size::new(4, 5), Size::new(None, None)));
/// ```
pub fn max_size_for_opt_size<T>(size1: Size<T>, size2: Size<Option<T>>) -> Size<T>
    where T: PartialOrd
{
    let width = max_width_for_opt_width(size1.width, size2.width);
    let height = max_height_for_opt_height(size1.height, size2.height);
    Size::new(width, height)
}

/// Returns a minimal width for the width and the optional width.
///
/// # Examples
/// ```
/// use lwltk::utils::min_width_for_opt_width;
///
/// assert_eq!(1, min_width_for_opt_width(1, Some(2)));
/// assert_eq!(2, min_width_for_opt_width(3, Some(2)));
/// assert_eq!(4, min_width_for_opt_width(4, None));
/// ```
pub fn min_width_for_opt_width<T>(width1: T, width2: Option<T>) -> T
    where T: PartialOrd
{
    match width2 {
        Some(width) => if width < width1 { width } else { width1 },
        None => width1,
    }
}

/// Returns a minimal height for the height and the optional height.
///
/// # Examples
/// ```
/// use lwltk::utils::min_height_for_opt_height;
///
/// assert_eq!(1, min_height_for_opt_height(1, Some(2)));
/// assert_eq!(2, min_height_for_opt_height(3, Some(2)));
/// assert_eq!(4, min_height_for_opt_height(4, None));
/// ```
pub fn min_height_for_opt_height<T>(height1: T, height2: Option<T>) -> T
    where T: PartialOrd
{ min_width_for_opt_width(height1, height2) }

/// Returns a minimal size for the size and the optional size.
///
/// # Examples
/// ```
/// use lwltk::utils::min_size_for_opt_size;
/// use lwltk::Size;
///
/// assert_eq!(Size::new(1, 3), min_size_for_opt_size(Size::new(1, 4), Size::new(Some(2), Some(3))));
/// assert_eq!(Size::new(4, 5), min_size_for_opt_size(Size::new(4, 5), Size::new(None, None)));
/// ```
pub fn min_size_for_opt_size<T>(size1: Size<T>, size2: Size<Option<T>>) -> Size<T>
    where T: PartialOrd
{
    let width = min_width_for_opt_width(size1.width, size2.width);
    let height = min_height_for_opt_height(size1.height, size2.height);
    Size::new(width, height)
}

/// Returns the optional width if the optional width isn't `None`, otherwise the width.
///
/// # Examples
/// ```
/// use lwltk::utils::width_for_opt_width;
///
/// assert_eq!(2, width_for_opt_width(1, Some(2)));
/// assert_eq!(4, width_for_opt_width(4, None));
/// ```
pub fn width_for_opt_width<T>(width1: T, width2: Option<T>) -> T
{
    match width2 {
        Some(width) => width,
        None => width1,
    }
}

/// Returns the optional height if the optional height isn't `None`, otherwise the height.
///
/// # Examples
/// ```
/// use lwltk::utils::height_for_opt_height;
///
/// assert_eq!(2, height_for_opt_height(1, Some(2)));
/// assert_eq!(4, height_for_opt_height(4, None));
/// ```
pub fn height_for_opt_height<T>(height1: T, height2: Option<T>) -> T
{ width_for_opt_width(height1, height2) }

/// Returns a size for the size and the optional size.
///
/// # Examples
/// ```
/// use lwltk::utils::size_for_opt_size;
/// use lwltk::Size;
///
/// assert_eq!(Size::new(3, 2), size_for_opt_size(Size::new(1, 2), Size::new(Some(3), None)));
/// assert_eq!(Size::new(1, 4), size_for_opt_size(Size::new(1, 2), Size::new(None, Some(4))));
/// ```
pub fn size_for_opt_size<T>(size1: Size<T>, size2: Size<Option<T>>) -> Size<T>
{
    let width = width_for_opt_width(size1.width, size2.width);
    let height = height_for_opt_height(size1.height, size2.height);
    Size::new(width, height)
}

/// Returns a X coordinate of for the horizontal alignment.
///
/// # Examples
/// ```
/// use lwltk::utils::x_for_h_align;
/// use lwltk::HAlign;
///
/// assert_eq!(1, x_for_h_align(4, 1, 6, HAlign::Left));
/// assert_eq!(2, x_for_h_align(4, 1, 6, HAlign::Center));
/// assert_eq!(3, x_for_h_align(4, 1, 6, HAlign::Right));
/// assert_eq!(1, x_for_h_align(6, 1, 6, HAlign::Fill));
/// ```
pub fn x_for_h_align<T>(width1: T, x2: T, width2: T, h_align: HAlign) -> T
    where T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32>
{
    match h_align {
        HAlign::Left => x2,
        HAlign::Center => x2 + (width2 - width1) / T::from(2),
        HAlign::Right => x2 + width2 - width1,
        HAlign::Fill => x2 + (width2 - width1) / T::from(2),
    }
}

/// Returns an Y coordinate for the vertical alignment.
///
/// # Examples
/// ```
/// use lwltk::utils::y_for_v_align;
/// use lwltk::VAlign;
///
/// assert_eq!(1, y_for_v_align(4, 1, 6, VAlign::Top));
/// assert_eq!(2, y_for_v_align(4, 1, 6, VAlign::Center));
/// assert_eq!(3, y_for_v_align(4, 1, 6, VAlign::Bottom));
/// assert_eq!(1, y_for_v_align(6, 1, 6, VAlign::Fill));
/// ```
pub fn y_for_v_align<T>(height1: T, y2: T, height2: T, v_align: VAlign) -> T
    where T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32>
{
    match v_align {
        VAlign::Top => y2,
        VAlign::Center => y2 + (height2 - height1) / T::from(2),
        VAlign::Bottom => y2 + height2 - height1,
        VAlign::Fill => y2 + (height2 - height1) / T::from(2),
    }
}

/// Returns a position for the horizontal alignment and the vertical alignment.
///
/// # Examples
/// ```
/// use lwltk::utils::pos_for_h_align_and_v_align;
/// use lwltk::HAlign;
/// use lwltk::Pos;
/// use lwltk::Rect;
/// use lwltk::Size;
/// use lwltk::VAlign;
///
/// assert_eq!(Pos::new(1, 2), pos_for_h_align_and_v_align(Size::new(4, 5), Rect::new(1, 2, 6, 7), HAlign::Left, VAlign::Top));
/// assert_eq!(Pos::new(2, 3), pos_for_h_align_and_v_align(Size::new(4, 5), Rect::new(1, 2, 6, 7), HAlign::Center, VAlign::Center));
/// assert_eq!(Pos::new(3, 4), pos_for_h_align_and_v_align(Size::new(4, 5), Rect::new(1, 2, 6, 7), HAlign::Right, VAlign::Bottom));
/// assert_eq!(Pos::new(1, 2), pos_for_h_align_and_v_align(Size::new(6, 7), Rect::new(1, 2, 6, 7), HAlign::Fill, VAlign::Fill));
/// ```
pub fn pos_for_h_align_and_v_align<T>(size1: Size<T>, rect2: Rect<T>, h_align: HAlign, v_align: VAlign) -> Pos<T>
    where T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32>
{
    let x = x_for_h_align(size1.width, rect2.x, rect2.width, h_align);
    let y = y_for_v_align(size1.height, rect2.y, rect2.height, v_align);
    Pos::new(x, y)
}

/// Returns a width for the horizontal alignment.
///
/// # Examples
/// ```
/// use lwltk::utils::width_for_h_align;
/// use lwltk::HAlign;
///
/// assert_eq!(4, width_for_h_align(4, Some(6), HAlign::Left));
/// assert_eq!(4, width_for_h_align(4, Some(6), HAlign::Center));
/// assert_eq!(4, width_for_h_align(4, Some(6), HAlign::Right));
/// assert_eq!(6, width_for_h_align(4, Some(6), HAlign::Fill));
/// ```
pub fn width_for_h_align<T>(width1: T, width2: Option<T>, h_align: HAlign) -> T
    where T: PartialOrd
{
    match h_align {
        HAlign::Fill => width_for_opt_width(width1, width2),
        _ => min_width_for_opt_width(width1, width2),
    }
}

/// Returns a height for the vertical alignment.
///
/// # Examples
/// ```
/// use lwltk::utils::height_for_v_align;
/// use lwltk::VAlign;
///
/// assert_eq!(4, height_for_v_align(4, Some(6), VAlign::Top));
/// assert_eq!(4, height_for_v_align(4, Some(6), VAlign::Center));
/// assert_eq!(4, height_for_v_align(4, Some(6), VAlign::Bottom));
/// assert_eq!(6, height_for_v_align(4, Some(6), VAlign::Fill));
/// ```
pub fn height_for_v_align<T>(height1: T, height2: Option<T>, v_align: VAlign) -> T
    where T: PartialOrd
{
    match v_align {
        VAlign::Fill => height_for_opt_height(height1, height2),
        _ => min_height_for_opt_height(height1, height2),
    }
}

/// Returns a size for the horizontal alignment and the vertical alignment.
///
/// # Examples
/// ```
/// use lwltk::utils::size_for_h_align_and_v_align;
/// use lwltk::HAlign;
/// use lwltk::Size;
/// use lwltk::VAlign;
///
/// assert_eq!(Size::new(4, 5), size_for_h_align_and_v_align(Size::new(4, 5), Size::new(Some(6), Some(7)), HAlign::Left, VAlign::Top));
/// assert_eq!(Size::new(4, 5), size_for_h_align_and_v_align(Size::new(4, 5), Size::new(Some(6), Some(7)), HAlign::Center, VAlign::Center));
/// assert_eq!(Size::new(4, 5), size_for_h_align_and_v_align(Size::new(4, 5), Size::new(Some(6), Some(7)), HAlign::Right, VAlign::Bottom));
/// assert_eq!(Size::new(6, 7), size_for_h_align_and_v_align(Size::new(4, 5), Size::new(Some(6), Some(7)), HAlign::Fill, VAlign::Fill));
/// ```
pub fn size_for_h_align_and_v_align<T>(size1: Size<T>, size2: Size<Option<T>>, h_align: HAlign, v_align: VAlign) -> Size<T>
    where T: PartialOrd
{
    let width = width_for_h_align(size1.width, size2.width, h_align);
    let height = height_for_v_align(size1.height, size2.height, v_align);
    Size::new(width, height)
}

/// Returns a maximal optional width for two optional widths.
///
/// # Examples
/// ```
/// use lwltk::utils::max_opt_width_for_opt_width;
///
/// assert_eq!(Some(2), max_opt_width_for_opt_width(Some(1), Some(2)));
/// assert_eq!(Some(3), max_opt_width_for_opt_width(Some(3), Some(2)));
/// assert_eq!(Some(4), max_opt_width_for_opt_width(Some(4), None));
/// assert_eq!(Some(5), max_opt_width_for_opt_width(None, Some(5)));
/// let none: Option<i32> = None;
/// assert_eq!(none, max_opt_width_for_opt_width(none, none));
/// ```
pub fn max_opt_width_for_opt_width<T>(width1: Option<T>, width2: Option<T>) -> Option<T>
    where T: PartialOrd
{
    match (width1, width2) {
        (Some(width1), Some(width2)) => if width2 > width1 { Some(width2) } else { Some(width1) },
        (Some(width1), None) => Some(width1),
        (None, Some(width2)) => Some(width2),
        (None, None) => None,
    }
}

/// Returns a maximal optional height for two optional heights.
///
/// # Examples
/// ```
/// use lwltk::utils::max_opt_height_for_opt_height;
///
/// assert_eq!(Some(2), max_opt_height_for_opt_height(Some(1), Some(2)));
/// assert_eq!(Some(3), max_opt_height_for_opt_height(Some(3), Some(2)));
/// assert_eq!(Some(4), max_opt_height_for_opt_height(Some(4), None));
/// assert_eq!(Some(5), max_opt_height_for_opt_height(None, Some(5)));
/// let none: Option<i32> = None;
/// assert_eq!(none, max_opt_height_for_opt_height(none, none));
/// ```
pub fn max_opt_height_for_opt_height<T>(height1: Option<T>, height2: Option<T>) -> Option<T>
    where T: PartialOrd
{ max_opt_width_for_opt_width(height1, height2) }

/// Returns a maximal optional size for two optional sizes.
///
/// # Examples
/// ```
/// use lwltk::utils::max_opt_size_for_opt_size;
/// use lwltk::Size;
///
/// assert_eq!(Size::new(Some(2), Some(4)), max_opt_size_for_opt_size(Size::new(Some(1), Some(4)), Size::new(Some(2), Some(3))));
/// assert_eq!(Size::new(Some(4), Some(5)), max_opt_size_for_opt_size(Size::new(Some(4), Some(5)), Size::new(None, None)));
/// assert_eq!(Size::new(Some(6), Some(7)), max_opt_size_for_opt_size(Size::new(None, None), Size::new(Some(6), Some(7))));
/// let none: Option<i32> = None;
/// assert_eq!(Size::new(none, none), max_opt_size_for_opt_size(Size::new(none, none), Size::new(none, none)));
/// ```
pub fn max_opt_size_for_opt_size<T>(size1: Size<Option<T>>, size2: Size<Option<T>>) -> Size<Option<T>>
    where T: PartialOrd
{
    let width = max_opt_width_for_opt_width(size1.width, size2.width);
    let height = max_opt_height_for_opt_height(size1.height, size2.height);
    Size::new(width, height)
}

/// Returns a minimal optional width for two optional widths.
///
/// # Examples
/// ```
/// use lwltk::utils::min_opt_width_for_opt_width;
///
/// assert_eq!(Some(1), min_opt_width_for_opt_width(Some(1), Some(2)));
/// assert_eq!(Some(2), min_opt_width_for_opt_width(Some(3), Some(2)));
/// assert_eq!(Some(4), min_opt_width_for_opt_width(Some(4), None));
/// assert_eq!(Some(5), min_opt_width_for_opt_width(None, Some(5)));
/// let none: Option<i32> = None;
/// assert_eq!(none, min_opt_width_for_opt_width(none, none));
/// ```
pub fn min_opt_width_for_opt_width<T>(width1: Option<T>, width2: Option<T>) -> Option<T>
    where T: PartialOrd
{
    match (width1, width2) {
        (Some(width1), Some(width2)) => if width2 < width1 { Some(width2) } else { Some(width1) },
        (Some(width1), None) => Some(width1),
        (None, Some(width2)) => Some(width2),
        (None, None) => None,
    }
}

/// Returns a minimal optional height for two optional heights.
///
/// # Examples
/// ```
/// use lwltk::utils::min_opt_height_for_opt_height;
///
/// assert_eq!(Some(1), min_opt_height_for_opt_height(Some(1), Some(2)));
/// assert_eq!(Some(2), min_opt_height_for_opt_height(Some(3), Some(2)));
/// assert_eq!(Some(4), min_opt_height_for_opt_height(Some(4), None));
/// assert_eq!(Some(5), min_opt_height_for_opt_height(None, Some(5)));
/// let none: Option<i32> = None;
/// assert_eq!(none, min_opt_height_for_opt_height(none, none));
/// ```
pub fn min_opt_height_for_opt_height<T>(height1: Option<T>, height2: Option<T>) -> Option<T>
    where T: PartialOrd
{ min_opt_width_for_opt_width(height1, height2) }

/// Returns a minimal optional size for two optional sizes.
///
/// # Examples
/// ```
/// use lwltk::utils::min_opt_size_for_opt_size;
/// use lwltk::Size;
///
/// assert_eq!(Size::new(Some(1), Some(3)), min_opt_size_for_opt_size(Size::new(Some(1), Some(4)), Size::new(Some(2), Some(3))));
/// assert_eq!(Size::new(Some(4), Some(5)), min_opt_size_for_opt_size(Size::new(Some(4), Some(5)), Size::new(None, None)));
/// assert_eq!(Size::new(Some(6), Some(7)), min_opt_size_for_opt_size(Size::new(None, None), Size::new(Some(6), Some(7))));
/// let none: Option<i32> = None;
/// assert_eq!(Size::new(none, none), min_opt_size_for_opt_size(Size::new(none, none), Size::new(none, none)));
/// ```
pub fn min_opt_size_for_opt_size<T>(size1: Size<Option<T>>, size2: Size<Option<T>>) -> Size<Option<T>>
    where T: PartialOrd
{
    let width = min_opt_width_for_opt_width(size1.width, size2.width);
    let height = min_opt_height_for_opt_height(size1.height, size2.height);
    Size::new(width, height)
}

/// Returns the second optional width if the second width isn't `None`, otherwise the first
/// optional width.
///
/// # Examples
/// ```
/// use lwltk::utils::opt_width_for_opt_width;
///
/// assert_eq!(Some(2), opt_width_for_opt_width(Some(1), Some(2)));
/// assert_eq!(Some(4), opt_width_for_opt_width(Some(4), None));
/// assert_eq!(Some(5), opt_width_for_opt_width(None, Some(5)));
/// let none: Option<i32> = None;
/// assert_eq!(none, opt_width_for_opt_width(none, none));
/// ```
pub fn opt_width_for_opt_width<T>(width1: Option<T>, width2: Option<T>) -> Option<T>
{
    match (width1, width2) {
        (_, Some(width2)) => Some(width2),
        (Some(width1), None) => Some(width1),
        (None, None) => None,
    }
}

/// Returns the second optional height if the second height isn't `None`, otherwise the first
/// optional height.
///
/// # Examples
/// ```
/// use lwltk::utils::opt_height_for_opt_height;
///
/// assert_eq!(Some(2), opt_height_for_opt_height(Some(1), Some(2)));
/// assert_eq!(Some(4), opt_height_for_opt_height(Some(4), None));
/// assert_eq!(Some(5), opt_height_for_opt_height(None, Some(5)));
/// let none: Option<i32> = None;
/// assert_eq!(none, opt_height_for_opt_height(none, none));
/// ```
pub fn opt_height_for_opt_height<T>(height1: Option<T>, height2: Option<T>) -> Option<T>
{ opt_width_for_opt_width(height1, height2) }

/// Returns an optional size for two optional sizes.
///
/// # Examples
/// ```
/// use lwltk::utils::opt_size_for_opt_size;
/// use lwltk::Size;
///
/// assert_eq!(Size::new(Some(3), Some(2)), opt_size_for_opt_size(Size::new(Some(1), Some(2)), Size::new(Some(3), None)));
/// assert_eq!(Size::new(Some(1), Some(4)), opt_size_for_opt_size(Size::new(Some(1), Some(2)), Size::new(None, Some(4))));
/// assert_eq!(Size::new(Some(5), Some(6)), opt_size_for_opt_size(Size::new(Some(1), None), Size::new(Some(5), Some(6))));
/// assert_eq!(Size::new(Some(7), Some(8)), opt_size_for_opt_size(Size::new(None, Some(2)), Size::new(Some(7), Some(8))));
/// let none: Option<i32> = None;
/// assert_eq!(Size::new(none, none), opt_size_for_opt_size(Size::new(none, none), Size::new(none, none)));
/// ```
pub fn opt_size_for_opt_size<T>(size1: Size<Option<T>>, size2: Size<Option<T>>) -> Size<Option<T>>
{
    let width = opt_width_for_opt_width(size1.width, size2.width);
    let height = opt_height_for_opt_height(size1.height, size2.height);
    Size::new(width, height)
}

/// Creates a position that has swapped coordinates for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_pos;
/// use lwltk::Orient;
/// use lwltk::Pos;
///
/// let pos1 = orient_pos(1, 2, Orient::Horizontal);
/// let pos2 = orient_pos(3, 4, Orient::Vertical);
/// assert_eq!(Pos::new(1, 2), pos1);
/// assert_eq!(Pos::new(4, 3), pos2);
/// ```
pub fn orient_pos<T>(x: T, y: T, orient: Orient) -> Pos<T>
{
    match orient {
        Orient::Horizontal => Pos::new(x, y),
        Orient::Vertical => Pos::new(y, x),
    }
}

/// Returns the X coordinate of the position that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_pos_x;
/// use lwltk::Orient;
/// use lwltk::Pos;
///
/// let pos = Pos::new(1, 2);
/// assert_eq!(1, orient_pos_x(pos, Orient::Horizontal));
/// assert_eq!(2, orient_pos_x(pos, Orient::Vertical));
/// ```
pub fn orient_pos_x<T>(pos: Pos<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => pos.x,
        Orient::Vertical => pos.y,
    }
}

/// Returns the Y coordinate of the position that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_pos_y;
/// use lwltk::Orient;
/// use lwltk::Pos;
///
/// let pos = Pos::new(1, 2);
/// assert_eq!(2, orient_pos_y(pos, Orient::Horizontal));
/// assert_eq!(1, orient_pos_y(pos, Orient::Vertical));
/// ```
pub fn orient_pos_y<T>(pos: Pos<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => pos.y,
        Orient::Vertical => pos.x,
    }
}

/// Sets the X coordinate of the position that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_pos_x;
/// use lwltk::Orient;
/// use lwltk::Pos;
///
/// let mut pos = Pos::new(1, 2);
/// set_orient_pos_x(&mut pos, 3, Orient::Horizontal);
/// assert_eq!(Pos::new(3, 2), pos);
/// set_orient_pos_x(&mut pos, 4, Orient::Vertical);
/// assert_eq!(Pos::new(3, 4), pos);
/// ```
pub fn set_orient_pos_x<T>(pos: &mut Pos<T>, x: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => pos.x = x,
        Orient::Vertical => pos.y = x,
    }
}

/// Sets the Y coordinate of the position that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_pos_y;
/// use lwltk::Orient;
/// use lwltk::Pos;
///
/// let mut pos = Pos::new(1, 2);
/// set_orient_pos_y(&mut pos, 3, Orient::Horizontal);
/// assert_eq!(Pos::new(1, 3), pos);
/// set_orient_pos_y(&mut pos, 4, Orient::Vertical);
/// assert_eq!(Pos::new(4, 3), pos);
/// ```
pub fn set_orient_pos_y<T>(pos: &mut Pos<T>, y: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => pos.y = y,
        Orient::Vertical => pos.x = y,
    }
}

/// Creates a size that has swapped values for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_size;
/// use lwltk::Orient;
/// use lwltk::Size;
///
/// let size1 = orient_size(1, 2, Orient::Horizontal);
/// let size2 = orient_size(3, 4, Orient::Vertical);
/// assert_eq!(Size::new(1, 2), size1);
/// assert_eq!(Size::new(4, 3), size2);
/// ```
pub fn orient_size<T>(width: T, height: T, orient: Orient) -> Size<T>
{
    match orient {
        Orient::Horizontal => Size::new(width, height),
        Orient::Vertical => Size::new(height, width),
    }
}

/// Returns the width of the size that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_size_width;
/// use lwltk::Orient;
/// use lwltk::Size;
///
/// let size = Size::new(1, 2);
/// assert_eq!(1, orient_size_width(size, Orient::Horizontal));
/// assert_eq!(2, orient_size_width(size, Orient::Vertical));
/// ```
pub fn orient_size_width<T>(size: Size<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => size.width,
        Orient::Vertical => size.height,
    }
}

/// Returns the height of the size that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_size_height;
/// use lwltk::Orient;
/// use lwltk::Size;
///
/// let size = Size::new(1, 2);
/// assert_eq!(2, orient_size_height(size, Orient::Horizontal));
/// assert_eq!(1, orient_size_height(size, Orient::Vertical));
/// ```
pub fn orient_size_height<T>(size: Size<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => size.height,
        Orient::Vertical => size.width,
    }
}

/// Sets the width of the size that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_size_width;
/// use lwltk::Orient;
/// use lwltk::Size;
///
/// let mut size = Size::new(1, 2);
/// set_orient_size_width(&mut size, 3, Orient::Horizontal);
/// assert_eq!(Size::new(3, 2), size);
/// set_orient_size_width(&mut size, 4, Orient::Vertical);
/// assert_eq!(Size::new(3, 4), size);
/// ```
pub fn set_orient_size_width<T>(size: &mut Size<T>, width: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => size.width = width,
        Orient::Vertical => size.height = width,
    }
}

/// Sets the height of the size that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_size_height;
/// use lwltk::Orient;
/// use lwltk::Size;
///
/// let mut size = Size::new(1, 2);
/// set_orient_size_height(&mut size, 3, Orient::Horizontal);
/// assert_eq!(Size::new(1, 3), size);
/// set_orient_size_height(&mut size, 4, Orient::Vertical);
/// assert_eq!(Size::new(4, 3), size);
/// ```
pub fn set_orient_size_height<T>(size: &mut Size<T>, height: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => size.height = height,
        Orient::Vertical => size.width = height,
    }
}

/// Creates a rectangle that has swapped coordinates and swapped values for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_rect;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let rect1 = orient_rect(1, 2, 3, 4, Orient::Horizontal);
/// let rect2 = orient_rect(5, 6, 7, 8, Orient::Vertical);
/// assert_eq!(Rect::new(1, 2, 3, 4), rect1);
/// assert_eq!(Rect::new(6, 5, 8, 7), rect2);
/// ```
pub fn orient_rect<T>(x: T, y: T, width: T, height: T, orient: Orient) -> Rect<T>
{
    match orient {
        Orient::Horizontal => Rect::new(x, y, width, height),
        Orient::Vertical => Rect::new(y, x, height, width),
    }
}

/// Returns the X coordinate of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_rect_x;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let rect = Rect::new(1, 2, 3, 4);
/// assert_eq!(1, orient_rect_x(rect, Orient::Horizontal));
/// assert_eq!(2, orient_rect_x(rect, Orient::Vertical));
/// ```
pub fn orient_rect_x<T>(rect: Rect<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => rect.x,
        Orient::Vertical => rect.y,
    }
}

/// Returns the Y coordinate of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_rect_y;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let rect = Rect::new(1, 2, 3, 4);
/// assert_eq!(2, orient_rect_y(rect, Orient::Horizontal));
/// assert_eq!(1, orient_rect_y(rect, Orient::Vertical));
/// ```
pub fn orient_rect_y<T>(rect: Rect<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => rect.y,
        Orient::Vertical => rect.x,
    }
}

/// Returns the width of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_rect_width;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let rect = Rect::new(1, 2, 3, 4);
/// assert_eq!(3, orient_rect_width(rect, Orient::Horizontal));
/// assert_eq!(4, orient_rect_width(rect, Orient::Vertical));
/// ```
pub fn orient_rect_width<T>(rect: Rect<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => rect.width,
        Orient::Vertical => rect.height,
    }
}

/// Returns the height of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::orient_rect_height;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let rect = Rect::new(1, 2, 3, 4);
/// assert_eq!(4, orient_rect_height(rect, Orient::Horizontal));
/// assert_eq!(3, orient_rect_height(rect, Orient::Vertical));
/// ```
pub fn orient_rect_height<T>(rect: Rect<T>, orient: Orient) -> T
{
    match orient {
        Orient::Horizontal => rect.height,
        Orient::Vertical => rect.width,
    }
}

/// Sets the X coordinate of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_rect_x;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let mut rect = Rect::new(1, 2, 3, 4);
/// set_orient_rect_x(&mut rect, 5, Orient::Horizontal);
/// assert_eq!(Rect::new(5, 2, 3, 4), rect);
/// set_orient_rect_x(&mut rect, 6, Orient::Vertical);
/// assert_eq!(Rect::new(5, 6, 3, 4), rect);
/// ```
pub fn set_orient_rect_x<T>(rect: &mut Rect<T>, x: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => rect.x = x,
        Orient::Vertical => rect.y = x,
    }
}

/// Sets the Y coordinate of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_rect_y;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let mut rect = Rect::new(1, 2, 3, 4);
/// set_orient_rect_y(&mut rect, 5, Orient::Horizontal);
/// assert_eq!(Rect::new(1, 5, 3, 4), rect);
/// set_orient_rect_y(&mut rect, 6, Orient::Vertical);
/// assert_eq!(Rect::new(6, 5, 3, 4), rect);
/// ```
pub fn set_orient_rect_y<T>(rect: &mut Rect<T>, y: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => rect.y = y,
        Orient::Vertical => rect.x = y,
    }
}

/// Sets the width of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_rect_width;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let mut rect = Rect::new(1, 2, 3, 4);
/// set_orient_rect_width(&mut rect, 7, Orient::Horizontal);
/// assert_eq!(Rect::new(1, 2, 7, 4), rect);
/// set_orient_rect_width(&mut rect, 8, Orient::Vertical);
/// assert_eq!(Rect::new(1, 2, 7, 8), rect);
/// ```
pub fn set_orient_rect_width<T>(rect: &mut Rect<T>, width: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => rect.width = width,
        Orient::Vertical => rect.height = width,
    }
}

/// Sets the height of the rectangle that is swapped for the orientation.
///
/// # Examples
/// ```
/// use lwltk::utils::set_orient_rect_height;
/// use lwltk::Orient;
/// use lwltk::Rect;
///
/// let mut rect = Rect::new(1, 2, 3, 4);
/// set_orient_rect_height(&mut rect, 7, Orient::Horizontal);
/// assert_eq!(Rect::new(1, 2, 3, 7), rect);
/// set_orient_rect_height(&mut rect, 8, Orient::Vertical);
/// assert_eq!(Rect::new(1, 2, 8, 7), rect);
/// ```
pub fn set_orient_rect_height<T>(rect: &mut Rect<T>, height: T, orient: Orient)
{
    match orient {
        Orient::Horizontal => rect.height = height,
        Orient::Vertical => rect.width = height,
    }
}
