//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeSet;
use std::env;
use std::fs::*;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::*;
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
use crate::queue_context::*;
use crate::window::*;
use crate::window_context::*;
use crate::types::*;

pub(crate) struct ClientWindow
{
    pub(crate) surface: Main<wl_surface::WlSurface>,
    pub(crate) shell_surface: Main<wl_shell_surface::WlShellSurface>,
    pub(crate) buffer: Main<wl_buffer::WlBuffer>,
    pub(crate) file: File,
    pub(crate) mmap: MmapMut,
    pub(crate) size: Size<i32>,
    pub(crate) title: Option<String>,
    pub(crate) is_maximized: bool,
    pub(crate) parent_index: Option<WindowIndex>,
    pub(crate) child_indices: BTreeSet<WindowIndex>,
}

fn create_buffer(client_context: &ClientContext, window: &dyn Window) -> Result<(Main<wl_buffer::WlBuffer>, File, MmapMut), ClientError>
{
    match env::var("XDG_RUNTIME_DIR") {
        Ok(xdg_runtime_dir) => {
            let mut tempfile_builder = tempfile::Builder::new();
            tempfile_builder.prefix("lwltk-");
            match tempfile_builder.tempfile_in(xdg_runtime_dir) {
                Ok(named_temp_file) => {
                    let tmp_file = named_temp_file.into_file();
                    let scale = client_context.scale;
                    let size = window.width() * window.height() * scale * scale * 4;
                    match tmp_file.set_len(size as u64) {
                        Ok(()) => {
                            let mut mmap_opts = MmapOptions::new();
                            mmap_opts.len(size as usize);
                            match unsafe { mmap_opts.map_mut(&tmp_file) } {
                                Ok(mmap) => {
                                    let shm_pool = client_context.shm.create_pool(tmp_file.as_raw_fd(), size);
                                    let buffer = shm_pool.create_buffer(0, window.width() * scale, window.height() * scale, window.width() * scale * 4, wl_shm::Format::Argb8888);
                                    shm_pool.destroy();
                                    Ok((buffer, tmp_file, mmap))
                                },
                                Err(err) => Err(ClientError::Io(err)),
                            }
                        },
                        Err(err) => Err(ClientError::Io(err)),
                    }
                },
                Err(err) => Err(ClientError::Io(err)),   
            }
        },
        Err(_) => Err(ClientError::NoXdgRuntimeDir),
    }
}

impl ClientWindow
{
    pub(crate) fn new(client_context: &ClientContext, window: &dyn Window) -> Result<ClientWindow, ClientError>
    {
       let surface = client_context.compositor.create_surface();
       let shell_surface = client_context.shell.get_shell_surface(&surface);
       let size = window.size();
       let title = window.title().map(|s| String::from(s));
       let is_maximized = window.is_maximized();
       let (buffer, file, mmap) = create_buffer(client_context, window)?;
       Ok(ClientWindow {
               surface,
               shell_surface,
               buffer,
               file,
               mmap,
               size,
               title,
               is_maximized,
               parent_index: None,
               child_indices: BTreeSet::new(),
       })
    }
    
    pub(crate) fn assign(&self, client_context2: Rc<RefCell<ClientContext>>, window_context2: Arc<RwLock<WindowContext>>, queue_context2: Arc<Mutex<QueueContext>>)
    {
         self.shell_surface.quick_assign(move |shell_surface, event, _| {
                 match  event {
                     wl_shell_surface::Event::Ping { serial, } => {
                         let client_context_r = client_context2.borrow_mut();
                         shell_surface.pong(serial);
                     },
                     wl_shell_surface::Event::Configure { edges, width, height, } => {
                         let client_context_r = client_context2.borrow_mut();
                     },
                     wl_shell_surface::Event::PopupDone => {
                         let client_context_r = client_context2.borrow_mut();
                     },
                     _ => (),
                 }
         });
    }
    
    pub(crate) fn destroy(&self)
    {
        self.buffer.destroy();
        self.surface.destroy();
    }
}
