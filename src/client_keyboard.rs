//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::os::unix::io::RawFd;
use wayland_client::protocol::wl_keyboard;
use wayland_client::protocol::wl_surface;
use crate::client_context::*;
use crate::events::*;
use crate::queue_context::*;
use crate::window_context::*;

pub(crate) fn initialize_keyboard(_client_context: &mut ClientContext, _format: wl_keyboard::KeymapFormat, _fd: RawFd, _size: u32)
{}

pub(crate) fn prepare_event_for_client_keyboard_enter(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _surface: &wl_surface::WlSurface) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_keyboard_leave(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _surface: &wl_surface::WlSurface) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_keyboard_key(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _time: u32, _key: u32, _state: wl_keyboard::KeyState) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_keyboard_modifiers(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _queue_context: &mut QueueContext, _mods_depressed: u32, _mods_latched: u32, _mods_locked: u32, _group: u32) -> Event
{ Event::Char('e') }
