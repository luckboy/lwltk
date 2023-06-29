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
use lwltk::widgets::Check;
use lwltk::widgets::Button;
use lwltk::widgets::Label;
use lwltk::widgets::LinearLayout;
use lwltk::widgets::Radio;
use lwltk::widgets::RadioGroup;
use lwltk::windows::ToplevelWindow;
use lwltk::AbsWidgetPath;
use lwltk::App;
use lwltk::Orient;
use lwltk::PreferredSize;
use lwltk::QueueContext;
use lwltk::Size;
use lwltk::WindowIndex;
use lwltk::WindowContext;

struct AppData
{
    window_index: WindowIndex,
    #[allow(dead_code)]
    layout_path: AbsWidgetPath,
    #[allow(dead_code)]
    label1_path: AbsWidgetPath,
    button_path: AbsWidgetPath,
    #[allow(dead_code)]
    label2_path: AbsWidgetPath,
    check1_path: AbsWidgetPath,
    check2_path: AbsWidgetPath,
    check3_path: AbsWidgetPath,
    #[allow(dead_code)]
    label3_path: AbsWidgetPath,
    radio1_path: AbsWidgetPath,
    radio2_path: AbsWidgetPath,
    radio3_path: AbsWidgetPath,
}

fn create_app_data(window_context: &mut WindowContext, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>) -> Option<AppData>
{
    let mut window = ToplevelWindow::new()?;
    window.set_title("simple widgets");
    window.set_preferred_size(Size::new(Some(256), None));
    let window_idx = window_context.add_window(window)?;
    let mut layout = LinearLayout::new();
    layout.set_orient(Orient::Vertical);
    let layout_path = window_context.abs_widget_path1(window_idx, |window: &mut ToplevelWindow| window.set(layout))?;
    let label1 = Label::new("Some button:");
    let label1_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(label1))?;
    let button = Button::new("Some button");
    let button_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(button))?;
    let label2 = Label::new("Some checks:");
    let label2_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(label2))?;
    let check1 = Check::new("Some check1");
    let check1_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(check1))?;
    let check2 = Check::new("Some check2");
    let check2_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(check2))?;
    let check3 = Check::new("Some check3");
    let check3_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(check3))?;
    let label3 = Label::new("Some radios:");
    let label3_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(label3))?;
    let radio_group = Arc::new(RadioGroup::new());
    let radio1 = Radio::new("Some radio1", radio_group.clone());
    let radio1_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(radio1))?;
    let radio2 = Radio::new("Some radio2", radio_group.clone());
    let radio2_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(radio2))?;
    let radio3 = Radio::new("Some radio3", radio_group.clone());
    let radio3_path = window_context.abs_widget_path(&layout_path, |layout: &mut LinearLayout| layout.add(radio3))?;
    Some(AppData {
            window_index: window_idx,
            layout_path,
            label1_path,
            button_path,
            label2_path,
            check1_path,
            check2_path,
            check3_path,
            label3_path,
            radio1_path,
            radio2_path,
            radio3_path,
    })
}

fn set_app_data(window_context: &mut WindowContext, app_data: &mut AppData, _window_context2: Arc<RwLock<WindowContext>>, _queue_context2: Arc<Mutex<QueueContext>>, _app_data2: Arc<RwLock<AppData>>) -> Option<()>
{
    window_context.widget_mut::<Button>(&app_data.button_path)?.set_on(move |_, _, event| {
            match event {
                Event::Click => println!("Clicked button!"),
                Event::DoubleClick => println!("Doubly clicked button!"),
                Event::LongClick => println!("Longly clicked button!"),
                _ => (),
            }
            Some(EventOption::Default)
    });
    window_context.widget_mut::<Check>(&app_data.check1_path)?.set_on(move |_, _, event| {
            match event {
                Event::CheckChange(is_checked) => println!("Changed check1: {}", is_checked),
                _ => (),
            }
            Some(EventOption::Default)
    });    
    window_context.widget_mut::<Check>(&app_data.check2_path)?.set_on(move |_, _, event| {
            match event {
                Event::CheckChange(is_checked) => println!("Changed check2: {}", is_checked),
                _ => (),
            }
            Some(EventOption::Default)
    });    
    window_context.widget_mut::<Check>(&app_data.check3_path)?.set_on(move |_, _, event| {
            match event {
                Event::CheckChange(is_checked) => println!("Changed check3: {}", is_checked),
                _ => (),
            }
            Some(EventOption::Default)
    });
    window_context.widget_mut::<Radio>(&app_data.radio1_path)?.set_on(move |_, _, event| {
            match event {
                Event::RadioSelection(selected) => println!("Selected radio1: {}", selected),
                _ => (),
            }
            Some(EventOption::Default)
    });    
    window_context.widget_mut::<Radio>(&app_data.radio2_path)?.set_on(move |_, _, event| {
            match event {
                Event::RadioSelection(selected) => println!("Selected radio2: {}", selected),
                _ => (),
            }
            Some(EventOption::Default)
    });    
    window_context.widget_mut::<Radio>(&app_data.radio3_path)?.set_on(move |_, _, event| {
            match event {
                Event::RadioSelection(selected) => println!("Selected radio3: {}", selected),
                _ => (),
            }
            Some(EventOption::Default)
    });    
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
