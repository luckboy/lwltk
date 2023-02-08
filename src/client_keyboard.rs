//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::os::unix::io::RawFd;
use std::sync::mpsc;
use wayland_client::protocol::wl_keyboard;
use wayland_client::protocol::wl_surface;
use xkbcommon::xkb;
use crate::client_context::*;
use crate::client_error::*;
use crate::event_queue::*;
use crate::events::*;
use crate::keys::*;
use crate::queue_context::*;
use crate::thread_signal::*;
use crate::types::*;
use crate::window_context::*;

fn update_focused_rel_widget_path(window_context: &mut WindowContext, window_idx: WindowIndex) -> Option<CallOnPath>
{
    match window_context.window_container.dyn_window_mut(window_idx) {
        Some(window) => {
            window.update_focused_rel_widget_path();
            match window.focused_rel_widget_path() {
                Some(rel_widget_path) => Some(CallOnPath::Widget(rel_widget_path.to_abs_widget_path(window_idx))),
                None => Some(CallOnPath::Window(window_idx)),
            }
        },
        None => None,
    }
}

fn decode_key_code(client_context: &ClientContext, key_code: u32) -> Option<Option<(Vec<VKey>, String)>>
{
    match &client_context.fields.xkb_state {
        Some(xkb_state) => {
            let keys: Vec<VKey> = xkb_state.key_get_syms(key_code).iter().map(|ks| client_context.fields.keys.get(ks).map(|k| *k)).flatten().collect();
            let s = if client_context.fields.key_modifiers & (KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::NUM | KeyModifiers::LOGO) != KeyModifiers::EMPTY {
                xkb_state.key_get_utf8(key_code)
            } else {
                String::new()
            };
            if !keys.is_empty() || !s.is_empty() {
                Some(Some((keys, s)))
            } else {
                Some(None)
            }
        },
        None => None,
    }
}

pub(crate) fn initialize_keyboard(client_context: &mut ClientContext, format: wl_keyboard::KeymapFormat, fd: RawFd, size: u32)
{
    match format {
        wl_keyboard::KeymapFormat::XkbV1 => {
            match unsafe { xkb::Keymap::new_from_fd(&client_context.fields.xkb_context, fd, size as usize, xkb::compose::FORMAT_TEXT_V1, 0) } {
                Ok(keymap) => client_context.fields.xkb_keymap = keymap,
                Err(err) => {
                    eprintln!("lwltk: {}", ClientError::Io(err));
                    client_context.fields.xkb_keymap = None;
                },
            }
            match &client_context.fields.xkb_keymap {
                Some(keymap) => {
                    client_context.fields.xkb_state = Some(xkb::State::new(keymap));
                    client_context.fields.xkb_shift_mask = (1 as xkb::ModMask) << keymap.mod_get_index("Shift");
                    client_context.fields.xkb_caps_mask = (1 as xkb::ModMask) << keymap.mod_get_index("Lock");
                    client_context.fields.xkb_ctrl_mask = (1 as xkb::ModMask) << keymap.mod_get_index("Control");
                    client_context.fields.xkb_alt_mask = (1 as xkb::ModMask) << keymap.mod_get_index("Mod1");
                    client_context.fields.xkb_num_mask = (1 as xkb::ModMask) << keymap.mod_get_index("Mod2");
                    client_context.fields.xkb_logo_mask = (1 as xkb::ModMask) << keymap.mod_get_index("Mod4");
                },
                None => eprintln!("lwltk: {}", ClientError::NoXkbKeymap),
            }
        },
        _ => eprintln!("lwltk: {}", ClientError::UnsupportedXkbKeymapFormat),
    }
}

