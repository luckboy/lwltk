//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::*;
use crate::client_context::*;
use crate::client_error::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::thread_signal::*;
use crate::window_context::*;

/// An application structure.
///
/// The application structure is a main structure that allows to run an application. This structure
/// contains contexts, the thread signal channel, and an application data.
///
/// # Examples
/// ```no_run
/// use lwltk::events::Event;
/// use lwltk::events::EventOption;
/// use lwltk::widgets::Button;
/// use lwltk::windows::ToplevelWindow;
/// use lwltk::AbsWidgetPath;
/// use lwltk::App;
/// use lwltk::HAlign;
/// use lwltk::PreferredSize;
/// use lwltk::Size;
/// use lwltk::WindowIndex;
/// use lwltk::WindowContext;
/// use lwltk::VAlign;
///
/// struct AppData
/// {
///     window_index: WindowIndex,
///     button_path: AbsWidgetPath,
/// }
///
/// let mut app = App::new(|window_context, _, _, _| {
///         let mut window = ToplevelWindow::new()?;
///         window.set_title("Example");
///         window.set_preferred_size(Size::new(Some(256), Some(256)));
///         let window_idx = window_context.add_window(window)?;
///         let mut button = Button::new("Button");
///         button.set_h_align(HAlign::Center);
///         button.set_v_align(VAlign::Center);
///         let button_path = window_context.abs_widget_path1(window_idx, |window: &mut ToplevelWindow| window.set(button))?;
///         Some(AppData {
///                 window_index: window_idx,
///                 button_path,
///         })
/// }, |window_context, app_data, _, _, _, _| {
///         window_context.window_mut::<ToplevelWindow>(app_data.window_index)?.set_on(move |client_context, _, event| {
///                 match event {
///                     Event::Close => client_context.exit(),
///                     _ => (),
///                 }
///                 Some(EventOption::Default)
///         });
///         Some(())
/// }).unwrap();
/// app.run().unwrap();
/// ```
pub struct App<T>
{
    client_display: ClientDisplay,
    client_context: Rc<RefCell<ClientContext>>,
    window_context: Arc<RwLock<WindowContext>>,
    queue_context: Arc<Mutex<QueueContext>>,
    thread_signal_sender: ThreadSignalSender,
    thread_signal_receiver: ThreadSignalReceiver,
    data: Arc<RwLock<T>>,
}

impl<T> App<T>
{
    /// Creates an application object.
    ///
    /// This method takes two arguments which are functions. The first function creates an
    /// application data. The second functions sets the application data, widgets, and windows. The
    /// first function that takes arguments:
    /// - a reference to the window context
    /// - a reference-counting pointer to the window context
    /// - a reference-counting pointer to the queue context
    /// - the thread signal sender
    ///
    /// The second function that takes arguments:
    /// - a reference to the window context
    /// - a reference to the application data
    /// - a reference-counting pointer to the window context
    /// - a reference-counting pointer to the queue context
    /// - the thread signal sender
    /// - a reference-counting pointer to the application data
    pub fn new<F, G>(creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, ThreadSignalSender) -> Option<T>,
              G: FnOnce(&mut WindowContext, &mut T, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, ThreadSignalSender, Arc<RwLock<T>>) -> Option<()>
    { Self::new_with_dyn_theme(theme_from_env()?, creating_f, setting_f) }

    /// Creates an application object with a theme.
    ///
    /// See [new](Self::new) for information about two last arguments.
    pub fn new_with_theme<U: Theme + 'static, F, G>(theme: U, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, ThreadSignalSender) -> Option<T>,
              G: FnOnce(&mut WindowContext, &mut T, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, ThreadSignalSender, Arc<RwLock<T>>) -> Option<()>
    { Self::new_with_dyn_theme(Box::new(theme), creating_f, setting_f) }
    
