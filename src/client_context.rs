//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::env;
use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::*;
use nix::errno::Errno;
use nix::poll::PollFd;
use nix::poll::PollFlags;
use nix::poll::poll;
use wayland_client::protocol::wl_compositor;
use wayland_client::protocol::wl_keyboard;
use wayland_client::protocol::wl_pointer;
use wayland_client::protocol::wl_seat;
use wayland_client::protocol::wl_shm;
use wayland_client::protocol::wl_shell;
use wayland_client::protocol::wl_surface;
use wayland_client::protocol::wl_touch;
use wayland_client::Display;
use wayland_client::EventQueue as WaylandEventQueue;
use wayland_client::Filter;
use wayland_client::GlobalManager;
use wayland_client::Main;
use wayland_client::event_enum;
use crate::client_error::*;
use crate::client_keyboard::*;
use crate::client_pointer::*;
use crate::client_touch::*;
use crate::client_window::*;
use crate::event_handler::*;
use crate::queue_context::*;
use crate::thread_signal::*;
use crate::types::*;
use crate::window_context::*;

const DEFAULT_SCALE: i32 = 1;
const DEFAULT_KEY_REPEAT_DELAY: u64 = 500;
const DEFAULT_KEY_REPEAT_TIME: u64 = 30;
const DEFAULT_TEXT_CURSOR_BLINK_TIME: u64 = 1200;
const DEFAULT_DOUBLE_CLICK_DELAY: u64 = 400;
const DEFAULT_LONG_CLICK_DELAY: u64 = 1000;

pub(crate) struct ClientDisplay
{
    display: Display,
    event_queue: WaylandEventQueue,
}

pub(crate) struct ClientContextFields
{
    pub(crate) compositor: Main<wl_compositor::WlCompositor>,
    pub(crate) shell: Main<wl_shell::WlShell>,
    pub(crate) seat: Main<wl_seat::WlSeat>,
    pub(crate) shm: Main<wl_shm::WlShm>,
    pub(crate) pointer: Option<Main<wl_pointer::WlPointer>>,
    pub(crate) keyboard: Option<Main<wl_keyboard::WlKeyboard>>,
    pub(crate) touch: Option<Main<wl_touch::WlTouch>>,
    pub(crate) serial: Option<u32>,
    pub(crate) xdg_runtime_dir: String,
    pub(crate) scale: i32,
    pub(crate) key_repeat_delay: u64,
    pub(crate) key_repeat_time: u64,
    pub(crate) text_cursor_blink_time: u64,
    pub(crate) double_click_delay: u64,
    pub(crate) long_click_delay: u64,
    pub(crate) has_exit: bool,
}

