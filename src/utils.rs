//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
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

pub fn create_dummy_cairo_surface() -> Result<ImageSurface, CairoError>
{ ImageSurface::create(Format::ARgb32, 1, 1) }

pub fn with_cairo_context<T, F>(cairo_surface: &ImageSurface, f: F) -> Result<T, CairoError>
    where F: FnOnce(&CairoContext) -> Result<T, CairoError>
{
    match CairoContext::new(&cairo_surface) {
        Ok(cairo_context) => f(&cairo_context),
        Err(err) => Err(err),
    }
}

pub fn with_dummy_cairo_context<T, F>(f: F) -> Result<T, CairoError>
    where F: FnOnce(&CairoContext) -> Result<T, CairoError>
{
    match create_dummy_cairo_surface() {
        Ok(cairo_surface) => with_cairo_context(&cairo_surface, f),
        Err(err) => Err(err),
    }
}

pub fn default_widget_on_for_client_pointer(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::PointerEnter(pos)) => {
            client_context.set_cursor(widget.cursor(*pos, queue_context.has_wait_cursor()));
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerLeave) => {
            match queue_context.motion_call_on_path(CallOnId::Pointer) {
                Some(CallOnPath::Widget(abs_widget_path)) => {
                    let tmp_abs_widget_path = abs_widget_path.clone();
                    let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                    queue_context.push_callback(move |_, window_context, queue_context| {
                            if queue_context.decrease_active_count(&tmp_call_on_path) {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                            }
                            Some(())
                    });
                },
                _ => (),
            }
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.decrease_active_count(&current_call_on_path) {
                widget.set_state(WidgetState::Hover);
            }
            queue_context.unset_motion_call_on_path(CallOnId::Pointer);
            queue_context.unset_motion_resize_edges(CallOnId::Pointer);
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            queue_context.unset_pressed_instant(CallOnId::Pointer);
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
                                if queue_context.decrease_active_count(&tmp_call_on_path) {
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
                if queue_context.increase_active_count(&current_call_on_path) {
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
            queue_context.set_long_click(false);
            if client_context.post_button_release_call_on_path().is_none() {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.increase_active_count(&current_call_on_path) {
                    widget.set_state(WidgetState::Active);
                }
                queue_context.set_double_click(false);
            }
            queue_context.push_callback(move |_, window_context, _| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_idx = window_context.focused_window_index()?;
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(focused_window_idx)?;
                    window.set_focused_rel_widget_path(window.point_focusable(current_pos));
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
                }
                client_context.send_after_button_release(queue_context.current_call_on_path()?.clone());
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.decrease_active_count(&tmp_call_on_path) {
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
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Pressed)) => {
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            queue_context.push_callback(move |_, window_context, queue_context| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_idx = window_context.focused_window_index()?;
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(focused_window_idx)?;
                    window.set_focused_rel_widget_path(window.point_focusable(current_pos));
                    queue_context.event_queue_mut().push(EventPair::new(current_call_on_path.clone(), Event::PopupClick));
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Released)) => Some(Some(None)),
        Event::Client(ClientEvent::PostButtonRelease) => {
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.decrease_active_count(&current_call_on_path) {
                widget.set_state(WidgetState::Hover);
            }
            if queue_context.has_double_click() {
                queue_context.push_event(Event::DoubleClick);
            } else if queue_context.has_long_click() {
                queue_context.push_event(Event::LongClick);
            } else {
                queue_context.push_event(Event::Click);
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

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
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()));
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c));
            }
            if widget.is_clickable() {
                if keys.iter().any(|k| *k == VKey::Return || *k == VKey::Space) {
                    let current_call_on_path = queue_context.current_call_on_path()?.clone();
                    if queue_context.increase_active_count(&current_call_on_path) {
                        widget.set_state(WidgetState::Active);
                    }
                }
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, _, ClientState::Released)) => {
            if widget.is_clickable() {
                if keys.iter().any(|k| *k == VKey::Return || *k == VKey::Space) {
                    let current_call_on_path = queue_context.current_call_on_path()?.clone();
                    if queue_context.decrease_active_count(&current_call_on_path) {
                        if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                            widget.set_state(WidgetState::Hover);
                        } else {
                            widget.set_state(WidgetState::None);
                        }
                    }
                    queue_context.push_event(Event::Click);
                }
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardModifiers(_)) => Some(Some(None)),
        Event::Client(ClientEvent::RepeatedKey(keys, s)) => {
            for key in keys {
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()));
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c));
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

