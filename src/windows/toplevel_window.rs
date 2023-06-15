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
use crate::min_size::*;
use crate::preferred_size::*;
use crate::queue_context::*;
use crate::theme::*;
use crate::types::*;
use crate::utils::*;
use crate::widget::*;
use crate::window::*;
use crate::windows::child_index_set::*;

pub struct ToplevelWindow
{
    title: Option<String>,
    size: Size<i32>,
    padding_bounds: Rect<i32>,
    edges: Edges<i32>,
    corners: Corners<i32>,
    is_visible: bool,
    is_focused: bool,
    is_resizable: bool,
    change_flag_arc: Arc<AtomicBool>,
    is_moved: bool,
    resize_edges: Option<ClientResize>,
    min_size: Size<Option<i32>>,
    preferred_size: Size<Option<i32>>,
    child_index_set: ChildIndexSet,
    call_on_fun: CallOnFun,
    content: Option<Box<dyn Widget>>,
    focused_rel_widget_path: Option<RelWidgetPath>,
}

impl ToplevelWindow
{
    pub fn new() -> Option<Self>
    {
        let window = ToplevelWindow {
            title: None,
            size: Size::new(0, 0),
            padding_bounds: Rect::new(0, 0, 0, 0),
            edges: Edges::new(0, 0, 0, 0),
            corners: Corners::new(0, 0, 0, 0, 0, 0, 0, 0),
            is_visible: true,
            is_focused: false,
            is_resizable: true,
            change_flag_arc: Arc::new(AtomicBool::new(false)),
            is_moved: false,
            resize_edges: None,
            min_size: Size::new(None, None),
            preferred_size: Size::new(None, None),
            child_index_set: ChildIndexSet::new(),
            call_on_fun: CallOnFun::new(),
            content: None,
            focused_rel_widget_path: None,
        };
        Some(window)
    }
    
    pub fn set_title(&mut self, title: &str)
    {
        self.title = Some(String::from(title));
        self.change_flag_arc.store(true, Ordering::SeqCst);
    }
    
    pub fn unset_title(&mut self)
    {
        self.title = None;
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

    pub fn set_dyn(&mut self, mut widget: Box<dyn Widget>) -> Option<WidgetIndexPair>
    {
        widget.set_change_flag_arc(self.change_flag_arc.clone());
        self.content = Some(widget);
        self.change_flag_arc.store(true, Ordering::SeqCst);
        Some(WidgetIndexPair(1, 0))
    }

    pub fn set<T: Widget + 'static>(&mut self, widget: T) -> Option<WidgetIndexPair>
    { self.set_dyn(Box::new(widget)) }
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
    
    fn is_resizable(&self) -> bool
    { self.is_resizable }    

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
        self.is_moved = true;
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
        if self.content.is_some() {
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
    {
        match idx_pair {
            None => {
                if self.content.is_some() {
                    Some(WidgetIndexPair(1, 0))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    {
        match idx_pair {
            None => {
                if self.content.is_some() {
                    Some(WidgetIndexPair(1, 0))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    {
        if idx_pair == WidgetIndexPair(1, 0) {
            match &self.content {
                Some(widget) => Some(&**widget),
                None => None,
            }
        } else {
            None
        }
    }

    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    {
        if idx_pair == WidgetIndexPair(1, 0) {
            match &mut self.content {
                Some(widget) => Some(&mut **widget),
                None => None,
            }
        } else {
            None
        }
    }
    
    fn point_for_index_pair(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    {
        match &self.content {
            Some(widget) => {
                if widget.bounds().to_f64_rect().contains(pos) {
                    Some(WidgetIndexPair(1, 0))
                } else {
                    None
                }
            },
            None => None,
        }
    }
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
        match &mut self.content {
            Some(content) => {
                content.update_size(cairo_context, theme, area_size)?;
                self.padding_bounds.set_size(content.margin_size());
            },
            None => self.padding_bounds.set_size(Size::new(1, 1)),
        }
        self.size = outer_size(self.padding_bounds.size(), self.edges);
        self.size = size_for_opt_size(self.size, self.preferred_size);
        self.padding_bounds.set_size(inner_size(self.size, self.edges));
        match &mut self.content {
            Some(content) => {
                if content.width() > self.padding_bounds.width || content.height() > self.padding_bounds.height || content.h_align() == HAlign::Fill || content.v_align() == VAlign::Fill {
                    let area_size2 = Size::new(Some(self.padding_bounds.width), Some(self.padding_bounds.height));
                    content.update_size(cairo_context, theme, area_size2)?;
                }
            },
            None => (),
        }
        Ok(())
    }
    
    fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>
    {
        self.padding_bounds.set_pos(inner_pos(area_bounds, self.edges));
        match &mut self.content {
            Some(content) => content.update_pos(cairo_context, theme, self.padding_bounds)?,
            None => (),
        }
        Ok(())
    }

    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>
    { 
        theme.draw_toplevel_window_content_bg(cairo_context, Rect::new(0, 0, self.size.width, self.size.height), is_focused_window)?;
        match &self.content {
            Some(content) => content.draw(cairo_context, theme, self.is_focused)?,
            None => (),
        }
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
