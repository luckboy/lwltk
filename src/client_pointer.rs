//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::mpsc;
use wayland_client::protocol::wl_pointer;
use wayland_client::protocol::wl_surface;
use crate::client_context::*;
use crate::client_error::*;
use crate::events::*;
use crate::queue_context::*;
use crate::thread_signal::*;
use crate::types::*;
use crate::window_context::*;

const BTN_LEFT: u32 = 0x110;
const BTN_RIGHT: u32 = 0x111;
const BTN_MIDDLE: u32 = 0x112;

pub(crate) fn prepare_event_for_client_pointer_enter(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, surface: &wl_surface::WlSurface, surface_x: f64, surface_y: f64) -> Option<Event>
{
    match client_context.window_index_for_surface(surface) {
        Some(window_idx) => {
            let pos = Pos::new(surface_x / (client_context.fields.scale as f64), surface_y / (client_context.fields.scale as f64));
            match client_context.add_event_preparation(window_context, CallOnId::Pointer, window_idx, pos, None) {
                Some((call_on_path, pos)) => {
                    client_context.fields.has_cursor = true;
                    window_context.current_window_index = Some(call_on_path.window_index());
                    window_context.current_pos = Some(pos);
                    queue_context.current_call_on_path = Some(call_on_path);
                    Some(Event::Client(ClientEvent::PointerEnter(pos)))
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::EventPreparation);
                    None
                },
            }
        },
        None => {
            eprintln!("lwltk: {}", ClientError::NoClientWindow);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_pointer_leave(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, surface: &wl_surface::WlSurface) -> Option<Event>
{
    match client_context.window_index_for_surface(surface) {
        Some(window_idx) => {
            match client_context.remove_event_preparation(window_context, CallOnId::Pointer) {
                Some((call_on_path, pos)) => {
                    if call_on_path.window_index() != window_idx {
                        eprintln!("lwltk: {}", ClientError::DifferentWindows);
                    }
                    client_context.fields.has_cursor = false;
                    window_context.current_window_index = Some(call_on_path.window_index());
                    window_context.current_pos = Some(pos);
                    queue_context.current_call_on_path = Some(call_on_path);
                    Some(Event::Client(ClientEvent::PointerLeave))
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::EventPreparation);
                    None
                },
            }
        },
        None => {
            eprintln!("lwltk: {}", ClientError::NoClientWindow);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_pointer_motion(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, surface_x: f64, surface_y: f64) -> Option<Event>
{
    let pos = Pos::new(surface_x / (client_context.fields.scale as f64), surface_y / (client_context.fields.scale as f64));
    match client_context.set_event_preparation(window_context, CallOnId::Pointer, pos) {
        Some((call_on_path, pos)) => {
            window_context.current_window_index = Some(call_on_path.window_index());
            window_context.current_pos = Some(pos);
            queue_context.current_call_on_path = Some(call_on_path);
            Some(Event::Client(ClientEvent::PointerMotion(time, pos)))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::EventPreparation);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_pointer_button(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, button: u32, state: wl_pointer::ButtonState, timer_tx: &mpsc::Sender<ThreadTimerCommand>) -> Option<Event>
{
    let client_button = if button == BTN_LEFT {
        Some(ClientButton::Left)
    } else if button == BTN_RIGHT {
        Some(ClientButton::Right)
    } else if button == BTN_MIDDLE {
        Some(ClientButton::Middle)
    } else {
        None
    };
    let client_state = match state {
        wl_pointer::ButtonState::Released => Some(ClientState::Released),
        wl_pointer::ButtonState::Pressed => Some(ClientState::Pressed),
        _ => None
    };
    match client_button {
        Some(client_button) => {
            match client_state {
                Some(client_state) => {
                    match client_context.update_event_preparation(window_context, CallOnId::Pointer) {
                        Some((call_on_path, pos)) => {
                            match (client_button, client_state) {
                                (ClientButton::Left, ClientState::Pressed) => {
                                    match timer_tx.send(ThreadTimerCommand::Start(ThreadTimer::Button)) {
                                        Ok(()) => (),
                                        Err(_) => eprintln!("lwltk: {}", ClientError::Send),
                                    }
                                    client_context.fields.has_pressed_button = true;
                                },
                                (ClientButton::Left, ClientState::Released) => {
                                    client_context.fields.has_pressed_button = false;
                                    match timer_tx.send(ThreadTimerCommand::Stop(ThreadTimer::Button)) {
                                        Ok(()) => (),
                                        Err(_) => eprintln!("lwltk: {}", ClientError::Send),
                                    }
                                    client_context.unset_first_pos(CallOnId::Pointer);
                                },
                                _ => (),
                            }
                            window_context.current_window_index = Some(call_on_path.window_index());
                            window_context.current_pos = Some(pos);
                            queue_context.current_call_on_path = Some(call_on_path);
                            Some(Event::Client(ClientEvent::PointerButton(time, client_button, client_state)))
                        },
                        None => {
                            eprintln!("lwltk: {}", ClientError::EventPreparation);
                            None
                        },
                    }
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::InvalidState);
                    None
                },
            }
        },
        None => {
            eprintln!("lwltk: {}", ClientError::InvalidButton);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_pointer_axis(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, axis: wl_pointer::Axis, value: f64) -> Option<Event>
{
    let client_axis = match axis {
        wl_pointer::Axis::VerticalScroll => Some(ClientAxis::VScroll),
        wl_pointer::Axis::HorizontalScroll => Some(ClientAxis::HScroll),
        _ => None,
    };
    match client_axis {
        Some(client_axis) => {
            match client_context.update_event_preparation(window_context, CallOnId::Pointer) {
                Some((call_on_path, pos)) => {
                    window_context.current_window_index = Some(call_on_path.window_index());
                    window_context.current_pos = Some(pos);
                    queue_context.current_call_on_path = Some(call_on_path);
                    Some(Event::Client(ClientEvent::PointerAxis(time, client_axis, value)))
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::EventPreparation);
                    None
                },
            }
        },
        None => {
            eprintln!("lwltk: {}", ClientError::InvalidAxis);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_repeated_button(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext) -> Option<Event>
{
    match client_context.update_event_preparation(window_context, CallOnId::Pointer) {
        Some((call_on_path, pos)) => {
            window_context.current_window_index = Some(call_on_path.window_index());
            window_context.current_pos = Some(pos);
            queue_context.current_call_on_path = Some(call_on_path);
            Some(Event::Client(ClientEvent::RepeatedButton))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::EventPreparation);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_post_button_release(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext) -> Option<Event>
{
    match &client_context.fields.post_button_release_call_on_path {
        Some(call_on_path) => {
            window_context.current_window_index = Some(call_on_path.window_index());
            window_context.current_pos = client_context.fields.post_button_release_pos;
            queue_context.current_call_on_path = Some(call_on_path.clone());
            client_context.fields.post_button_release_call_on_path = None;
            client_context.fields.post_button_release_pos = None;
            client_context.fields.has_sent_post_button_release_call_on_path = false;
            Some(Event::Client(ClientEvent::PostButtonRelease))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::NoPostButtonReleaseCallOnPath);
            None
        },
    }
}
