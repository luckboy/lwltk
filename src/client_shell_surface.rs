//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use wayland_client::protocol::wl_shell_surface;
use crate::client_context::*;
use crate::client_error::*;
use crate::event_queue::*;
use crate::events::*;
use crate::queue_context::*;
use crate::types::*;
use crate::window_context::*;

pub(crate) fn prepare_event_for_client_shell_surface_configure(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, shell_surface: &wl_shell_surface::WlShellSurface, resize: wl_shell_surface::Resize, width: i32, height: i32) -> Option<Event>
{
    let client_resize = match resize {
        wl_shell_surface::Resize::Top => ClientResize::Top,
        wl_shell_surface::Resize::Bottom => ClientResize::Bottom,
        wl_shell_surface::Resize::Left => ClientResize::Left,
        wl_shell_surface::Resize::Right => ClientResize::Right,
        wl_shell_surface::Resize::TopLeft => ClientResize::TopLeft,
        wl_shell_surface::Resize::TopRight => ClientResize::TopRight,
        wl_shell_surface::Resize::BottomLeft => ClientResize::BottomLeft,
        wl_shell_surface::Resize::BottomRight => ClientResize::BottomRight,
        _ => ClientResize::None,
    };
    match client_context.window_index_for_shell_surface(shell_surface) {
        Some(window_idx) => {
            let size = Size::new((width + client_context.fields.scale - 1) / client_context.fields.scale, (height + client_context.fields.scale - 1) / client_context.fields.scale);
            window_context.current_window_index = Some(window_idx);
            queue_context.current_call_on_path = Some(CallOnPath::Window(window_idx));
            Some(Event::Client(ClientEvent::ShellSurfaceConfigure(client_resize, size)))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::NoClientWindow);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_shell_surface_popup_done(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, shell_surface: &wl_shell_surface::WlShellSurface) -> Option<Event>
{
    match client_context.window_index_for_shell_surface(shell_surface) {
        Some(window_idx) => {
            window_context.current_window_index = Some(window_idx);
            queue_context.current_call_on_path = Some(CallOnPath::Window(window_idx));
            Some(Event::Client(ClientEvent::ShellSurfacePopupDone))
        },
        None => {
            eprintln!("lwltk: {}", ClientError::NoClientWindow);
            None
        },
    }
}
