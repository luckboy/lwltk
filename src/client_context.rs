//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::io::ErrorKind;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::*;
use memmap2::MmapOptions;
use memmap2::MmapMut;
use nix::errno::Errno;
use nix::poll::PollFd;
use nix::poll::PollFlags;
use nix::poll::poll;
use wayland_client::protocol::wl_buffer;
use wayland_client::protocol::wl_compositor;
use wayland_client::protocol::wl_keyboard;
use wayland_client::protocol::wl_pointer;
use wayland_client::protocol::wl_seat;
use wayland_client::protocol::wl_shm;
use wayland_client::protocol::wl_shell;
use wayland_client::protocol::wl_shell_surface;
use wayland_client::protocol::wl_surface;
use wayland_client::protocol::wl_touch;
use wayland_client::Display;
use wayland_client::EventQueue as WaylandEventQueue;
use wayland_client::Filter;
use wayland_client::GlobalManager;
use wayland_client::Main;
use wayland_client::event_enum;
use crate::client_error::*;
use crate::queue_context::*;
use crate::thread_signal::*;
use crate::window_context::*;

pub(crate) struct ClientDisplay
{
    display: Display,
    event_queue: WaylandEventQueue,
}

pub struct ClientContext
{
    pub(crate) compositor: Main<wl_compositor::WlCompositor>,
    pub(crate) shell: Main<wl_shell::WlShell>,
    pub(crate) seat: Main<wl_seat::WlSeat>,
}

impl ClientContext
{
    pub(crate) fn new() -> Result<(ClientDisplay, Self), ClientError>
    {
        let display = match Display::connect_to_env() {
            Ok(tmp_display) => tmp_display,
            Err(err) => return Err(ClientError::Connect(err)),
        };
        let mut event_queue = display.create_event_queue();
        let attached_display = (*display).clone().attach(event_queue.token());
        let global_manager = GlobalManager::new(&attached_display);
        match event_queue.sync_roundtrip(&mut (), |_, _, _| ()) {
            Ok(_) => (),
            Err(err) => return Err(ClientError::Io(err)),
        }
        let compositor = match global_manager.instantiate_exact::<wl_compositor::WlCompositor>(1) {
            Ok(tmp_compositor) => tmp_compositor,
            Err(err) => return Err(ClientError::Global(err)),
        };
        let shell = match global_manager.instantiate_exact::<wl_shell::WlShell>(1) {
            Ok(tmp_shell) => tmp_shell,
            Err(err) => return Err(ClientError::Global(err)),
        };
        let seat = match global_manager.instantiate_exact::<wl_seat::WlSeat>(1) {
            Ok(tmp_seat) => tmp_seat,
            Err(err) => return Err(ClientError::Global(err)),
        };
        Ok((ClientDisplay {
                display,
                event_queue,
        }, ClientContext {
            compositor,
            shell,
            seat,
        }))
    }
}

event_enum!(
    WaylandEvent |
    Pointer => wl_pointer::WlPointer,
    Keyboard => wl_keyboard::WlKeyboard,
    Touch => wl_touch::WlTouch
);

