//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use wayland_client::protocol::wl_surface;
use crate::client_context::*;
use crate::events::*;
use crate::queue_context::*;
use crate::window_context::*;

pub(crate) fn prepare_event_for_client_touch_down(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _time: u32, _surface: &wl_surface::WlSurface, _id: i32, _x: f64, _y: f64) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_touch_up(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _time: u32, _id: i32) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_touch_motion(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _time: u32, _id: i32, _x: f64, _y: f64) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_touch_frame(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_touch_cancel(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext) -> Event
{ Event::Char('e') }