pub struct ClientContext
{
    pub(crate) fields: ClientContextFields,
    pub(crate) client_windows: BTreeMap<WindowIndex, Box<ClientWindow>>,
    pub(crate) client_windows_to_destroy: VecDeque<BTreeMap<WindowIndex, Box<ClientWindow>>>,
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
        let shm = match global_manager.instantiate_exact::<wl_shm::WlShm>(1) {
            Ok(tmp_shm) => tmp_shm,
            Err(err) => return Err(ClientError::Global(err)),
        };
        let xdg_runtime_dir = match env::var("XDG_RUNTIME_DIR") {
            Ok(tmp_xdg_runtime_dir) => tmp_xdg_runtime_dir,
            Err(_) => return Err(ClientError::NoXdgRuntimeDir),
        };
        let scale = match env::var("LWLTK_SCALE") {
            Ok(s) => {
                match s.parse::<i32>() {
                    Ok(tmp_scale) if tmp_scale <= 0 => {
                        eprintln!("lwltk: warning: invalid value of scale");
                        DEFAULT_SCALE
                    },
                    Ok(tmp_scale) => tmp_scale,
                    Err(_) => {
                        eprintln!("lwltk: warning: invalid value of scale");
                        DEFAULT_SCALE
                    },
                }
            },
            Err(_) => DEFAULT_SCALE,
        };
        let key_repeat_delay = match env::var("LWLTK_KEY_REPEAT_DELAY") {
            Ok(s) => {
                match s.parse::<u64>() {
                    Ok(tmp_repeated_key_delay) => tmp_repeated_key_delay,
                    Err(_) => {
                        eprintln!("lwltk: warning: invalid value of key repeat delay");
                        DEFAULT_KEY_REPEAT_DELAY
                    },
                }
            },
            Err(_) => DEFAULT_KEY_REPEAT_DELAY,
        };
        let key_repeat_time = match env::var("LWLTK_KEY_REPEAT_TIME") {
            Ok(s) => {
                match s.parse::<u64>() {
                    Ok(tmp_repeated_key_time) => tmp_repeated_key_time,
                    Err(_) => {
                        eprintln!("lwltk: warning: invalid value of key repeat time");
                        DEFAULT_KEY_REPEAT_TIME
                    },
                }
            },
            Err(_) => DEFAULT_KEY_REPEAT_TIME,
        };
        let text_cursor_blink_time = match env::var("LWLTK_TEXT_CURSOR_BLINK_TIME") {
            Ok(s) => {
                match s.parse::<u64>() {
                    Ok(tmp_text_cursor_blink_time) => tmp_text_cursor_blink_time,
                    Err(_) => {
                        eprintln!("lwltk: warning: invalid value of text cursor blink time");
                        DEFAULT_TEXT_CURSOR_BLINK_TIME
                    },
                }
            },
            Err(_) => DEFAULT_TEXT_CURSOR_BLINK_TIME,
        };
        let double_click_delay = match env::var("LWLTK_DOUBLE_CLICK_DELAY") {
            Ok(s) => {
                match s.parse::<u64>() {
                    Ok(tmp_double_click_delay) => tmp_double_click_delay,
                    Err(_) => {
                        eprintln!("lwltk: warning: invalid value of double click delay");
                        DEFAULT_DOUBLE_CLICK_DELAY
                    },
                }
            },
            Err(_) => DEFAULT_DOUBLE_CLICK_DELAY,
        };
        let long_click_delay = match env::var("LWLTK_LONG_CLICK_DELAY") {
            Ok(s) => {
                match s.parse::<u64>() {
                    Ok(tmp_long_click_delay) => tmp_long_click_delay,
                    Err(_) => {
                        eprintln!("lwltk: warning: invalid value of long click delay");
                        DEFAULT_LONG_CLICK_DELAY
                    },
                }
            },
            Err(_) => DEFAULT_LONG_CLICK_DELAY,
        };
        Ok((ClientDisplay {
                display,
                event_queue,
        }, ClientContext {
            fields: ClientContextFields {
                compositor,
                shell,
                seat,
                shm,
                pointer: None,
                keyboard: None,
                touch: None,
                serial: None,
                xdg_runtime_dir,
                scale,
                key_repeat_delay,
                key_repeat_time,
                text_cursor_blink_time,
                double_click_delay,
                long_click_delay,
                has_exit: false,
            },
            client_windows: BTreeMap::new(),
            client_windows_to_destroy: VecDeque::new(),
        }))
    }
    
    pub(crate) fn client_window(&self, idx: WindowIndex) -> Option<&ClientWindow>
    {
        match self.client_windows.get(&idx) {
            Some(client_window) => Some(&**client_window),
            None => None,
        }
    }

    pub(crate) fn client_window_mut(&mut self, idx: WindowIndex) -> Option<&mut ClientWindow>
    {
        match self.client_windows.get_mut(&idx) {
            Some(client_window) => Some(&mut **client_window),
            None => None,
        }
    }
    
    pub(crate) fn add_client_window(&mut self, idx: WindowIndex, client_window: Box<ClientWindow>)
    { self.client_windows.insert(idx, client_window); }

    pub(crate) fn remove_client_window(&mut self, idx: WindowIndex) -> Option<Box<ClientWindow>>
    { self.client_windows.remove(&idx) }

    fn create_client_windows_from(&mut self, window_context: &mut WindowContext, idx: WindowIndex, visiteds: &mut BTreeSet<WindowIndex>, parent_surface: Option<&wl_surface::WlSurface>, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>) -> Result<(), ClientError>
    {
        if visiteds.contains(&idx) {
            return Err(ClientError::WindowCycle);
        }
        let child_idxs = match window_context.window_container.dyn_window_mut(idx) {
            Some(window) => {
                let mut client_window = ClientWindow::new(&self.fields, window, &*window_context.theme)?;
                client_window.assign(client_context2.clone(), window_context2.clone(), queue_context2.clone());
                match client_window.set(&self.fields, window, &*window_context.theme, parent_surface) {
                    Ok(()) => (),
                    Err(err) => {
                        client_window.destroy();
                        return Err(err);
                    },
                }
                self.add_client_window(idx, Box::new(client_window));
                window.child_indices().collect::<Vec<WindowIndex>>()
            },
            None => return Err(ClientError::NoWindow),
        };
        visiteds.insert(idx);
        let surface = match self.client_window(idx) {
            Some(client_window) => client_window.surface.clone(),
            None => return Err(ClientError::NoWindow),
        };
        for child_idx in &child_idxs {
            self.create_client_windows_from(window_context, *child_idx, visiteds, Some(&surface), client_context2.clone(), window_context2.clone(), queue_context2.clone())?;
            match self.client_window_mut(idx) {
                Some(client_window) => client_window.add_child(*child_idx),
                None => return Err(ClientError::NoWindow),
            }
        }
        Ok(())
    }
    
    fn create_client_windows(&mut self, window_context: &mut WindowContext, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>) -> Result<(), ClientError>
    {
        match (window_context.focused_window_index, window_context.old_focused_window_index) {
            (Some(idx), Some(old_idx)) => {
                if idx != old_idx {
                    match window_context.window_container.dyn_window_mut(old_idx) {
                        Some(window) => window.set_focus(false),
                        None => return Err(ClientError::NoWindow),
                    }
                }
                match window_context.window_container.dyn_window_mut(idx) {
                    Some(window) => window.set_focus(true),
                    None => return Err(ClientError::NoWindow),
                }
            },
            (Some(idx), None) => {
                match window_context.window_container.dyn_window_mut(idx) {
                    Some(window) => window.set_focus(true),
                    None => return Err(ClientError::NoWindow),
                }
            },
            (None, Some(old_idx)) => {
                match window_context.window_container.dyn_window_mut(old_idx) {
                    Some(window) => window.set_focus(false),
                    None => return Err(ClientError::NoWindow),
                }
            },
            (None, None) => (),
        }
        window_context.old_focused_window_index = window_context.focused_window_index;
        let idxs: Vec<WindowIndex> = window_context.window_container.window_map().keys().map(|i| *i).collect();
        let mut visiteds: BTreeSet<WindowIndex> = BTreeSet::new();
        for idx in &idxs {
            let is_creating = match window_context.window_container.dyn_window(*idx) {
                Some(window) => !(window.is_popup() || window.is_transient()) && window.is_visible(),
                None => return Err(ClientError::NoWindow),
            };
            if is_creating {
                self.create_client_windows_from(window_context, *idx, &mut visiteds, None, client_context2.clone(), window_context2.clone(), queue_context2.clone())?;
            }
        }
        Ok(())
    }
    
    fn add_child_client_window_indices_to_destroy_from(&mut self, client_windows_to_destroy: &BTreeMap<WindowIndex, Box<ClientWindow>>, idx: WindowIndex, visiteds: &mut BTreeSet<WindowIndex>, indices_to_destroy: &mut Vec<WindowIndex>) -> Result<(), ClientError>
    {
        if !visiteds.contains(&idx) {
            let child_idxs = match map_client_window(client_windows_to_destroy, idx) {
                Some(client_window) => client_window.child_indices.iter().map(|i| *i).collect::<Vec<WindowIndex>>(),
                None => {
                    match self.client_window(idx) {
                        Some(client_window) => client_window.child_indices.iter().map(|i| *i).collect::<Vec<WindowIndex>>(),
                        None => return Err(ClientError::NoClientWindow),
                    }
                },
            };
            visiteds.insert(idx);
            for child_idx in &child_idxs {
                if self.client_window(*child_idx).is_some() {
                    indices_to_destroy.push(*child_idx);
                }
                self.add_child_client_window_indices_to_destroy_from(client_windows_to_destroy, *child_idx, visiteds, indices_to_destroy)?;
            }
        }
        Ok(())
    }
    
    fn add_client_windows_to_destroy(&mut self, window_context: &mut WindowContext) -> Result<(), ClientError>
    {
        let mut client_windows_to_destroy: BTreeMap<WindowIndex, Box<ClientWindow>> = BTreeMap::new();
        for idx in window_context.window_container.window_map().keys() {
            match window_context.window_container.dyn_window(*idx) {
                Some(window) => {
                    if !window.is_visible() {
                        match self.remove_client_window(*idx) {
                            Some(client_window) => add_map_client_window(&mut client_windows_to_destroy, *idx, client_window),
                            None => (),
                        }
                    } else {
                        let is_parent_diff = match self.client_window(*idx) {
                            Some(client_window) => client_window.parent_index != window.parent_index(),
                            None => false,
                        };
                        if is_parent_diff {
                            match self.remove_client_window(*idx) {
                                Some(client_window) => add_map_client_window(&mut client_windows_to_destroy, *idx, client_window),
                                None => (),
                            }
                        }
                    }
                },
                None => return Err(ClientError::NoWindow),
            }
        }
        for idx in window_context.window_container.indices_to_destroy().iter() {
            match self.remove_client_window(*idx) {
                Some(client_window) => add_map_client_window(&mut client_windows_to_destroy, *idx, client_window),
                None => (),
            }
        }
        window_context.window_container.clear_indices_to_destroy();
        let mut idxs_to_destroy: Vec<WindowIndex> = Vec::new();
        let mut visiteds: BTreeSet<WindowIndex> = BTreeSet::new();
        for idx in client_windows_to_destroy.keys() {
            self.add_child_client_window_indices_to_destroy_from(&client_windows_to_destroy, *idx, &mut visiteds, &mut idxs_to_destroy)?;
        }
        for idx in &idxs_to_destroy {
            match self.remove_client_window(*idx) {
                Some(client_window) => add_map_client_window(&mut client_windows_to_destroy, *idx, client_window),
                None => return Err(ClientError::NoClientWindow),
            }
        }
        for idx in client_windows_to_destroy.keys() {
            let parent_idx = match map_client_window(&client_windows_to_destroy, *idx) {
                Some(client_window) => client_window.parent_index,
                None => return Err(ClientError::NoClientWindow),
            };
            match parent_idx {
                Some(parent_idx) => {
                    match self.client_window_mut(parent_idx) {
                        Some(parent_client_window) => {
                            parent_client_window.remove_child(*idx);
                        },
                        None => return Err(ClientError::NoClientWindow),
                    }
                },
                None => (),
            }
        }
        self.client_windows_to_destroy.push_back(client_windows_to_destroy);
        Ok(())
    }
    
    fn create_or_update_client_windows_from(&mut self, window_context: &mut WindowContext, idx: WindowIndex, visiteds: &mut BTreeSet<WindowIndex>, parent_surface: Option<&wl_surface::WlSurface>, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>) -> Result<(), ClientError>
    {
        if visiteds.contains(&idx) {
            return Err(ClientError::WindowCycle);
        }
        let child_idxs = match map_client_window_mut(&mut self.client_windows, idx) {
            Some(client_window) => {
                match window_context.window_container.dyn_window_mut(idx) {
                    Some(window) => {
                        client_window.update(&self.fields, window, &*window_context.theme)?;
                        window.child_indices().collect::<Vec<WindowIndex>>()
                    },
                    None => return Err(ClientError::NoWindow),
                }
            },
            None => {
                match window_context.window_container.dyn_window_mut(idx) {
                    Some(window) => {
                        let mut client_window = ClientWindow::new(&self.fields, window, &*window_context.theme)?;
                        client_window.assign(client_context2.clone(), window_context2.clone(), queue_context2.clone());
                        match client_window.set(&self.fields, window, &*window_context.theme, parent_surface) {
                            Ok(()) => (),
                            Err(err) => {
                                client_window.destroy();
                                return Err(err);
                            },
                        }
                        self.add_client_window(idx, Box::new(client_window));
                        window.child_indices().collect::<Vec<WindowIndex>>()
                    },
                    None => return Err(ClientError::NoWindow),
                }
            },
        };
        visiteds.insert(idx);
        let surface = match self.client_window(idx) {
            Some(client_window) => client_window.surface.clone(),
            None => return Err(ClientError::NoWindow),
        };
        for child_idx in &child_idxs {
            self.create_or_update_client_windows_from(window_context, *child_idx, visiteds, Some(&surface), client_context2.clone(), window_context2.clone(), queue_context2.clone())?;
            match self.client_window_mut(idx) {
                Some(client_window) => client_window.add_child(*child_idx),
                None => return Err(ClientError::NoWindow),
            }
        }
        Ok(())
    }
    
    fn create_or_update_client_windows(&mut self, window_context: &mut WindowContext, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>) -> Result<(), ClientError>
    {
        match (window_context.focused_window_index, window_context.old_focused_window_index) {
            (Some(idx), Some(old_idx)) => {
                if idx != old_idx {
                    match window_context.window_container.dyn_window_mut(old_idx) {
                        Some(window) => window.set_focus(false),
                        None => return Err(ClientError::NoWindow),
                    }
                    match window_context.window_container.dyn_window_mut(idx) {
                        Some(window) => window.set_focus(true),
                        None => return Err(ClientError::NoWindow),
                    }
                }
            },
            (Some(idx), None) => {
                match window_context.window_container.dyn_window_mut(idx) {
                    Some(window) => window.set_focus(true),
                    None => return Err(ClientError::NoWindow),
                }
            },
            (None, Some(old_idx)) => {
                match window_context.window_container.dyn_window_mut(old_idx) {
                    Some(window) => window.set_focus(false),
                    None => return Err(ClientError::NoWindow),
                }
            },
            (None, None) => (),
        }
        window_context.old_focused_window_index = window_context.focused_window_index;
        let idxs: Vec<WindowIndex> = window_context.window_container.window_map().keys().map(|i| *i).collect();
        let mut visiteds: BTreeSet<WindowIndex> = BTreeSet::new();
        for idx in &idxs {
            let is_creating = match window_context.window_container.dyn_window(*idx) {
                Some(window) => !(window.is_popup() || window.is_transient()) && window.is_visible(),
                None => return Err(ClientError::NoWindow),
            };
            if is_creating {
                self.create_or_update_client_windows_from(window_context, *idx, &mut visiteds, None, client_context2.clone(), window_context2.clone(), queue_context2.clone())?;
            }
        }
        Ok(())
    }
    
    pub(crate) fn add_to_destroy_and_create_or_update_client_windows(&mut self, window_context: &mut WindowContext, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>)
    {
        match self.add_client_windows_to_destroy(window_context) {
            Ok(()) => (),
            Err(err) => eprintln!("lwltk: {}", err),
        }
        match self.create_or_update_client_windows(window_context, client_context2, window_context2, queue_context2) {
            Ok(()) => (),
            Err(err) => eprintln!("lwltk: {}", err),
        }
    }
    
    pub(crate) fn destroy_client_windows_to_destroy(&mut self) -> Result<(), ClientError>
    {
        loop {
            match self.client_windows_to_destroy.pop_front() {
                Some(client_windows) => destroy_map_client_windows(&client_windows)?,
                None => break,
            }
        }
        Ok(())
    }
    
    pub(crate) fn destroy(&mut self)
    {
        match self.destroy_client_windows_to_destroy() {
            Ok(()) => (),
            Err(err) => eprintln!("lwltk: {}", err),
        };
        match destroy_map_client_windows(&self.client_windows) {
            Ok(()) => (),
            Err(err) => eprintln!("lwltk: {}", err),
        }
        self.client_windows.clear();
    }
    
    pub fn has_exit(&self) -> bool
    { self.fields.has_exit }

    pub fn set_exit(&mut self, is_exit: bool)
    { self.fields.has_exit = is_exit; }

    pub fn exit(&mut self)
    { self.fields.has_exit = true; }
}

