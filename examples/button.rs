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
use lwltk::image::ButtonIcon;
use lwltk::widgets::Button;
use lwltk::windows::ToplevelWindow;
use lwltk::AbsWidgetPath;
use lwltk::App;
use lwltk::HAlign;
use lwltk::PreferredSize;
use lwltk::QueueContext;
use lwltk::Size;
use lwltk::WindowIndex;
use lwltk::WindowContext;
use lwltk::VAlign;

struct AppData
{
    #[allow(dead_code)]
    window_index: WindowIndex,
    button_path: AbsWidgetPath,
}

fn create_app_data(window_context: &mut WindowContext, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>) -> Option<AppData>
{
    let mut window = ToplevelWindow::new()?;
    window.set_title("button");
    window.set_preferred_size(Size::new(Some(256), Some(256)));
    let window_idx = window_context.add_window(window)?;
    let mut button = Button::new_with_icon(ButtonIcon::Ok, "OK");
    button.set_h_align(HAlign::Center);
    button.set_v_align(VAlign::Center);
    let button_path = window_context.abs_widget_path1(window_idx, |window: &mut ToplevelWindow| window.set(button))?;
    Some(AppData {
            window_index: window_idx,
            button_path,
    })
}

fn set_app_data(window_context: &mut WindowContext, app_data: &mut AppData, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>, _app_data2: Arc<RwLock<AppData>>) -> Option<()>
{
    window_context.widget_mut::<Button>(&app_data.button_path)?.set_on(|_, _, event| {
            match event {
                Event::Click => println!("Clicked button!"),
                Event::DoubleClick => println!("Doubly clicked button!"),
                Event::LongClick => println!("Longly clicked button!"),
                _ => (),
            }
            Some(EventOption::Default)
    });
    window_context.window_mut::<ToplevelWindow>(app_data.window_index)?.set_on(|_, _, event| {
            match event {
                Event::Click => println!("Clicked window!"),
                Event::DoubleClick => println!("Doubly clicked window!"),
                Event::LongClick => println!("Longly clicked window!"),
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
