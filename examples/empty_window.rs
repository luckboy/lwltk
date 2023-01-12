//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use std::sync::RwLock;
use lwltk::windows::ToplevelWindow;
use lwltk::App;
use lwltk::PreferredSize;
use lwltk::Size;
use lwltk::WindowIndex;
use lwltk::WindowContext;

struct AppData
{
    #[allow(dead_code)]
    window_index: WindowIndex,
}

fn creating_f(window_context: &mut WindowContext) -> Option<AppData>
{
    let mut window = ToplevelWindow::new();
    window.set_title("empty window");
    window.set_preferred_size(Size::new(Some(256), Some(256)));
    let window_idx = window_context.add_window(window)?;
    Some(AppData {
            window_index: window_idx,
    })
}

fn setting_f(_window_context: &mut WindowContext, _data: Arc<RwLock<AppData>>) -> Option<()>
{ Some(()) }

fn main()
{
    match App::new(creating_f, setting_f) {
        Ok(mut app) => {
            match app.run() {
                Ok(()) => (),
                Err(err) => eprintln!("lwltk: {}", err),
            }
        },
        Err(err) => eprintln!("lwltk: {}", err),
    }
}