    /// Creates an application object with a dynamic theme.
    ///
    /// See [new](Self::new) for information about two last arguments.
    pub fn new_with_dyn_theme<F, G>(theme: Box<dyn Theme>, creating_f: F, setting_f: G) -> Result<Self, ClientError>
        where F: FnOnce(&mut WindowContext, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, ThreadSignalSender) -> Option<T>,
              G: FnOnce(&mut WindowContext, &mut T, Arc<RwLock<WindowContext>>, Arc<Mutex<QueueContext>>, ThreadSignalSender, Arc<RwLock<T>>) -> Option<()>
    {
        let window_context = Arc::new(RwLock::new(WindowContext::new(theme)));
        let window_context2 = window_context.clone();
        let queue_context2 = Arc::new(Mutex::new(QueueContext::new()));
        let (thread_signal_sender, thread_signal_receiver) = thread_signal_channel()?;
        let res2 = match window_context.write() {
            Ok(mut window_context_g) => {
                match creating_f(&mut *window_context_g, window_context2.clone(), queue_context2.clone(), thread_signal_sender) {
                    Some(tmp_data) => {
                        let data = Arc::new(RwLock::new(tmp_data));
                        let data2 = data.clone();
                        let res = match data.write() {
                            Ok(mut data_g) => {
                                match setting_f(&mut *window_context_g, &mut *data_g, window_context2.clone(), queue_context2.clone(), thread_signal_sender, data2.clone()) {
                                    Some(()) => {
                                        let (client_display, client_context) = ClientContext::new()?;
                                        let (thread_signal_sender, thread_signal_receiver) = thread_signal_channel()?;
                                        let app = App {
                                            client_display,
                                            client_context: Rc::new(RefCell::new(client_context)),
                                            window_context: window_context2,
                                            queue_context: queue_context2,
                                            thread_signal_sender,
                                            thread_signal_receiver,
                                            data: data2,
                                        };
                                        Ok(app)
                                    },
                                    None => {
                                        let _res1 = thread_signal_sender.close();
                                        let _res2 = thread_signal_receiver.close();
                                        Err(ClientError::Data)
                                    },
                                }
                            },
                            Err(_) => {
                                let _res1 = thread_signal_sender.close();
                                let _res2 = thread_signal_receiver.close();
                                Err(ClientError::RwLock)
                            },
                        };
                        res
                    },
                    None => {
                        let _res1 = thread_signal_sender.close();
                        let _res2 = thread_signal_receiver.close();
                        Err(ClientError::Data)
                    },
                }
            },
            Err(_) => {
                let _res1 = thread_signal_sender.close();
                let _res2 = thread_signal_receiver.close();
                Err(ClientError::RwLock)
            },
        };
        res2
    }

    /// Returns a reference-counting pointer to the client context.
    pub fn client_context(&self) -> Rc<RefCell<ClientContext>>
    { self.client_context.clone() }

    /// Returns a reference-counting pointer to the window context.
    pub fn window_context(&self) -> Arc<RwLock<WindowContext>>
    { self.window_context.clone() }

    /// Returns a reference-counting pointer to the queue context.
    pub fn queue_context(&self) -> Arc<Mutex<QueueContext>>
    { self.queue_context.clone() }

    /// Returns the thread signal sender.
    pub fn thread_signal_sender(&self) -> ThreadSignalSender
    { self.thread_signal_sender }
    
    /// Returns a reference-counting pointer to the application data.
    pub fn data(&self) -> Arc<RwLock<T>>
    { self.data.clone() }
    
    /// Runs the application.
    ///
    /// This method executes a main loop for a graphic thread. The graphic thread usually is a
    /// main thread. A main loop handles wayland events and thread signals in the graphic thread.
    pub fn run(&mut self) -> Result<(), ClientError>
    { run_main_loop(&mut self.client_display, self.client_context.clone(), self.window_context.clone(), self.queue_context.clone(), self.thread_signal_sender, self.thread_signal_receiver) }
}

impl<T> Drop for App<T>
{
    fn drop(&mut self)
    {
        let _res1 = self.thread_signal_sender.close();
        let _res2 = self.thread_signal_receiver.close();
    }
}