pub fn default_widget_on_for_client_touch(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::TouchDown(_, id, _)) => {
            queue_context.set_motion_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.unset_motion_resize_edges(CallOnId::Touch(*id));
            queue_context.set_pressed_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.set_pressed_instant(CallOnId::Touch(*id), Instant::now());
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            if queue_context.increase_active_count(&current_call_on_path) {
                widget.set_state(WidgetState::Active);
            }
            queue_context.push_callback(move |_, window_context, _| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_idx = window_context.focused_window_index()?;
                    let current_pos = window_context.current_pos()?;
                    let window = window_context.dyn_window_mut(focused_window_idx)?;
                    window.set_focused_rel_widget_path(window.point_focusable(current_pos));
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchUp(_, id)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Touch(*id));
            if pressed_call_on_path == queue_context.current_call_on_path() {
                let current_call_on_path = queue_context.current_call_on_path()?.clone();
                if queue_context.decrease_active_count(&current_call_on_path) {
                    if queue_context.motion_call_on_path(CallOnId::Pointer) == queue_context.current_call_on_path() {
                        widget.set_state(WidgetState::Hover);
                    } else {
                        widget.set_state(WidgetState::None);
                    }
                }
                let duration = Duration::from_millis(client_context.long_click_delay());
                if queue_context.pressed_instant(CallOnId::Pointer)?.elapsed() >= duration {
                    queue_context.push_event(Event::LongClick);
                } else {
                    queue_context.push_event(Event::Click);
                }
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.decrease_active_count(&tmp_call_on_path) {
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
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchMotion(_, id, _)) => {
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Touch(*id));
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.decrease_active_count(&tmp_call_on_path) {
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
                if queue_context.increase_active_count(&current_call_on_path) {
                    widget.set_state(WidgetState::Active);
                }
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

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

#[allow(unused_variables)]
pub fn default_widget_on_for_key_and_char(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Key(_, _) | Event::Char(_) => Some(Some(Some(event.clone()))),
        _ => Some(None),
    }
}

#[allow(unused_variables)]
pub fn default_widget_on_for_window_events(widget: &mut dyn Widget, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Menu | Event::Close | Event::Maximize=> Some(Some(Some(event.clone()))),
        _ => Some(None),
    }
}

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
        Some(ClientResize::Left);
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

pub fn cursor_for_client_resize_and_resizable(edges: Option<ClientResize>, is_resizable: bool) -> Cursor
{
    if is_resizable {
        match edges {
            Some(ClientResize::Top) => Cursor::TopSide,
            Some(ClientResize::Bottom) => Cursor::BottomSide,
            Some(ClientResize::Left) => Cursor::LeftSide,
            Some(ClientResize::Right) => Cursor::RightSide,
            Some(ClientResize::TopLeft) => Cursor::TopLeftCorner,
            Some(ClientResize::TopRight) => Cursor::TopRightCorner,
            Some(ClientResize::BottomLeft) => Cursor::BottomLeftCorner,
            Some(ClientResize::BottomRight) => Cursor::BottomRightCorner,
            _ => Cursor::Default,
        }
    } else {
        Cursor::Default
    }
}

#[allow(unused_variables)]
pub fn default_window_on_for_client_shell_surface(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::ShellSurfaceConfigure(_, size)) => {
            window.set_preferred_size(Size::new(Some(size.width), Some(size.height)));
            Some(Some(None))
        },
        Event::Client(ClientEvent::ShellSurfacePopupDone) => Some(Some(None)),
        _ => Some(None),
    }
}

