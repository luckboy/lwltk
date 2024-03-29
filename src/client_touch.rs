//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::mpsc;
use wayland_client::protocol::wl_surface;
use crate::client_context::*;
use crate::client_error::*;
use crate::events::*;
use crate::queue_context::*;
use crate::thread_signal::*;
use crate::types::*;
use crate::window_context::*;

pub(crate) fn prepare_event_for_client_touch_down(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, surface: &wl_surface::WlSurface, id: i32, x: f64, y: f64, timer_tx: &mpsc::Sender<ThreadTimerCommand>) -> Option<Event>
{
    match client_context.window_index_for_surface(surface) {
        Some(window_idx) => {
            let pos = Pos::new(x / (client_context.fields.scale as f64), y / (client_context.fields.scale as f64));
            match client_context.add_event_preparation(window_context, CallOnId::Touch(id), window_idx, pos, None) {
                Some((call_on_path, pos)) => {
                    if client_context.fields.touch_ids.is_empty() {
                        match timer_tx.send(ThreadTimerCommand::Start(ThreadTimer::Touch)) {
                            Ok(()) => (),
                            Err(_) => eprintln!("lwltk: {}", ClientError::Send),
                        }
                    }
                    client_context.fields.touch_ids.insert(id);
                    window_context.current_window_index = Some(call_on_path.window_index());
                    window_context.current_pos = Some(pos);
                    queue_context.current_call_on_path = Some(call_on_path);
                    Some(Event::Client(ClientEvent::TouchDown(time, id, pos)))
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

pub(crate) fn prepare_event_for_client_touch_up(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, id: i32, timer_tx: &mpsc::Sender<ThreadTimerCommand>) -> Option<Event>
{
    match client_context.remove_event_preparation(window_context, CallOnId::Touch(id)) {
        Some((call_on_path, pos)) => {
            client_context.fields.touch_ids.remove(&id);
            if client_context.fields.touch_ids.is_empty() {
                match timer_tx.send(ThreadTimerCommand::Stop(ThreadTimer::Touch)) {
                    Ok(()) => (),
                    Err(_) => eprintln!("lwltk: {}", ClientError::Send),
                }
            }
            window_context.current_window_index = Some(call_on_path.window_index());
            window_context.current_pos = Some(pos);
            queue_context.current_call_on_path = Some(call_on_path);
            Some(Event::Client(ClientEvent::TouchUp(time, id)))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::EventPreparation);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_touch_motion(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, id: i32, x: f64, y: f64) -> Option<Event>
{
    let pos = Pos::new(x / (client_context.fields.scale as f64), y / (client_context.fields.scale as f64));
    match client_context.set_event_preparation(window_context, CallOnId::Touch(id), pos) {
        Some((call_on_path, pos)) => {
            window_context.current_window_index = Some(call_on_path.window_index());
            window_context.current_pos = Some(pos);
            queue_context.current_call_on_path = Some(call_on_path);
            Some(Event::Client(ClientEvent::TouchMotion(time, id, pos)))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::EventPreparation);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_repeated_touch(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, id: i32) -> Option<Event>
{
    match client_context.update_event_preparation(window_context, CallOnId::Touch(id)) {
        Some((call_on_path, pos)) => {
            window_context.current_window_index = Some(call_on_path.window_index());
            window_context.current_pos = Some(pos);
            queue_context.current_call_on_path = Some(call_on_path);
            Some(Event::Client(ClientEvent::RepeatedTouch(id)))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::EventPreparation);
            None
        },
    }
}