pub(crate) fn map_client_window(client_windows: &BTreeMap<WindowIndex, Box<ClientWindow>>, idx: WindowIndex) -> Option<&ClientWindow>
{
    match client_windows.get(&idx) {
        Some(client_window) => Some(&**client_window),
        None => None,
    }
}

pub(crate) fn map_client_window_mut(client_windows: &mut BTreeMap<WindowIndex, Box<ClientWindow>>, idx: WindowIndex) -> Option<&mut ClientWindow>
{
    match client_windows.get_mut(&idx) {
        Some(client_window) => Some(&mut **client_window),
        None => None,
    }
}

pub(crate) fn add_map_client_window(client_windows: &mut BTreeMap<WindowIndex, Box<ClientWindow>>, idx: WindowIndex, client_window: Box<ClientWindow>)
{ client_windows.insert(idx, client_window); }

fn destroy_map_client_windows_from(client_windows: &BTreeMap<WindowIndex, Box<ClientWindow>>, idx: WindowIndex, visiteds: &mut BTreeSet<WindowIndex>) -> Result<(), ClientError>
{
    if !visiteds.contains(&idx) {
        match map_client_window(client_windows, idx) {
            Some(client_window) => {
                visiteds.insert(idx);
                for child_idx in client_window.child_indices.iter() {
                    destroy_map_client_windows_from(client_windows, *child_idx, visiteds)?;
                }
                client_window.destroy();
            },
            None => return Err(ClientError::NoClientWindow),
        }
    }
    Ok(())
}