pub fn default_window_on_for_client_pointer(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::PointerEnter(pos)) => {
            let resize_edges = client_resize_for_pos(*pos, window.size(), window.edges(), window.corners());
            client_context.set_cursor(cursor_for_client_resize_and_resizable(resize_edges, window.is_resizable()));
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
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
                            if queue_context.decrease_active_count(&tmp_call_on_path) {
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
                                if queue_context.decrease_active_count(&tmp_call_on_path) {
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
                    queue_context.set_long_click(false);
                    if client_context.post_button_release_call_on_path().is_none() {
                        queue_context.set_double_click(false);
                    }
                },
            }
            queue_context.push_callback(move |_, window_context, _| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_idx = window_context.focused_window_index()?;
                    let window = window_context.dyn_window_mut(focused_window_idx)?;
                    window.set_focused_rel_widget_path(None);
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
                    }
                    client_context.send_after_button_release(queue_context.current_call_on_path()?.clone());
                } else {
                    match pressed_call_on_path {
                        Some(CallOnPath::Widget(abs_widget_path)) => {
                            let tmp_abs_widget_path = abs_widget_path.clone();
                            let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                            queue_context.push_callback(move |_, window_context, queue_context| {
                                    if queue_context.decrease_active_count(&tmp_call_on_path) {
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
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Pressed)) => {
            let current_call_on_path = queue_context.current_call_on_path()?.clone();
            let are_motion_resize_edges = queue_context.motion_resize_edges(CallOnId::Pointer).is_some();
            queue_context.push_callback(move |_, window_context, queue_context| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_idx = window_context.focused_window_index()?;
                    let window = window_context.dyn_window_mut(focused_window_idx)?;
                    window.set_focused_rel_widget_path(None);
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
                queue_context.push_event(Event::DoubleClick);
            } else if queue_context.has_long_click() {
                queue_context.push_event(Event::LongClick);
            } else {
                queue_context.push_event(Event::Click);
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

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
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()));
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c));
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, _, ClientState::Released)) => Some(Some(None)),
        Event::Client(ClientEvent::KeyboardModifiers(_)) => Some(Some(None)),
        Event::Client(ClientEvent::RepeatedKey(keys, s)) => {
            for key in keys {
                queue_context.push_event(Event::Key(*key, client_context.key_modifiers()));
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c));
            }
            Some(Some(None))
        },
        _ => Some(None),
    }
}

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
                    queue_context.push_callback(move |_, window_context, _| {
                            window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                            let focused_window_idx = window_context.focused_window_index()?;
                            let current_pos = window_context.current_pos()?;
                            let window = window_context.dyn_window_mut(focused_window_idx)?;
                            window.set_focused_rel_widget_path(window.point_focusable(current_pos));
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
                    if queue_context.pressed_instant(CallOnId::Pointer)?.elapsed() >= duration {
                        queue_context.push_event(Event::LongClick);
                    } else {
                        queue_context.push_event(Event::Click);
                    }
                } else {
                    match pressed_call_on_path {
                        Some(CallOnPath::Widget(abs_widget_path)) => {
                            let tmp_abs_widget_path = abs_widget_path.clone();
                            let tmp_call_on_path = CallOnPath::Widget(abs_widget_path.clone());
                            queue_context.push_callback(move |_, window_context, queue_context| {
                                    if queue_context.decrease_active_count(&tmp_call_on_path) {
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
                        queue_context.push_callback(move |_, window_context, queue_context| {
                                if queue_context.decrease_active_count(&tmp_call_on_path) {
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

#[allow(unused_variables)]
pub fn default_window_on_for_clicks(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Click | Event::DoubleClick | Event::LongClick | Event::PopupClick => Some(Some(None)),
        _ => Some(None),
    }
}

#[allow(unused_variables)]
pub fn default_window_on_for_key(window: &mut dyn Window, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Key(key, modifiers) => {
            if *modifiers == KeyModifiers::EMPTY {
                match *key {
                    VKey::Escape | VKey::PageUp => {
                        window.up_focused_widget()?;
                        Some(Some(None))
                    },
                    VKey::PageDown => {
                        window.down_focused_widget()?;
                        Some(Some(None))
                    },
                    VKey::Up | VKey::Left => {
                        window.prev_focused_widget()?;
                        Some(Some(None))
                    },
                    VKey::Down | VKey::Right => {
                        window.next_focused_widget()?;
                        Some(Some(None))
                    },
                    _ => Some(None),
                }
            } else if *modifiers == KeyModifiers::ALT {
                match *key {
                    VKey::F4 => {
                        queue_context.push_event(Event::Close);
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
    } else {
        Some(None)
    }
}
