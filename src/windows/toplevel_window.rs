//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use crate::as_any::*;
use crate::call_on::*;
use crate::client_context::*;
use crate::container::*;
use crate::draw::*;
use crate::events::*;
use crate::image::*;
use crate::min_size::*;
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;
use crate::widgets::*;
use crate::window::*;
use crate::windows::child_index_set::*;
use crate::windows::two_window_widgets::*;

pub struct ToplevelWindow
{
    title: Option<String>,
    size: Size<i32>,
    padding_bounds: Rect<i32>,
    edges: Edges<i32>,
    corners: Corners<i32>,
    is_visible: bool,
    is_focused: bool,
    is_maximized: bool,
    is_resizable: bool,
    change_flag_arc: Arc<AtomicBool>,
    is_moved: bool,
    resize_edges: Option<ClientResize>,
    min_size: Size<Option<i32>>,
    preferred_size: Size<Option<i32>>,
    child_index_set: ChildIndexSet,
    call_on_fun: CallOnFun,
    widgets: TwoWindowWidgets,
    focused_rel_widget_path: Option<RelWidgetPath>,
    menu_button_path: Option<RelWidgetPath>,
    title_path: Option<RelWidgetPath>,
    maximize_button_path: Option<RelWidgetPath>,
    close_button_path: Option<RelWidgetPath>,
}

impl ToplevelWindow
{
    pub fn new() -> Option<Self>
    {
        let mut window = ToplevelWindow {
            title: None,
            size: Size::new(0, 0),
            padding_bounds: Rect::new(0, 0, 0, 0),
            edges: Edges::new(0, 0, 0, 0),
            corners: Corners::new(0, 0, 0, 0, 0, 0, 0, 0),
            is_visible: true,
            is_focused: false,
            is_maximized: false,
            is_resizable: true,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            is_moved: false,
            resize_edges: None,
            min_size: Size::new(None, None),
            preferred_size: Size::new(None, None),
            child_index_set: ChildIndexSet::new(),
            call_on_fun: CallOnFun::new(),
            widgets: TwoWindowWidgets::new(),
            focused_rel_widget_path: None,
            menu_button_path: None,
            title_path: None,
            maximize_button_path: None,
            close_button_path: None,
        };
        let title_bar = TitleBar::new();
        let title_bar_path = container_rel_widget_path1(&mut window, |window: &mut ToplevelWindow| window.set_title_bar(title_bar))?;
        let mut menu_button = TitleButton::new(TitleButtonIcon::Menu);
        menu_button.set_enabled(false);
        let menu_button_path = container_rel_widget_path(&mut window, &title_bar_path, |title_bar: &mut TitleBar| title_bar.add(menu_button))?;
        let title = Title::new("");
        let title_path = container_rel_widget_path(&mut window, &title_bar_path, |title_bar: &mut TitleBar| title_bar.add(title))?;
        let maximize_button = TitleButton::new(TitleButtonIcon::Maximize);
        let maximize_button_path = container_rel_widget_path(&mut window, &title_bar_path, |title_bar: &mut TitleBar| title_bar.add(maximize_button))?;
        let close_button = TitleButton::new(TitleButtonIcon::Close);
        let close_button_path = container_rel_widget_path(&mut window, &title_bar_path, |title_bar: &mut TitleBar| title_bar.add(close_button))?;
        let title: &mut Title = container_widget_mut(&mut window, &title_path)?;
        title.set_on(move |client_context, queue_context, event| {
                match event {
                     Event::Client(ClientEvent::PointerButton(_, ClientButton::Left, ClientState::Pressed)) |
                     Event::Client(ClientEvent::TouchDown(_, _, _)) => {
                         client_context.stop_button_timer();
                         client_context.stop_touch_timer();
                         let current_window_idx = queue_context.current_call_on_path()?.window_index();
                         queue_context.push_callback(move |_, window_context, _| {
                                 window_context.dyn_window_mut(current_window_idx)?._move();
                                 Some(())
                         });
                    }
                    _ => (),
                }
                Some(EventOption::Default)
        });
        let maximize_button: &mut TitleButton = container_widget_mut(&mut window, &maximize_button_path)?;
        maximize_button.set_on(move |_, _, event| {
                match event {
                    Event::Click | Event::DoubleClick | Event::LongClick => Some(EventOption::Some(Event::Maximize)),
                    _ => Some(EventOption::Default),
                }
        });
        let close_button: &mut TitleButton = container_widget_mut(&mut window, &close_button_path)?;
        close_button.set_on(move |_, _, event| {
                match event {
                    Event::Click | Event::DoubleClick | Event::LongClick => Some(EventOption::Some(Event::Close)),
                    _ => Some(EventOption::Default),
                }
        });
        window.set_menu_button_path(Some(menu_button_path));
        window.set_title_path(Some(title_path));
        window.set_maximize_button_path(Some(maximize_button_path));
        window.set_maximize_button_path(Some(close_button_path));
        Some(window)
    }
    