fn destroy_map_client_windows(client_windows: &BTreeMap<WindowIndex, Box<ClientWindow>>) -> Result<(), ClientError>
{
    let mut visiteds: BTreeSet<WindowIndex> = BTreeSet::new();
    for idx in client_windows.keys() {
        destroy_map_client_windows_from(&client_windows, *idx, &mut visiteds)?;
    }
    Ok(())
}

event_enum!(
    WaylandEvent |
    Pointer => wl_pointer::WlPointer,
    Keyboard => wl_keyboard::WlKeyboard,
    Touch => wl_touch::WlTouch
);

pub(crate) fn run_main_loop(client_display: &mut ClientDisplay, client_context: Rc<RefCell<ClientContext>>, window_context: Arc<RwLock<WindowContext>>, queue_context: Arc<Mutex<QueueContext>>, thread_signal_sender: ThreadSignalSender,thread_signal_receiver: ThreadSignalReceiver) -> Result<(), ClientError>
{
    let client_context2 = client_context.clone();
    let window_context2 = window_context.clone();
    let queue_context2 = queue_context.clone();
    let client_context3 = client_context.clone();
    let client_context4 = client_context.clone();
    let window_context4 = window_context.clone();
    let queue_context4 = queue_context.clone();
    {
        let mut client_context_r = client_context.borrow_mut();
        let filter = Filter::new(move |event, _, _| {
                match event {
                    WaylandEvent::Pointer { event, .. } => {
                        match event {
                            wl_pointer::Event::Enter { serial, surface, surface_x, surface_y, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_pointer_enter(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &surface, surface_x, surface_y);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_pointer::Event::Leave { serial, surface, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_pointer_leave(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &surface);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_pointer::Event::Motion { time, surface_x, surface_y, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_pointer_motion(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, surface_x, surface_y);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_pointer::Event::Button { serial, time, button, state, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_pointer_button(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, button, state);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event)
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_pointer::Event::Axis { time, axis, value, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_pointer_axis(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, axis, value);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            _ => (),
                        }
                    },
                    WaylandEvent::Keyboard { event, .. } => {
                        match event {
                            wl_keyboard::Event::Keymap { format, fd, size, } => {
                                let mut client_context_r = client_context2.borrow_mut();
                                initialize_keyboard(&mut *client_context_r, format, fd, size);
                            },
                            wl_keyboard::Event::Enter { serial, surface, .. } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_keyboard_enter(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &surface);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_keyboard::Event::Leave { serial, surface, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_keyboard_leave(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &surface);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_keyboard::Event::Key { serial, time, key, state, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_keyboard_key(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, key, state);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_keyboard::Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_keyboard_modifiers(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, mods_depressed, mods_latched, mods_locked, group);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            _ => (),
                        }
                    },
                    WaylandEvent::Touch { event, .. } => {
                        match event {
                            wl_touch::Event::Down { serial, time, surface, id, x, y, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_touch_down(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, &surface, id, x, y);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_touch::Event::Up { serial, time, id,  } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                client_context_r.fields.serial = Some(serial);
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_touch_up(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, id);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            wl_touch::Event::Motion { time, id, x, y, } => {
                                let client_context3 = client_context2.clone();
                                let window_context3 = window_context2.clone();
                                let queue_context3 = queue_context2.clone();
                                let mut client_context_r = client_context2.borrow_mut();
                                match window_context2.write() {
                                    Ok(mut window_context_g) => {
                                        match queue_context2.lock() {
                                            Ok(mut queue_context_g) => {
                                                let event = prepare_event_for_client_touch_motion(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, time, id, x, y);
                                                handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event);
                                            },
                                            Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                        }
                                        client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context3, window_context3, queue_context3);
                                    },
                                    Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                                }
                            },
                            _ => (),
                        }
                    },
                }
        });
        client_context_r.fields.seat.quick_assign(move |seat, event, _| {
                match event {
                    wl_seat::Event::Capabilities { capabilities } => {
                        let mut client_context_r = client_context3.borrow_mut();
                        if !client_context_r.fields.pointer.is_some() && capabilities.contains(wl_seat::Capability::Pointer) {
                            let pointer = seat.get_pointer();
                            pointer.assign(filter.clone());
                            client_context_r.fields.pointer = Some(pointer);
                        }
                        if !client_context_r.fields.keyboard.is_some() && capabilities.contains(wl_seat::Capability::Keyboard) {
                            let keyboard = seat.get_keyboard();
                            keyboard.assign(filter.clone());
                            client_context_r.fields.keyboard = Some(keyboard);
                        }
                        if !client_context_r.fields.touch.is_some() && capabilities.contains(wl_seat::Capability::Touch) {
                            let touch = seat.get_touch();
                            touch.assign(filter.clone());
                            client_context_r.fields.touch = Some(touch);
                        }
                    },
                    _ => (),
                }
        });
        match window_context.write() {
            Ok(mut window_context_g) => client_context_r.create_client_windows(&mut *window_context_g, client_context4, window_context4, queue_context4)?,
            Err(_) => return Err(ClientError::RwLock),
        }
    }
    loop {
        match client_display.display.flush() {
            Err(err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(err) => {
                let mut client_context_r = client_context.borrow_mut();
                client_context_r.destroy();
                return Err(ClientError::Io(err));
            },
            _ => (),
        }
        let mut poll_fds: [PollFd; 2] = [
            PollFd::new(client_display.display.get_connection_fd(), PollFlags::POLLIN),
            PollFd::new(thread_signal_receiver.fd(), PollFlags::POLLIN)
        ];
        loop {
            match poll(&mut poll_fds, -1) {
                Ok(_) => break,
                Err(Errno::EINTR) => (),
                Err(err) => {
                    let mut client_context_r = client_context.borrow_mut();
                    client_context_r.destroy();
                    return Err(ClientError::Nix(err));
                },
            }
        }
        match poll_fds[0].revents() {
            Some(revents) => {
                if !revents.is_empty() {
                    match client_display.event_queue.prepare_read() {
                        Some(guard) => {
                            match guard.read_events() {
                                Err(err) if err.kind() == ErrorKind::WouldBlock => (),
                                Err(err) => {
                                    let mut client_context_r = client_context.borrow_mut();
                                    client_context_r.destroy();
                                    return Err(ClientError::Io(err));
                                },
                                _ => (),
                            }
                        },
                        None => (),
                    }
                    match client_display.event_queue.dispatch_pending(&mut (), |_, _, _| ()) {
                        Ok(_) => (),
                        Err(err) => {
                            let mut client_context_r = client_context.borrow_mut();
                            client_context_r.destroy();
                            return Err(ClientError::Io(err));
                        },
                    }
                }
            },
            None => (),
        }
        match poll_fds[1].revents() {
            Some(revents) => {
                if !revents.is_empty() {
                    let mut is_cursor_timer = false;
                    let mut is_key_timer = false;
                    let mut is_text_cursor_timer = false;
                    let mut is_other = false;
                    loop {
                        match thread_signal_receiver.recv() {
                            Ok(Some(ThreadSignal::Timer(ThreadTimer::Cursor))) => is_cursor_timer = true,
                            Ok(Some(ThreadSignal::Timer(ThreadTimer::Key))) => is_key_timer = true,
                            Ok(Some(ThreadSignal::Timer(ThreadTimer::TextCursor))) => is_text_cursor_timer = true,
                            Ok(Some(ThreadSignal::Other)) => is_other = true,
                            Ok(None) => (),
                            Err(err) => {
                                let mut client_context_r = client_context.borrow_mut();
                                client_context_r.destroy();
                                return Err(err);
                            }
                        }
                        let mut poll_fds: [PollFd; 1] = [
                            PollFd::new(thread_signal_receiver.fd(), PollFlags::POLLIN)
                        ];
                        loop {
                            match poll(&mut poll_fds, 0) {
                                Ok(_) => break,
                                Err(Errno::EINTR) => (),
                                Err(err) => {
                                    let mut client_context_r = client_context.borrow_mut();
                                    client_context_r.destroy();
                                    return Err(ClientError::Nix(err));
                                },
                            }
                        }
                        match poll_fds[1].revents() {
                            Some(revents) => {
                                if revents.is_empty() { break; }
                            },
                            None => break,
                        }
                    }
                    if is_other {
                        let client_context2 = client_context.clone();
                        let window_context2 = window_context.clone();
                        let queue_context2 = queue_context.clone();
                        let mut client_context_r = client_context.borrow_mut();
                        match window_context.write() {
                            Ok(mut window_context_g) => {
                                client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context2, window_context2, queue_context2);
                            },
                            Err(_) => {
                                client_context_r.destroy();
                                return Err(ClientError::RwLock);
                            },
                        }
                    }
                }
            },
            None => (),
        }
        let mut client_context_r = client_context.borrow_mut();
        match client_context_r.destroy_client_windows_to_destroy() {
            Ok(()) => (),
            Err(err) => {
                client_context_r.destroy();
                return Err(err);
            },
        }
        if client_context_r.fields.has_exit {
            break;
        }
    }
    let mut client_context_r = client_context.borrow_mut();
    client_context_r.destroy();
    Ok(())
}
