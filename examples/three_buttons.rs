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
use lwltk::widgets::Button;
use lwltk::widgets::LinearLayout;
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
    #[allow(dead_code)]
    layout_path: AbsWidgetPath,
    button1_path: AbsWidgetPath,
    button2_path: AbsWidgetPath,
    button3_path: AbsWidgetPath,
}

fn create_app_data(window_context: &mut WindowContext, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>) -> Option<AppData>
{
    let mut window = ToplevelWindow::new()?;
    window.set_title("three buttons");
    window.set_preferred_size(Size::new(Some(320), Some(240)));
    let window_idx = window_context.add_window(window)?;
    let mut layout = LinearLayout::new();
    layout.set_h_align(HAlign::Fill);
    layout.set_v_align(VAlign::Fill);
    let layout_path = window_context.abs_widget_path1(window_idx, |window: &mut ToplevelWindow| window.set(layout))?;
    let mut button1 = Button::new("Button1");
    button1.set_weight(1);
    button1.set_h_align(HAlign::Fill);
    button1.set_v_align(VAlign::Fill);
    let button1_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(button1))?;
    let mut button2 = Button::new("Button2");
    button2.set_v_align(VAlign::Fill);
    let button2_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(button2))?;
    let mut button3 = Button::new("Button3");
    button3.set_weight(1);
    button3.set_h_align(HAlign::Fill);
    button3.set_v_align(VAlign::Fill);
    let button3_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(button3))?;
    Some(AppData {
            window_index: window_idx,
            layout_path,
            button1_path,
            button2_path,
            button3_path,
    })
}

fn set_app_data(window_context: &mut WindowContext, app_data: &mut AppData, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>, _app_data2: Arc<RwLock<AppData>>) -> Option<()>
{
    window_context.widget_mut::<Button>(&app_data.button1_path)?.set_on(|_, _, event| {
            match event {
                Event::Click => println!("Clicked button1!"),
                Event::DoubleClick => println!("Doubly clicked button1!"),
                Event::LongClick => println!("Longly clicked button1!"),
                _ => (),
            }
            Some(EventOption::Default)
    });
    window_context.widget_mut::<Button>(&app_data.button2_path)?.set_on(|_, _, event| {
            match event {
                Event::Click => println!("Clicked button2!"),
                Event::DoubleClick => println!("Doubly clicked button2!"),
                Event::LongClick => println!("Longly clicked button2!"),
                _ => (),
            }
            Some(EventOption::Default)
    });
    window_context.widget_mut::<Button>(&app_data.button3_path)?.set_on(|_, _, event| {
            match event {
                Event::Click => println!("Clicked button3!"),
                Event::DoubleClick => println!("Doubly clicked button3!"),
                Event::LongClick => println!("Longly clicked button3!"),
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
