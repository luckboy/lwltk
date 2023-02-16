//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_context::*;
use crate::client_error::*;
use crate::event_queue::*;
use crate::events::*;
use crate::queue_context::*;
use crate::window_context::*;

fn handle_only_event(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, event: &Event) -> Option<Event>
{
    match &queue_context.current_call_on_path {
        Some(CallOnPath::Window(window_idx)) => {
            match window_context.window_container.dyn_window_mut(*window_idx) {
                Some(window) => {
                    match window.call_on(client_context, queue_context, event) {
                        Some(new_event) => new_event,
                        None => {
                            eprintln!("lwltk: {}", ClientError::Event(event.clone()));
                            None
                        },
                    }
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::NoWindow);
                    None
                },
            }
        },
        Some(CallOnPath::Widget(abs_widget_path)) => {
            match window_context.window_container.dyn_widget_mut(abs_widget_path) {
                Some(widget) => {
                    match widget.call_on(client_context, queue_context, event) {
                        Some(new_event) => new_event,
                        None => {
                            eprintln!("lwltk: {}", ClientError::Event(event.clone()));
                            None
                        },
                    }
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::NoWidget);
                    None
                },
            }
        },
        None => {
            eprintln!("lwltk: {}", ClientError::NoCurrentCallOnPath);
            None
        },
    }
}

fn handle_only_event_with_propagation(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, event: &Event)
{
    let mut new_event = handle_only_event(client_context, window_context, queue_context, event);
    loop {
        match &new_event {
            Some(tmp_event) => {
                match &mut queue_context.current_call_on_path {
                    Some(CallOnPath::Window(_)) => break,
                    Some(CallOnPath::Widget(abs_widget_path)) => {
                        if abs_widget_path.pop().is_none() {
                            queue_context.current_call_on_path = Some(CallOnPath::Window(abs_widget_path.window_index()))
                        }
                    },
                    None => eprintln!("lwltk: {}", ClientError::NoCurrentCallOnPath),
                }
                new_event = handle_only_event(client_context, window_context, queue_context, tmp_event);
            },
            None => break,
        }
    }
    window_context.current_window_index = None;
    queue_context.current_call_on_path = None;
}

fn handle_only_callback(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, callback: &mut (dyn FnMut(&mut ClientContext, &mut WindowContext, &mut QueueContext) -> Option<()> + Send + Sync + 'static))
{
    if callback(client_context, window_context, queue_context).is_none() {
        eprintln!("lwltk: {}", ClientError::Callback);
    }
}

pub(crate) fn handle_events_and_callbacks_from_queues(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext)
{
    while !queue_context.event_queue.is_empty() || !queue_context.callback_queue.is_empty() {
        loop {
            match queue_context.event_queue.pop() {
                Some(event_pair) => {
                    window_context.current_window_index = Some(event_pair.call_on_path.window_index());
                    queue_context.current_call_on_path = Some(event_pair.call_on_path);
                    handle_only_event_with_propagation(client_context, window_context, queue_context, &event_pair.event);
                },
                None => break,
            }
        }
        loop {
            match queue_context.callback_queue.pop() {
                Some(mut callback) => handle_only_callback(client_context, window_context, queue_context, &mut *callback),
                None => break,
            }
        }
    }
}

pub(crate) fn handle_event(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, event: &Event)
{
    handle_only_event_with_propagation(client_context, window_context, queue_context, event);
    handle_events_and_callbacks_from_queues(client_context, window_context, queue_context);
}