    pub fn set_title(&mut self, title: &str)
    {
        self.title = Some(String::from(title));
        match self.title_path.clone() {
            Some(title_path) => {
                let title_widget: Option<&mut Title> = container_widget_mut(self, &title_path);
                match title_widget {
                    Some(title_widget) => title_widget.set_text(title),
                    None => (),
                }
            },
            None => {
            },
        }
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
    
    pub fn unset_title(&mut self)
    {
        self.title = None;
        match self.title_path.clone() {
            Some(title_path) => {
                let title_widget: Option<&mut Title> = container_widget_mut(self, &title_path);
                match title_widget {
                    Some(title_widget) => title_widget.set_text(""),
                    None => (),
                }
            },
            None => {
            },
        }
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }

    pub fn set_visible(&mut self, is_visible: bool)
    {
        let old_visible_flag = self.is_visible;
        self.is_visible = is_visible;
        if old_visible_flag != self.is_visible {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
        self.reset_descendant_states();
    }
    
    pub fn set_resizeble(&mut self, is_resizable: bool)
    {
        let old_resizable_flag = self.is_resizable;
        self.is_resizable = is_resizable;
        if old_resizable_flag != self.is_resizable {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn set_dyn_on(&mut self, f: Box<dyn FnMut(&mut ClientContext, &mut QueueContext, &Event) -> Option<EventOption> + Send + Sync + 'static>)
    { self.call_on_fun.fun = f; }

    pub fn set_on<F>(&mut self, f: F)
        where F: FnMut(&mut ClientContext, &mut QueueContext, &Event) -> Option<EventOption> + Send + Sync + 'static
    { self.set_dyn_on(Box::new(f)) }

    pub fn has_trimmed_width(&self) -> bool
    { self.widgets.has_trimmed_width }
    
    pub fn set_trimmed_width(&mut self, is_trimmed_width: bool)
    {
        let old_trimmed_width_flag = self.widgets.has_trimmed_width;
        self.widgets.has_trimmed_width = is_trimmed_width;
        if old_trimmed_width_flag != self.widgets.has_trimmed_width {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }

    pub fn has_trimmed_height(&self) -> bool
    { self.widgets.has_trimmed_height }

    pub fn set_trimmed_height(&mut self, is_trimmed_height: bool)
    {
        let old_trimmed_height_flag = self.widgets.has_trimmed_height;
        self.widgets.has_trimmed_height = is_trimmed_height;
        if old_trimmed_height_flag != self.widgets.has_trimmed_height {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
    
    pub fn set_dyn_title_bar(&mut self, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        self.widgets.title_bar = Some(widget);
        self.change_flag_arc.store(true, Ordering::SeqCst);
        Some(WidgetIndexPair(0, 0))
    }

    pub fn set_title_bar<T: Widget + 'static>(&mut self, widget: T) -> Option<WidgetIndexPair>
    { self.set_dyn_title_bar(Box::new(widget)) }

    pub fn unset_title_bar(&mut self) -> Option<Box<dyn Widget>>
    {
        let title_bar = self.widgets.title_bar.take();
        self.change_flag_arc.store(true, Ordering::SeqCst);
        title_bar
    }    
    
    pub fn set_dyn(&mut self, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        self.widgets.content = Some(widget);
        self.change_flag_arc.store(true, Ordering::SeqCst);
        Some(WidgetIndexPair(1, 0))
    }

    pub fn set<T: Widget + 'static>(&mut self, widget: T) -> Option<WidgetIndexPair>
    { self.set_dyn(Box::new(widget)) }
    
    pub fn unset(&mut self) -> Option<Box<dyn Widget>>
    {
        let content = self.widgets.content.take();
        self.change_flag_arc.store(true, Ordering::SeqCst);
        content
    }
    
    pub fn menu_button_path(&self) -> Option<&RelWidgetPath>
    {
        match &self.menu_button_path {
            Some(path) => Some(path),
            None => None,
        }
    }
    
    pub fn set_menu_button_path(&mut self, path: Option<RelWidgetPath>)
    { self.menu_button_path = path; }

    pub fn title_path(&self) -> Option<&RelWidgetPath>
    {
        match &self.title_path {
            Some(path) => Some(path),
            None => None,
        }
    }
    
    pub fn set_title_path(&mut self, path: Option<RelWidgetPath>)
    { self.title_path = path; }

    pub fn maximize_button_path(&self) -> Option<&RelWidgetPath>
    {
        match &self.maximize_button_path {
            Some(path) => Some(path),
            None => None,
        }
    }
    
    pub fn set_maximize_button_path(&mut self, path: Option<RelWidgetPath>)
    { self.maximize_button_path = path; }

    pub fn close_button_path(&self) -> Option<&RelWidgetPath>
    {
        match &self.close_button_path {
            Some(path) => Some(path),
            None => None,
        }
    }
    
    pub fn set_close_button_path(&mut self, path: Option<RelWidgetPath>)
    { self.close_button_path = path; }
}

impl Window for ToplevelWindow
{
    fn size(&self) -> Size<i32>
    { self.size }

    fn padding_bounds(&self) -> Rect<i32>
    { self.padding_bounds }

    fn edges(&self) -> Edges<i32>
    { self.edges }

    fn corners(&self) -> Corners<i32>
    { self.corners }

    fn is_visible(&self) -> bool
    { self.is_visible }
    
    fn is_focused(&self) -> bool
    { self.is_focused }
    
    fn set_focus(&mut self, is_focused: bool) -> bool
    {
        let old_focus_flag = self.is_focused;
        self.is_focused = is_focused;
        if old_focus_flag != self.is_focused {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
        true
    }

    fn title(&self) -> Option<&str>
    {
        match &self.title {
            Some(title) => Some(title.as_str()),
            None => None,
        }
    }

    fn is_maximizable(&self) -> bool
    { true }

    fn is_maximized(&self) -> bool
    { self.is_maximized }
    
    fn set_maximized(&mut self, is_maximized: bool) -> bool
    { 
        let old_maximized_flag = self.is_maximized;
        self.is_maximized = is_maximized;
        if old_maximized_flag != self.is_maximized {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
        true
    }
    
    fn is_moveable(&self) -> bool
    { true }
    
    fn is_resizable(&self) -> bool
    { !self.is_maximized && self.is_resizable }

    fn is_changed(&self) -> bool
    { self.change_flag_arc.load(Ordering::SeqCst) }
    
    fn clear_change_flag(&mut self)
    { self.change_flag_arc.store(false, Ordering::SeqCst); }

    fn is_moved(&self) -> bool
    { self.is_moved }

    fn _move(&mut self) -> bool
    {
        self.is_moved = true;
        true
    }

    fn clear_move_flag(&mut self) -> bool
    {
        self.is_moved = false;
        true
    }

    fn resize_edges(&self) -> Option<ClientResize>
    { self.resize_edges }

    fn resize(&mut self, edges: ClientResize) -> bool
    {
        self.resize_edges = Some(edges);
        true
    }
    
    fn clear_resize_edges(&mut self) -> bool
    {
        self.resize_edges = None;
        true
    }

    fn content_index_pair(&self) -> Option<WidgetIndexPair>
    {
        if self.widgets.content.is_some() {
            Some(WidgetIndexPair(1, 0))
        } else {
            None
        }
    }
    
    fn child_index_iter(&self) -> Option<Box<dyn WindowIterator + '_>>
    { self.child_index_set.child_index_iter() }

    fn add_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    { self.child_index_set.add(idx) }

    fn remove_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    { self.child_index_set.remove(idx) }
    
    fn focused_rel_widget_path(&self) -> Option<&RelWidgetPath>
    { 
        match &self.focused_rel_widget_path {
            Some(rel_widget_path) => Some(rel_widget_path),
            None => None,
        }
    }

    fn set_only_focused_rel_widget_path(&mut self, rel_widget_path: Option<RelWidgetPath>) -> bool
    {
        self.focused_rel_widget_path = rel_widget_path;
        true
    }    
}

impl Container for ToplevelWindow
{
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { self.widgets.prev(idx_pair) }

    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { self.widgets.next(idx_pair) }

    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    { self.widgets.dyn_widget(idx_pair) }

    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    { self.widgets.dyn_widget_mut(idx_pair) }
    
    fn point_for_index_pair(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    { self.widgets.point(pos) }
}

impl MinSize for ToplevelWindow
{
    fn min_size(&self) -> Size<Option<i32>>
    { self.min_size }
    
    fn set_min_size(&mut self, size: Size<Option<i32>>)
    {
        let old_min_size = self.min_size;
        self.min_size = size;
        if old_min_size != self.min_size {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
}

impl PreferredSize for ToplevelWindow
{
    fn preferred_size(&self) -> Size<Option<i32>>
    { self.preferred_size }
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>)
    {
        let old_preferred_size = self.preferred_size;
        self.preferred_size = size;
        if old_preferred_size != self.preferred_size {
            self.change_flag_arc.store(true, Ordering::SeqCst);
        }
    }
}

impl Draw for ToplevelWindow
{
    fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>
    {
        self.edges = theme.toplevel_window_edges();
        self.corners = theme.toplevel_window_corners();
        let padding_area_size = inner_opt_size(area_size, self.edges);
        self.widgets.update_size(cairo_context, theme, padding_area_size)?;
        self.padding_bounds.set_size(self.widgets.padding_size(padding_area_size));
        self.size = outer_size(self.padding_bounds.size(), self.edges);
        Ok(())
    }
    
    fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>
    {
        self.padding_bounds.set_pos(inner_pos(area_bounds, self.edges));
        self.widgets.update_pos(cairo_context, theme, inner_rect(area_bounds, self.edges))?;
        Ok(())
    }

    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    { 
        let mut is_title_bar = false;
        let mut bounds = Rect::new(0, 0, self.size.width, self.size.height);
        match &self.widgets.title_bar {
            Some(title_bar) => {
                theme.draw_toplevel_window_title_bar_bg(cairo_context, Rect::new(0, 0, self.size.width, title_bar.margin_y() + title_bar.margin_height()), is_focused_window)?;
                bounds.y += title_bar.margin_y() + title_bar.margin_height();
                bounds.height -= title_bar.margin_y() + title_bar.margin_height(); 
                is_title_bar = true;
            },
            None => (),
        }
        theme.draw_toplevel_window_content_bg(cairo_context, bounds, is_focused_window, is_title_bar)?;
        self.widgets.draw(cairo_context, theme, is_focused_window)?;
        Ok(())
    }
}

impl CallOn for ToplevelWindow
{
    fn call_on(&mut self, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Event>>
    {
        let default_event = if let Some(tmp_default_event) = default_window_on(self, client_context, queue_context, event)? {
            tmp_default_event
        } else {
            None
        };
        self.call_on_fun.call_on(client_context, queue_context, event, default_event)
    }
}

impl AsAny for ToplevelWindow
{
    fn as_any(&self) -> &dyn Any
    { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any
    { self }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;

    #[test]
    fn test_two_window_widgets_update_size_and_position()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        let mut theme = MockTheme::new();
        theme.set_font_size(32.0);
        theme.set_toplevel_window_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_margin_edges(Edges::new(0, 0, 0, 0));
        theme.set_title_padding_edges(Edges::new(4, 4, 2, 2));
        theme.set_title_font_size(16.0);
        theme.set_title_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_title_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_title_button_icon_size(Size::new(12, 12));
        theme.set_button_margin_edges(Edges::new(2, 2, 2, 2));
        theme.set_button_padding_edges(Edges::new(4, 4, 4, 4));
        theme.set_button_font_size(16.0);
        let mut toplevel_window = ToplevelWindow::new().unwrap();
        toplevel_window.set_title("T");
        let mut button = Button::new("B");
        button.set_preferred_size(Size::new(Some(120), Some(60)));
        toplevel_window.set(button);
        theme.set_title_font(&cairo_context).unwrap();
        let font_height = cairo_context.font_extents().unwrap().height;
        theme.set_cairo_context(&cairo_context, 1).unwrap();
        let area_size = Size::new(None, None);
        match toplevel_window.update_size(&cairo_context, &theme, area_size) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_padding_width = 124;
        let expected_padding_height = (font_height.ceil() as i32) + 8 + 64;
        assert_eq!(Size::new(expected_padding_width, expected_padding_height), toplevel_window.padding_bounds.size());
        let expected_width = expected_padding_width + 8;
        let expected_height = expected_padding_height + 8;
        assert_eq!(Size::new(expected_width, expected_height), toplevel_window.size);
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), toplevel_window.widgets.title_bar.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(124, (font_height.ceil() as i32) + 8), toplevel_window.widgets.title_bar.as_ref().unwrap().size());
        assert_eq!(Size::new(124, 64), toplevel_window.widgets.content.as_ref().unwrap().margin_size());
        assert_eq!(Size::new(120, 60), toplevel_window.widgets.content.as_ref().unwrap().size());
        let padding_size = toplevel_window.widgets.padding_size(area_size);
        let area_bounds = outer_rect(Rect::new(4, 4, padding_size.width, padding_size.height), Edges::new(4, 4, 4, 4));
        match toplevel_window.update_pos(&cairo_context, &theme, area_bounds) {
            Ok(()) => (),
            Err(_) => assert!(false),
        }
        let expected_padding_x = 4;
        let expected_padding_y = 4;
        assert_eq!(Pos::new(expected_padding_x, expected_padding_y), toplevel_window.padding_bounds.pos());
        assert_eq!(Size::new(expected_padding_width, expected_padding_height), toplevel_window.padding_bounds.size());
        assert_eq!(Size::new(expected_width, expected_height), toplevel_window.size);
        assert_eq!(Pos::new(4, 4), toplevel_window.widgets.title_bar.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(4, 4), toplevel_window.widgets.title_bar.as_ref().unwrap().pos());
        assert_eq!(Pos::new(4, 4 + (font_height.ceil() as i32) + 8), toplevel_window.widgets.content.as_ref().unwrap().margin_pos());
        assert_eq!(Pos::new(4 + 2, 4 + (font_height.ceil() as i32) + 8 + 2), toplevel_window.widgets.content.as_ref().unwrap().pos());
    }
}
