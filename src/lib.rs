//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
mod app;
mod as_any;
mod call_on;
mod callback_queue;
mod client_context;
mod client_error;
mod client_keyboard;
mod client_pointer;
mod client_shell_surface;
mod client_touch;
mod client_window;
mod container;
mod draw;
mod event_handler;
mod event_queue;
mod key_map_init;
mod min_size;
mod mod_key_set_init;
mod preferred_size;
mod queue_context;
mod theme;
mod thread_signal;
mod types;
mod widget;
mod window;
mod window_container;
mod window_context;

pub mod cursors;
pub mod events;
pub mod keys;
pub mod themes;
pub mod utils;
pub mod windows;

#[cfg(test)]
mod mocks;

pub use crate::app::*;
pub use crate::as_any::*;
pub use crate::call_on::*;
pub use crate::callback_queue::*;
pub use crate::client_context::*;
pub use crate::client_error::*;
pub use crate::container::*;
pub use crate::draw::*;
pub use crate::event_queue::*;
pub use crate::min_size::*;
pub use crate::preferred_size::*;
pub use crate::queue_context::*;
pub use crate::theme::*;
pub use crate::thread_signal::*;
pub use crate::types::*;
pub use crate::widget::*;
pub use crate::window::*;
pub use crate::window_container::*;
pub use crate::window_context::*;
