//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeSet;
use std::fs::*;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::*;
use cairo::Format;
use cairo::ImageSurface;
use memmap2::MmapOptions;
use memmap2::MmapMut;
use tempfile;
use wayland_client::protocol::wl_buffer;
use wayland_client::protocol::wl_shell_surface;
use wayland_client::protocol::wl_shm;
use wayland_client::protocol::wl_surface;
use wayland_client::Main;
use crate::client_context::*;
use crate::client_error::*;
use crate::client_shell_surface::*;
use crate::event_handler::*;
use crate::queue_context::*;
use crate::window::*;
use crate::window_context::*;
use crate::theme::*;
use crate::types::*;

pub(crate) struct ClientWindow
{
    pub(crate) surface: Main<wl_surface::WlSurface>,
    pub(crate) shell_surface: Main<wl_shell_surface::WlShellSurface>,
    pub(crate) buffer: Main<wl_buffer::WlBuffer>,
    pub(crate) file: File,
    pub(crate) mmap: MmapMut,
    pub(crate) cairo_surface: ImageSurface,
    pub(crate) size: Size<i32>,
    pub(crate) unmaximized_size: Size<i32>,
    pub(crate) title: Option<String>,
    pub(crate) is_maximized: bool,
    pub(crate) parent_index: Option<WindowIndex>,
    pub(crate) child_indices: BTreeSet<WindowIndex>,
}

fn create_buffer(client_context_fields: &ClientContextFields, window: &dyn Window) -> Result<(Main<wl_buffer::WlBuffer>, File, MmapMut, ImageSurface), ClientError>
{
    let mut tempfile_builder = tempfile::Builder::new();
    tempfile_builder.prefix("lwltk-");
    match tempfile_builder.tempfile_in(client_context_fields.xdg_runtime_dir.as_str()) {
        Ok(named_temp_file) => {
            let tmp_file = named_temp_file.into_file();
            let scale = client_context_fields.scale;
            let size = window.width() * window.height() * scale * scale * 4;
            match tmp_file.set_len(size as u64) {
                Ok(()) => {
                    let mut mmap_opts = MmapOptions::new();
                    mmap_opts.len(size as usize);
                    match unsafe { mmap_opts.map_mut(&tmp_file) } {
                        Ok(mut mmap) => {
                            let shm_pool = client_context_fields.shm.create_pool(tmp_file.as_raw_fd(), size);
                            let buffer = shm_pool.create_buffer(0, window.width() * scale, window.height() * scale, window.width() * scale * 4, wl_shm::Format::Argb8888);
                            shm_pool.destroy();
                            match Format::ARgb32.stride_for_width((window.width() * scale) as u32) {
                                Ok(stride) => {
                                    match unsafe { ImageSurface::create_for_data_unsafe(mmap.as_mut_ptr(), Format::ARgb32, window.width() * scale, window.height() * scale, stride) } {
                                        Ok(cairo_surface) => {
                                            Ok((buffer, tmp_file, mmap, cairo_surface))
                                        },
                                        Err(err) => {
                                            buffer.destroy();
                                            Err(ClientError::Cairo(err))
                                        },
                                    }
                                },
                                Err(err) => {
                                    buffer.destroy();
                                    Err(ClientError::Cairo(err))
                                },
                            }
                        },
                        Err(err) => Err(ClientError::Io(err)),
                    }
                },
                Err(err) => Err(ClientError::Io(err)),
            }
        },
        Err(err) => Err(ClientError::Io(err)),   
    }
}

fn update_window_size_and_window_pos(_window: &mut dyn Window, _theme: &dyn Theme)
{}

impl ClientWindow
{
    pub(crate) fn new(client_context_fields: &ClientContextFields, window: &mut dyn Window, theme: &dyn Theme) -> Result<ClientWindow, ClientError>
    {
        update_window_size_and_window_pos(window, theme);
        let surface = client_context_fields.compositor.create_surface();
        let shell_surface = client_context_fields.shell.get_shell_surface(&surface);
        let size = window.size();
        let title = window.title().map(|s| String::from(s));
        match title.clone() {
            Some(title) => shell_surface.set_title(title),
            None => (),
        }
        let is_maximized = window.is_maximized();
        let (buffer, file, mmap, cairo_surface) = match create_buffer(client_context_fields, window) {
            Ok(tuple) => tuple,
            Err(err) => {
                surface.destroy();
                return Err(err);
            }
        };
        Ok(ClientWindow {
                surface,
                shell_surface,
                buffer,
                file,
                mmap,
                cairo_surface,
                size,
                unmaximized_size: size,
                title,
                is_maximized,
                parent_index: None,
                child_indices: BTreeSet::new(),
        })
    }

    fn draw(&self, _window: &dyn Window, _theme: &dyn Theme, _is_focused_window: bool)
    {}
    
