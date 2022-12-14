//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use wayland_client::protocol::wl_shell_surface;
use crate::client_context::*;
use crate::events::*;
use crate::window_context::*;

pub(crate) fn prepare_event_for_client_shell_surface_configure(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _shell_surface: &wl_shell_surface::WlShellSurface, _resize: wl_shell_surface::Resize, _width: i32, _height: i32) -> Event
{ Event::Char('e') }

pub(crate) fn prepare_event_for_client_shell_surface_popup_done(_client_context: &mut ClientContext, _window_context: &mut WindowContext, _shell_surface: &wl_shell_surface::WlShellSurface) -> Event
{ Event::Char('e') }
