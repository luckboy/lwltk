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
use crate::event_queue::*;
use crate::events::*;
use crate::keys::*;
use crate::queue_context::*;
use crate::types::*;
use crate::widget::*;

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
            queue_context.set_current_pointer_pos(Some(*pos));
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerLeave) => {
            queue_context.set_current_pointer_pos(None);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerMotion(_, pos)) => {
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Pointer);
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        queue_context.push_callback(move |_, window_context, _| {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());            
            queue_context.set_current_pointer_pos(Some(*pos));
            widget.set_state(WidgetState::Hover);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Pressed)) => {
            queue_context.set_pressed_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());
            queue_context.set_pressed_instant(CallOnId::Pointer, Instant::now());
            queue_context.set_long_click(false);
            if client_context.post_button_release_call_on_path().is_none() {
                widget.set_state(WidgetState::Active);
                queue_context.set_double_click(false);
            }
            queue_context.push_callback(move |_, window_context, queue_context| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_index = window_context.focused_window_index()?;
                    let current_pos = queue_context.current_pointer_pos()?;
                    let window = window_context.dyn_window_mut(focused_window_index)?;
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
                        queue_context.push_callback(move |_, window_context, _| {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.unset_pressed_call_on_path(CallOnId::Pointer);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Pressed)) => {
            queue_context.set_pressed_call_on_path_for_popup_click(Some(queue_context.current_call_on_path()?.clone()));
            widget.set_state(WidgetState::Active);
            queue_context.push_callback(move |_, window_context, queue_context| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_index = window_context.focused_window_index()?;
                    let current_pos = queue_context.current_pointer_pos()?;
                    let window = window_context.dyn_window_mut(focused_window_index)?;
                    window.set_focused_rel_widget_path(window.point_focusable(current_pos));
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::PointerButton(_, ClientButton::Right, ClientState::Released)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path_for_popup_click();
            if pressed_call_on_path == queue_context.current_call_on_path() {
                queue_context.push_event(Event::PopupClick);
                widget.set_state(WidgetState::Hover);
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        queue_context.push_callback(move |_, window_context, _| {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_pressed_call_on_path_for_popup_click(None);
            Some(Some(None))
        },
        Event::Client(ClientEvent::PostButtonRelease) => {
            if queue_context.has_double_click() {
                queue_context.push_event(Event::DoubleClick);
            } else if queue_context.has_long_click() {
                queue_context.push_event(Event::LongClick);
            } else {
                queue_context.push_event(Event::Click);
            }
            widget.set_state(WidgetState::Hover);
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
            if widget.is_clickable() {
                if keys.iter().any(|k| *k == VKey::Return || *k == VKey::Space) {
                    widget.set_state(WidgetState::Active);
                }
            }
            for c in s.chars() {
                queue_context.push_event(Event::Char(c));
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::KeyboardKey(_, keys, _, ClientState::Released)) => {
            if widget.is_clickable() {
                if keys.iter().any(|k| *k == VKey::Return || *k == VKey::Space) {
                    queue_context.push_event(Event::Click);
                    widget.set_state(WidgetState::None);
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

pub fn default_widget_on_for_client_touch(widget: &mut dyn Widget, _client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Option<Event>>>
{
    match event {
        Event::Client(ClientEvent::TouchDown(_, id, pos)) => {
            queue_context.set_pressed_call_on_path(CallOnId::Touch(*id), queue_context.current_call_on_path()?.clone());
            queue_context.set_pressed_instant(CallOnId::Touch(*id), Instant::now());
            widget.set_state(WidgetState::Active);
            let tmp_pos = *pos;
            queue_context.push_callback(move |_, window_context, _| {
                    window_context.set_focused_window_index(Some(window_context.current_window_index()?));
                    let focused_window_index = window_context.focused_window_index()?;
                    let window = window_context.dyn_window_mut(focused_window_index)?;
                    window.set_focused_rel_widget_path(window.point_focusable(tmp_pos));
                    Some(())
            });
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchUp(_, id)) => {
            let pressed_call_on_path = queue_context.pressed_call_on_path(CallOnId::Touch(*id));
            if pressed_call_on_path == queue_context.current_call_on_path() {
                queue_context.push_event(Event::PopupClick);
                widget.set_state(WidgetState::None);
            } else {
                match pressed_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        queue_context.push_callback(move |_, window_context, _| {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            Some(Some(None))
        },
        Event::Client(ClientEvent::TouchMotion(_, id, _)) => {
            let motion_call_on_path = queue_context.motion_call_on_path(CallOnId::Touch(*id));
            if motion_call_on_path != queue_context.current_call_on_path() {
                match motion_call_on_path {
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        let tmp_abs_widget_path = abs_widget_path.clone();
                        queue_context.push_callback(move |_, window_context, _| {
                                window_context.dyn_widget_mut(&tmp_abs_widget_path)?.set_state(WidgetState::None);
                                Some(())
                        });
                    },
                    _ => (),
                }
            }
            queue_context.set_motion_call_on_path(CallOnId::Pointer, queue_context.current_call_on_path()?.clone());            
            widget.set_state(WidgetState::Hover);
            Some(Some(None))
        },
        _ => Some(None),
    }
}