    pub(crate) fn assign(&self, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>)
    {
         self.shell_surface.quick_assign(move |shell_surface, event, _| {
                 match  event {
                     wl_shell_surface::Event::Ping { serial, } => {
                         let mut client_context_r = client_context2.borrow_mut();
                         client_context_r.fields.serial = Some(serial);
                         shell_surface.pong(serial);
                     },
                     wl_shell_surface::Event::Configure { edges, width, height, } => {
                         let client_context_fields3 = client_context2.clone();
                         let window_context3 = window_context2.clone();
                         let queue_context3 = queue_context2.clone();
                         let mut client_context_r = client_context2.borrow_mut();
                         match window_context2.write() {
                             Ok(mut window_context_g) => {
                                 let event = prepare_event_for_client_shell_surface_configure(&mut client_context_r, &mut *window_context_g, &shell_surface, edges, width, height);
                                 match queue_context2.lock() {
                                     Ok(mut queue_context_g) => handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event),
                                     Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                 }
                                 client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context_fields3, window_context3, queue_context3);
                             },
                             Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                         }
                     },
                     wl_shell_surface::Event::PopupDone => {
                         let client_context_fields3 = client_context2.clone();
                         let window_context3 = window_context2.clone();
                         let queue_context3 = queue_context2.clone();
                         let mut client_context_r = client_context2.borrow_mut();
                         match window_context2.write() {
                             Ok(mut window_context_g) => {
                                 let event = prepare_event_for_client_shell_surface_popup_done(&mut client_context_r, &mut *window_context_g, &shell_surface);
                                 match queue_context2.lock() {
                                     Ok(mut queue_context_g) => handle_event(&mut client_context_r, &mut *window_context_g, &mut *queue_context_g, &event),
                                     Err(_) => eprintln!("lwltk: {}", ClientError::Mutex),
                                 }
                                 client_context_r.add_to_destroy_and_create_or_update_client_windows(&mut *window_context_g, client_context_fields3, window_context3, queue_context3);
                             },
                             Err(_) => eprintln!("lwltk: {}", ClientError::RwLock),
                         }
                     },
                     _ => (),
                 }
         });
    }
    
    pub(crate) fn set(&mut self, client_context_fields: &ClientContextFields, window: &mut dyn Window, theme: &dyn Theme, parent_surface: Option<&wl_surface::WlSurface>) -> Result<(), ClientError>
    {
        let scale = client_context_fields.scale;
        match (window.parent_index(), window.pos_in_parent(), parent_surface) {
            (Some(parent_idx), Some(pos_in_parent), Some(parent_surface)) => {
                if window.is_popup() {
                    match client_context_fields.serial {
                        Some(serial) => self.shell_surface.set_popup(&client_context_fields.seat, serial, parent_surface, pos_in_parent.x * scale, pos_in_parent.y * scale, wl_shell_surface::Transient::empty()),
                        None => return Err(ClientError::NoSerial),
                    }
                } else {
                    self.shell_surface.set_transient(parent_surface, pos_in_parent.x * scale, pos_in_parent.y * scale, wl_shell_surface::Transient::empty());
                }
                self.parent_index = Some(parent_idx);
            },
            _ => {
                if !window.is_maximized() {
                    self.shell_surface.set_maximized(None);
                } else {
                    self.shell_surface.set_toplevel();
                }
            },
        }
        self.draw(window, theme, window.is_focused());
        self.surface.attach(Some(&self.buffer), 0, 0);
        self.surface.commit();
        window.clear_change_flag();
        Ok(())
    }

    pub(crate) fn update(&mut self, client_context_fields: &ClientContextFields, window: &mut dyn Window, theme: &dyn Theme) -> Result<(), ClientError>
    {
        let scale = client_context_fields.scale;
        let new_title = window.title().map(|s| String::from(s));
        if self.title == new_title {
            self.title = new_title.clone();
            match new_title {
                Some(new_title) => self.shell_surface.set_title(new_title),
                None => (),
            }
        }
        if window.is_maximized() != self.is_maximized {
            if window.is_maximized() {
                self.unmaximized_size = self.size;
                self.shell_surface.set_maximized(None);
            } else {
                self.shell_surface.set_toplevel();
                window.set_preferred_size(Size::new(Some(self.unmaximized_size.width), Some(self.unmaximized_size.height)));
            }
            self.is_maximized = window.is_maximized();
        }
        if window.is_changed() {
            update_window_size_and_window_pos(window, theme);
            if self.size != window.size() {
                let (buffer, file, mmap, cairo_surface) = create_buffer(client_context_fields, window)?;
                self.buffer = buffer;
                self.mmap = mmap;
                self.cairo_surface = cairo_surface;
                self.draw(window, theme, window.is_focused());
                self.surface.attach(Some(&self.buffer), 0, 0);
                self.surface.damage(0, 0, window.width() * scale, window.height() * scale);
                self.surface.commit();
                self.file = file;
            } else {
                self.draw(window, theme, window.is_focused());
                self.surface.attach(Some(&self.buffer), 0, 0);
                self.surface.damage(0, 0, window.width() * scale, window.height() * scale);
                self.surface.commit();
            }
            self.size = window.size();
            window.clear_change_flag();
        }
        Ok(())
    }
    
    pub(crate) fn add_child(&mut self, idx: WindowIndex)
    { self.child_indices.insert(idx); }
    
    pub(crate) fn remove_child(&mut self, idx: WindowIndex)
    { self.child_indices.remove(&idx); }

    pub(crate) fn destroy(&self)
    {
        self.buffer.destroy();
        self.surface.destroy();
    }
}