pub(crate) fn prepare_event_for_client_keyboard_enter(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, surface: &wl_surface::WlSurface) -> Option<Event>
{
    match client_context.select_window_index_for_surface(surface) {
        Some(window_idx) => {
            match update_focused_rel_widget_path(window_context, window_idx) {
                Some(call_on_path) => {
                    client_context.fields.keyboard_window_index = Some(call_on_path.window_index());
                    window_context.current_window_index = Some(call_on_path.window_index());
                    queue_context.current_call_on_path = Some(call_on_path);
                    Some(Event::Client(ClientEvent::KeyboardEnter))
                },
                None => {
                    eprintln!("lwltk: {}", ClientError::NoWindow);
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

pub(crate) fn prepare_event_for_client_keyboard_leave(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, surface: &wl_surface::WlSurface) -> Option<Event>
{
    match client_context.select_window_index_for_surface(surface) {
        Some(window_idx) => {
            match update_focused_rel_widget_path(window_context, window_idx) {
                Some(call_on_path) => {
                    match client_context.fields.keyboard_window_index {
                        Some(keyboard_window_idx) => {
                            if call_on_path.window_index() != keyboard_window_idx {
                                eprintln!("lwltk: {}", ClientError::DifferentWindows);
                            }
                            client_context.fields.keyboard_window_index = None;
                            window_context.current_window_index = Some(call_on_path.window_index());
                            queue_context.current_call_on_path = Some(call_on_path);
                            Some(Event::Client(ClientEvent::KeyboardEnter))
                        },
                        None => {
                            eprintln!("lwltk: {}", ClientError::NoKeyboardWindowIndex);
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
        None => {
            eprintln!("lwltk: {}", ClientError::NoClientWindow);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_keyboard_key(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, time: u32, key: u32, state: wl_keyboard::KeyState, timer_tx: &mpsc::Sender<ThreadTimerCommand>) -> Option<Event>
{
    let client_state = match state {
        wl_keyboard::KeyState::Released => Some(ClientState::Released),
        wl_keyboard::KeyState::Pressed => Some(ClientState::Pressed),
        _ => None
    };
    match client_state {
        Some(client_state) => {
            match client_context.fields.keyboard_window_index {
                Some(keyboard_window_index) => {
                    match update_focused_rel_widget_path(window_context, keyboard_window_index) {
                        Some(call_on_path) => {
                            let key_code = key + 8;
                            match decode_key_code(client_context, key_code) {
                                Some(Some((keys, s))) => {
                                    let are_only_modifiers = keys.iter().all(|k| client_context.fields.modifier_keys.contains(k)) && s.is_empty();
                                    if !are_only_modifiers {
                                        match client_state {
                                            ClientState::Pressed => {
                                                if client_context.fields.key_codes.is_empty() {
                                                    match timer_tx.send(ThreadTimerCommand::Start(ThreadTimer::Key)) {
                                                        Ok(()) => (),
                                                        Err(_) => eprintln!("lwltk: {}", ClientError::Send),
                                                    }
                                                }
                                                client_context.fields.key_codes.insert(key_code);
                                            },
                                            ClientState::Released => {
                                                client_context.fields.key_codes.remove(&key_code);
                                                if client_context.fields.key_codes.is_empty() {
                                                    match timer_tx.send(ThreadTimerCommand::Stop(ThreadTimer::Key)) {
                                                        Ok(()) => (),
                                                        Err(_) => eprintln!("lwltk: {}", ClientError::Send),
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    window_context.current_window_index = Some(call_on_path.window_index());
                                    queue_context.current_call_on_path = Some(call_on_path);
                                    Some(Event::Client(ClientEvent::KeyboardKey(time, keys, s, client_state)))
                                },
                                Some(None) => None,
                                None => {
                                    eprintln!("lwltk: {}", ClientError::NoXkbState);
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
                None => {
                    eprintln!("lwltk: {}", ClientError::NoKeyboardWindowIndex);
                    None
                },
            }
        },
        None => None,
    }
}

pub(crate) fn prepare_event_for_client_keyboard_modifiers(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, mods_depressed: u32, mods_latched: u32, mods_locked: u32, group: u32) -> Option<Event>
{
    match client_context.fields.keyboard_window_index {
        Some(keyboard_window_index) => {
            match update_focused_rel_widget_path(window_context, keyboard_window_index) {
                Some(call_on_path) => {
                    match &mut client_context.fields.xkb_state {
                        Some(xkb_state) => {
                            xkb_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
                            let mask = xkb_state.serialize_mods(xkb::STATE_MODS_DEPRESSED | xkb::STATE_MODS_LATCHED | xkb::STATE_MODS_LOCKED);
                            let mut key_modifiers = KeyModifiers::EMPTY;
                            if (mask & client_context.fields.xkb_shift_mask) != 0 {
                                key_modifiers |= KeyModifiers::SHIFT;
                            }
                            if (mask & client_context.fields.xkb_caps_mask) != 0 {
                                key_modifiers |= KeyModifiers::CAPS;
                            }
                            if (mask & client_context.fields.xkb_ctrl_mask) != 0 {
                                key_modifiers |= KeyModifiers::CTRL;
                            }
                            if (mask & client_context.fields.xkb_alt_mask) != 0 {
                                key_modifiers |= KeyModifiers::ALT;
                            }
                            if (mask & client_context.fields.xkb_num_mask) != 0 {
                                key_modifiers |= KeyModifiers::NUM;
                            }
                            if (mask & client_context.fields.xkb_logo_mask) != 0 {
                                key_modifiers |= KeyModifiers::LOGO;
                            }
                            client_context.fields.key_modifiers = key_modifiers;
                            window_context.current_window_index = Some(call_on_path.window_index());
                            queue_context.current_call_on_path = Some(call_on_path);
                            Some(Event::Client(ClientEvent::KeyboardModifiers(key_modifiers)))
                        },
                        None => {
                            eprintln!("lwltk: {}", ClientError::NoXkbState);
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
        None => {
            eprintln!("lwltk: {}", ClientError::NoKeyboardWindowIndex);
            None
        },
    }
}

pub(crate) fn prepare_event_for_client_repeated_key(client_context: &mut ClientContext, window_context: &mut WindowContext, queue_context: &mut QueueContext, key_code: u32) -> Option<Event>
{
    match client_context.fields.keyboard_window_index {
        Some(keyboard_window_index) => {
            match update_focused_rel_widget_path(window_context, keyboard_window_index) {
                Some(call_on_path) => {
                    match decode_key_code(client_context, key_code) {
                        Some(Some((keys, s))) => {
                            window_context.current_window_index = Some(call_on_path.window_index());
                            queue_context.current_call_on_path = Some(call_on_path);
                            Some(Event::Client(ClientEvent::RepeatedKey(keys, s)))
                        },
                        Some(None) => None,
                        None => {
                            eprintln!("lwltk: {}", ClientError::NoXkbState);
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
        None => {
            eprintln!("lwltk: {}", ClientError::NoKeyboardWindowIndex);
            None
        },
    }
}
