//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use wayland_client::protocol::wl_pointer;
use wayland_client::protocol::wl_surface;
use crate::client_context::*;
use crate::events::*;
use crate::window_context::*;

pub(crate) fn prepare_event_for_client_pointer_enter(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _surface: &wl_surface::WlSurface, _surface_x: f64, _surface_y: f64) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_pointer_leave(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _surface: &wl_surface::WlSurface) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_pointer_motion(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _time: u32, _surface_x: f64, _surface_y: f64) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_pointer_button(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _time: u32, _button: u32, _state: wl_pointer::ButtonState) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_pointer_axis(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _time: u32, _axis: wl_pointer::Axis, _value: f64) -> Event
{ Event::Char('e') }
