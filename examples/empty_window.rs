//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use lwltk::events::Event;
use lwltk::events::EventOption;
use lwltk::windows::ToplevelWindow;
use lwltk::App;
use lwltk::PreferredSize;
use lwltk::QueueContext;
use lwltk::Size;
use lwltk::ThreadSignalSender;
use lwltk::WindowIndex;
use lwltk::WindowContext;

struct AppData
{
    window_index: WindowIndex,
}

fn create_app_data(window_context: &mut WindowContext, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>, _thread_signal_sender: ThreadSignalSender) -> Option<AppData>
{
    let mut window = ToplevelWindow::new()?;
    window.set_title("Empty window");
    window.set_preferred_size(Size::new(Some(256), Some(256)));
    let window_idx = window_context.add_window(window)?;
    Some(AppData {
            window_index: window_idx,
    })
}

fn set_app_data(window_context: &mut WindowContext, app_data: &mut AppData, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>, _thread_signal_sender: ThreadSignalSender, _app_data2: Arc<RwLock<AppData>>) -> Option<()>
{
    window_context.window_mut::<ToplevelWindow>(app_data.window_index)?.set_on(move |client_context, _, event| {
            match event {
                Event::Close => client_context.exit(),
                _ => (),
            }
            Some(EventOption::Default)
    });
    Some(())
}

fn main()
{
    match App::new(create_app_data, set_app_data) {
        Ok(mut app) => {
            match app.run() {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("lwltk: {}", err);
                    exit(1);
                },
            }
        },
        Err(err) => {
            eprintln!("lwltk: {}", err);
            exit(1);
        },
    }
}