pub(crate) fn run_main_loop(client_display: &mut ClientDisplay, client_context: Rc<RefCell<ClientContext>>, window_context: Arc<RwLock<WindowContext>>, queue_context: Arc<Mutex<QueueContext>>, thread_signal_receiver: ThreadSignalReceiver) -> Result<(), ClientError>
{
    let client_context2 = client_context.clone();
    {
        let client_context_r = client_context.borrow_mut();
        let filter = Filter::new(move |event, _, _| {
                match event {
                    WaylandEvent::Pointer { event, .. } => {
                        match event {
                            wl_pointer::Event::Enter { serial, surface, surface_x, surface_y, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_pointer::Event::Leave { serial, surface, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_pointer::Event::Motion { time, surface_x, surface_y, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_pointer::Event::Button { serial, time, button, state, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_pointer::Event::Axis { time, axis, value, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            _ => (),
                        }
                    },
                    WaylandEvent::Keyboard { event, .. } => {
                        match event {
                            wl_keyboard::Event::Keymap { format, fd, size, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_keyboard::Event::Enter { serial, surface, .. } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_keyboard::Event::Leave { serial, surface, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_keyboard::Event::Key { serial, time, key, state, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_keyboard::Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            _ => (),
                        }
                    },
                    WaylandEvent::Touch { event, .. } => {
                        match event {
                            wl_touch::Event::Down { serial, time, surface, id, x, y, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_touch::Event::Up { serial, time, id,  } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_touch::Event::Motion { time, id, x, y, } => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_touch::Event::Frame => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            wl_touch::Event::Cancel => {
                                let client_context_r = client_context2.borrow_mut();
                            },
                            _ => (),
                        }
                    },
                }
        });
        let mut is_pointer = false;
        let mut is_keyboard = false;
        let mut is_touch = false;
        client_context_r.seat.quick_assign(move |seat, event, _| {
                match event {
                    wl_seat::Event::Capabilities { capabilities } => {
                        if !is_pointer && capabilities.contains(wl_seat::Capability::Pointer) {
                            seat.get_pointer().assign(filter.clone());
                            is_pointer = true;
                        }
                        if !is_keyboard && capabilities.contains(wl_seat::Capability::Keyboard) {
                            seat.get_keyboard().assign(filter.clone());
                            is_keyboard = true;
                        }
                        if !is_touch && capabilities.contains(wl_seat::Capability::Touch) {
                            seat.get_keyboard().assign(filter.clone());
                            is_touch = true;
                        }
                    },
                    _ => (),
                }
        });
    }
    loop {
        match client_display.display.flush() {
            Err(err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(err) => return Err(ClientError::Io(err)),
            _ => (),
        }
        let mut poll_fds: [PollFd; 2] = [
            PollFd::new(client_display.display.get_connection_fd(), PollFlags::POLLIN),
            PollFd::new(thread_signal_receiver.as_raw_fd(), PollFlags::POLLIN)
        ];
        loop {
            match poll(&mut poll_fds, -1) {
                Ok(_) => break,
                Err(Errno::EINTR) => (),
                Err(err) => return Err(ClientError::Nix(err)),
            }
        }
        match poll_fds[0].revents() {
            Some(revents) => {
                if !revents.is_empty() {
                    match client_display.event_queue.prepare_read() {
                        Some(guard) => {
                            match guard.read_events() {
                                Err(err) if err.kind() == ErrorKind::WouldBlock => (),
                                Err(err) => return Err(ClientError::Io(err)),
                                _ => (),
                            }
                        },
                        None => (),
                    }
                    match client_display.event_queue.dispatch_pending(&mut (), |_, _, _| ()) {
                        Ok(_) => (),
                        Err(err) => return Err(ClientError::Io(err)),
                    }
                }
            },
            None => (),
        }
        match poll_fds[1].revents() {
            Some(revents) => {
                if !revents.is_empty() {
                    let mut is_timer = false;
                    let mut is_other = false;
                    loop {
                        match thread_signal_receiver.recv()? {
                            Some(ThreadSignal::Timer) => is_timer = true,
                            Some(ThreadSignal::Other) => is_other = true,
                            None => (),
                        }
                        let mut poll_fds: [PollFd; 1] = [
                            PollFd::new(thread_signal_receiver.as_raw_fd(), PollFlags::POLLIN)
                        ];
                        loop {
                            match poll(&mut poll_fds, 0) {
                                Ok(_) => break,
                                Err(Errno::EINTR) => (),
                                Err(err) => return Err(ClientError::Nix(err)),
                            }
                        }
                        match poll_fds[1].revents() {
                            Some(revents) => {
                                if revents.is_empty() { break; }
                            },
                            None => break,
                        }
                    }
                }
            },
            None => (),
        }
    }
    Ok(())
}
