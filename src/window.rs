//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use crate::container::*;
use crate::events::*;
use crate::min_size::*;
use crate::preferred_size::*;
use crate::types::*;
use crate::widget::*;

/// A structure of self window index.
///
/// The self window index can't be directly created by an application. This structure is used to
/// automatically set the window index.
#[derive(Copy, Clone, Debug)]
pub struct SelfWindowIndex(WindowIndex);

impl SelfWindowIndex
{
    pub(crate) fn new(idx: WindowIndex) -> SelfWindowIndex
    { SelfWindowIndex(idx) }
    
    /// Return the window index.
    pub fn window_index(&self) -> WindowIndex
    { self.0 }
}


/// A structure of self window tag.
///
/// The self window tag can't be directly created by an application. This structure is used to
/// automatically unset the window index.
#[derive(Copy, Clone, Debug)]
pub struct SelfWindowTag(());

impl SelfWindowTag
{
    pub(crate) fn new() -> SelfWindowTag
    { SelfWindowTag(()) }    
}

/// A structure of parent window index.
///
/// The parent window index can't be directly created by an application. This structure is used to
/// automatically set the parent window.
#[derive(Copy, Clone, Debug)]
pub struct ParentWindowIndex(WindowIndex);

impl ParentWindowIndex
{
    pub(crate) fn new(idx: WindowIndex) -> ParentWindowIndex
    { ParentWindowIndex(idx) }
    
    /// Returns the window index.
    pub fn window_index(&self) -> WindowIndex
    { self.0 }
}

/// A structure of parent window tag.
///
/// The parent window tag can't be directly created by an application. This structure is used to
/// automatically unset the parent window.
#[derive(Copy, Clone, Debug)]
pub struct ParentWindowTag(());

impl ParentWindowTag
{
    pub(crate) fn new() -> ParentWindowTag
    { ParentWindowTag(()) }    
}

/// A structure of child window index.
///
/// The child window index can't be directly created by an application. This structure is used to
/// automatically add and remove the child window.
#[derive(Copy, Clone, Debug)]
pub struct ChildWindowIndex(WindowIndex);

impl ChildWindowIndex
{
    pub(crate) fn new(idx: WindowIndex) -> ChildWindowIndex
    { ChildWindowIndex(idx) }

    /// Returns the window index.
    pub fn window_index(&self) -> WindowIndex
    { self.0 }
}

struct StackElem<'a>
{
    widget: &'a dyn Widget,
    widget_index_pair: Option<WidgetIndexPair>,
}

impl<'a> StackElem<'a>
{
    fn new(widget: &'a dyn Widget) -> Self
    { StackElem { widget, widget_index_pair: None, } }
}

/// A window trait.
///
/// The windows are for example toplevel windows and message windows. A widget of title bar and a
/// content widget can be in the window. The window is drawn with descendant widgets and reacts on
/// events if the window is visible.
pub trait Window: Container + MinSize + PreferredSize
{
    /// Returns the size of the window.
    fn size(&self) -> Size<i32>;

    /// Returns the padding bounds of the window.
    fn padding_bounds(&self) -> Rect<i32>;

    /// Returns the egdes of the window. 
    fn edges(&self) -> Edges<i32>;

    /// Returns the corners of the window.
    fn corners(&self) -> Corners<i32>;
    
    /// Returns `true` if the window is visible, otherwise `false`.
    fn is_visible(&self) -> bool;

    /// Returns `true` if the window is focusable, otherwise `false`.
    ///
    /// This method defaultly returns `true`.
    fn is_focusable(&self) -> bool
    { true }
    
    /// Returns `true` if the window is focused, otherwise `false`.
    fn is_focused(&self) -> bool;
    
    /// Sets the focus of the window if the window is focusable.
    ///
    /// This method should return `true` if the window is focusable, otherwise `false`.
    fn set_focus(&mut self, is_focused: bool) -> bool;

    /// Returns a slice of the title.
    fn title(&self) -> Option<&str>
    { None }
    
    /// Returns `true` if the window is popup, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_popup(&self) -> bool
    { false }
    
    /// Returns `true` if the window is transient, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_transient(&self) -> bool
    { false }
    
    /// Returns `true` if the window is maximizable, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_maximizable(&self) -> bool
    { false }
    
    /// Returns `true` if the window is maximized, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_maximized(&self) -> bool
    { false }
    
    /// Sets the maximized if the window is maximizable.
    ///
    /// This method should return `true` if the window is maximizable, otherwise `false`. This
    /// method defaultly returns `false`.
    #[allow(unused_variables)]
    fn set_maximized(&mut self, is_maximized: bool) -> bool
    { false }

    /// Maixmizes the window if the window is maximizable.
    ///
    /// This method should return `true` if the window is maximizable, otherwise `false`.
    fn maximize(&mut self) -> bool
    { self.set_maximized(true) }
    
    /// Unmaixmizes the window if the window is maximizable.
    ///
    /// This method should return `true` if the window is maximizable, otherwise `false`.
    fn unmaximize(&mut self) -> bool
    { self.set_maximized(false) }

    /// Returns `true` if the window is moveable, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_moveable(&self) -> bool
    { false }

    /// Returns `true` if the window is resizeble, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_resizable(&self) -> bool
    { false }    

    /// Sets the window index for the window.
    #[allow(unused_variables)]
    fn set_index(&mut self, idx: SelfWindowIndex)
    {}

    /// Unsets the window index for the window.
    fn unset_index(&mut self, _tag: SelfWindowTag)
    {}
    
    /// Returns the parent window index or `None`.
    ///
    /// This method defaultly returns `None`.
    fn parent_index(&self) -> Option<WindowIndex>
    { None }
    
    /// Returns the position in the parent window or `None`.
    ///
    /// This method defaultly returns `None`.
    fn pos_in_parent(&self) -> Option<Pos<i32>>
    { None }

    /// Sets the parent with the position in the parent.
    ///
    /// This method should return `Some(())` if the parent is set, otherwise `None` for an error.
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn set_parent(&mut self, idx: ParentWindowIndex, pos: Pos<i32>) -> Option<()>
    { None }

    /// Unsets the parent window.
    ///
    /// This method should return `Some(())` if the parent is unset, otherwise `None` for an error.
    /// This method defaultly returns `None`.
    fn unset_parent(&mut self, _tag: ParentWindowTag) -> Option<()>
    { None }

    /// Returns a window iterator that iterates over child windows or `None`.
    ///
    /// This method defaultly returns `None`.
    fn child_index_iter(&self) -> Option<Box<dyn WindowIterator + '_>>
    { None }
    
    /// Adds a child to the window.
    ///
    /// This method should return `Some(())` if the child is added, otherwise `None` for an error.
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn add_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    { None }

    /// Removes the child to the window.
    ///
    /// This method should return `Some(())` if the child is removed, otherwise `None` for an
    /// error. This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn remove_child(&mut self, idx: ChildWindowIndex) -> Option<()>
    { None }
    
    /// Returns `true` if the window is changed, otherwise `false`.
    ///
    /// If the window is changed, the window is redrawn.
    fn is_changed(&self) -> bool;
    
    /// Clears the change flag of the window.
    ///
    /// This method is called after redraw the window.
    fn clear_change_flag(&mut self);

    /// Returns `true` if the window is moved, otherwise `false`.
    ///
    /// This method defaultly returns `false`.
    fn is_moved(&self) -> bool
    { false }

    /// Moves the window if the window is moveable.
    ///
    /// This method should return `true` if the window is moveable, otherwise `false`. This method
    /// defaultly returns `false`.
    fn _move(&mut self) -> bool
    { false }
    
    /// Clears the move flag of the window if the window is moveable.
    ///
    /// This method should return `true` if the window is moveable, otherwise `false`. This method
    /// defaultly returns `false`.
    fn clear_move_flag(&mut self) -> bool
    { false }

    /// Returns the resize egdes of the window if the window has the resize edges, otherwise `None`.
    ///
    /// This method defaultly returns `None`.
    fn resize_edges(&self) -> Option<ClientResize>
    { None }

    /// Resizes the window if the window is resizable.
    ///
    /// This method should return `true` if the window is resizable, otherwise `false`. This method
    /// defaultly returns `false`.
    #[allow(unused_variables)]
    fn resize(&mut self, edges: ClientResize) -> bool
    { false }
    
    /// Clears the resize egdes of the window if the window is resizable.
    ///
    /// This method should return `true` if the window is resizable, otherwise `false`. This method
    /// defaultly returns `false`.
    fn clear_resize_edges(&mut self) -> bool
    { false }    
    
    /// Returns the pair of widget indices of the content if the window has the content, otherwise
    /// `None`.
    ///
    /// This method defaultly returns `None`.
    fn content_index_pair(&self) -> Option<WidgetIndexPair>
    { None }

    /// Returns a reference to the focused relative widget path if the window has the focused
    /// widget, otherwise `None`.
    ///
    /// This method defaultly returns `None`.
    fn focused_rel_widget_path(&self) -> Option<&RelWidgetPath>
    { None }

    /// Sets only the focused relative widget path.
    ///
    /// This method should return `true` if the focused relative widget path is set, otherwise
    /// `false`. Also, this method doesn't check widgets and set the widget focuses. A default
    /// result of this method is `false`. This method shouldn't be directly used by an application.
    #[allow(unused_variables)]
    fn set_only_focused_rel_widget_path(&mut self, rel_widget_path: Option<RelWidgetPath>) -> bool
    { false }    
    
    /// Returns an iterator that iterates over the child window indices.
    fn child_indices(&self) -> ChildWindowIndices<'_>
    { ChildWindowIndices::new(self.child_index_iter()) }
    
    /// Returns the width of the window.
    fn width(&self) -> i32
    { self.size().width }

    /// Returns the height of the window.
    fn height(&self) -> i32
    { self.size().height }

    /// Returns the padding position of the window.
    fn padding_pos(&self) -> Pos<i32>
    { self.padding_bounds().pos() }

    /// Returns the padding size of the window.
    fn padding_size(&self) -> Size<i32>
    { self.padding_bounds().size() }

    /// Returns the padding X coordinate of the window.
    fn padding_x(&self) -> i32
    { self.padding_bounds().x }

    /// Returns the padding Y coordinate of the window.
    fn padding_y(&self) -> i32
    { self.padding_bounds().y }

    /// Returns the padding width of the window.
    fn padding_width(&self) -> i32
    { self.padding_bounds().width }

    /// Returns the padding height of the window.
    fn padding_height(&self) -> i32
    { self.padding_bounds().height }
    
    /// Sets the focused relative widget path.
    ///
    /// This method returns `true` if the focused relative widget path is set, otherwise `false`.
    /// Also, this method checks widgets and sets the widget focuses. This method should be
    /// directly used by an application instead the 
    /// [`set_only_focused_rel_widget_path`](Self::set_only_focused_rel_widget_path) method.
    fn set_focused_rel_widget_path(&mut self, rel_widget_path: Option<RelWidgetPath>) -> bool
    {
        let saved_old_rel_widget_path = match self.focused_rel_widget_path().map(|rwp| rwp.clone()) {
            Some(old_rel_widget_path) => {
                match self.dyn_widget_mut(&old_rel_widget_path) {
                    Some(old_widget) => {
                        old_widget.set_focus(false);
                    },
                    None => (),
                }
                Some(old_rel_widget_path.clone())
            },
            None => None,
        };
        if self.set_only_focused_rel_widget_path(rel_widget_path) {
            match self.focused_rel_widget_path().map(|rwp| rwp.clone()) {
                Some(new_rel_widget_path) => {
                    let is_new_widget = match self.dyn_widget_mut(&new_rel_widget_path) {
                        Some(new_widget) => new_widget.set_focus(true),
                        None => false,
                    };
                    if !is_new_widget {
                        self.set_only_focused_rel_widget_path(saved_old_rel_widget_path);
                        match self.focused_rel_widget_path().map(|rwp| rwp.clone()) {
                            Some(old_rel_widget_path) => {
                                match self.dyn_widget_mut(&old_rel_widget_path) {
                                    Some(old_widget) => {
                                        old_widget.set_focus(true);
                                    },
                                    None => (),
                                }
                            },
                            None => (),
                        }
                        false
                    } else {
                        true
                    }
                },
                None => true,
            }
        } else {
            false
        }
    }

    /// Updates the focused relative widget path.
    ///
    /// This method returns `true` if the focused relative widget path is updated, otherwise
    /// `false`. If the focused relative widget path refers a non-existent widget, this method
    /// unsets the focused relative widget path.
    fn update_focused_rel_widget_path(&mut self) -> bool
    {
        let is_widget = match self.focused_rel_widget_path() {
            Some(rel_widget_path) => self.dyn_widget(rel_widget_path).is_some(),
            None => true,
        };
        if !is_widget {
            self.set_only_focused_rel_widget_path(None)
        } else {
            true
        }
    }
    
    /// Returns a reference of the dynamic focused widget or `None`.
    fn dyn_focused_widget(&self) -> Option<&dyn Widget>
    {
        match self.focused_rel_widget_path().map(|rwp| rwp.clone()) {
            Some(rel_widget_path) => self.dyn_widget(&rel_widget_path),
            None => None,
        }
    }

    /// Returns a mutable reference of the dynamic focused widget or `None`.
    fn dyn_focused_widget_mut(&mut self) -> Option<&mut dyn Widget>
    {
        match self.focused_rel_widget_path().map(|rwp| rwp.clone()) {
            Some(rel_widget_path) => self.dyn_widget_mut(&rel_widget_path),
            None => None,
        }
    }    

    /// Returns the previous or next focused relative widget path, `Some(None)` for no the previous
    /// or next focused relative widget path, or `None` for an error.
    fn prev_or_next_focused_widget(&self, dir: Dir, is_down: bool) -> Option<Option<RelWidgetPath>>
    {
        let mut stack: Vec<StackElem<'_>> = Vec::new();
        let (first_idx_pair, is_stop_for_none) = match self.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let widget = self.dyn_widget(rel_widget_path)?;
                if !widget.is_focusable() {
                    return None; 
                }
                for idx_pair in rel_widget_path.widget_index_pairs() {
                    let widget = match stack.last_mut() {
                        Some(elem) => {
                            elem.widget_index_pair = Some(idx_pair);
                            elem.widget.dyn_widget_for_index_pair(idx_pair)?
                        },
                        None => self.dyn_widget_for_index_pair(idx_pair)?,
                    };
                    stack.push(StackElem::new(widget));
                }
                if !is_down {
                    if stack.len() > 1 {
                        stack.pop();
                    } else {
                        return Some(Some(RelWidgetPath::new(rel_widget_path.widget_index_pairs().next()?)));
                    }
                }
                (rel_widget_path.widget_index_pairs().next()?, is_down)
            },
            None => {
                match self.content_index_pair() {
                    Some(idx_pair) => {
                        let widget = self.dyn_widget_for_index_pair(idx_pair)?;
                        if widget.is_focusable() {
                            return Some(Some(RelWidgetPath::new(idx_pair)));
                        } else {
                            stack.push(StackElem::new(widget));
                            (idx_pair, true)
                        }
                    },
                    None => return Some(None),
                }
            },
        };
        let mut is_path = true;
        loop {
            let stack_len = stack.len();
            match stack.last_mut() {
                Some(elem) => {
                    match elem.widget.prev_or_next(elem.widget_index_pair, dir) {
                        Some(idx_pair) => {
                            let widget = elem.widget.dyn_widget_for_index_pair(idx_pair)?;
                            elem.widget_index_pair = Some(idx_pair);
                            if widget.is_focusable() {
                                break;
                            } else {
                                stack.push(StackElem::new(widget));
                            }
                        },
                        None => {
                            if elem.widget.is_focusable() || stack_len == 1 {
                                if is_stop_for_none {
                                    if elem.widget.is_focusable() {
                                        stack.pop();
                                    } else {
                                        is_path = false;
                                    }
                                    break;
                                } else {
                                    elem.widget_index_pair = None;
                                }
                            } else {
                                stack.pop();
                            }
                        },
                    }
                },
                None => break,
            }
        }
        if is_path {
            let mut rel_widget_path = RelWidgetPath::new(first_idx_pair);
            for elem in &stack {
                rel_widget_path.push(elem.widget_index_pair?);
            }
            Some(Some(rel_widget_path))
        } else {
            Some(None)
        }
    }
    
    /// Sets the previous focused widget.
    ///
    /// This method returns `Some(())` if the previous focused widget is set, otherwise `None` for
    /// an error.
    fn prev_focused_widget(&mut self) -> Option<()>
    {
        if self.set_focused_rel_widget_path(self.prev_or_next_focused_widget(Dir::Prev, false)?) {
            Some(())
        } else {
            None
        }
    }

    /// Sets the next focused widget.
    ///
    /// This method returns `Some(())` if the next focused widget is set, otherwise `None` for an
    /// error.
    fn next_focused_widget(&mut self) -> Option<()>
    {
        if self.set_focused_rel_widget_path(self.prev_or_next_focused_widget(Dir::Next, false)?) {
            Some(())
        } else {
            None
        }
    }
    
    /// Sets the up focused widget.
    ///
    /// This method returns `Some(())` if the up focused widget is set, otherwise `None` for an
    /// error.
    fn up_focused_widget(&mut self) -> Option<()>
    {
        match self.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let widget = self.dyn_widget(rel_widget_path)?;
                if !widget.is_focusable() {
                    return None; 
                }
                let mut tmp_rel_widget_path = rel_widget_path.clone();
                let mut new_rel_widget_path = tmp_rel_widget_path.clone();
                loop {
                    if tmp_rel_widget_path.pop().is_some() {
                        if self.dyn_widget(&tmp_rel_widget_path)?.is_focusable() {
                            new_rel_widget_path = tmp_rel_widget_path.clone();
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if self.set_focused_rel_widget_path(Some(new_rel_widget_path)) {
                    Some(())
                } else {
                    None
                }
            },
            None => Some(()),
        }
    }

    /// Sets the down focused widget.
    ///
    /// This method returns `Some(())` if the down focused widget is set, otherwise `None` for an
    /// error.
    fn down_focused_widget(&mut self) -> Option<()>
    {
        match self.prev_or_next_focused_widget(Dir::Next, true)? {
            Some(rel_widget_path) => {
                if self.set_focused_rel_widget_path(Some(rel_widget_path)) {
                    Some(())
                } else {
                    None
                }
            },
            None => Some(()),
        }
    }
}

/// A trait of window iterator.
pub trait WindowIterator<'a>
{
    fn next(&mut self) -> Option<WindowIndex>;
}

/// An iterator that iterates over child window indices. 
pub struct ChildWindowIndices<'a>
{
    iter: Option<Box<dyn WindowIterator<'a> + 'a>>,
}

impl<'a> ChildWindowIndices<'a>
{
    fn new(iter: Option<Box<dyn WindowIterator<'a> + 'a>>) -> Self
    { ChildWindowIndices { iter, } }
}

impl<'a> Iterator for ChildWindowIndices<'a>
{
    type Item = WindowIndex;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.as_mut().map(|i| i.next()).flatten() }
}

/// Returns a reference to the window for the reference to the dynamic window or `None`.
pub fn dyn_window_as_window<T: Any>(window: &dyn Window) -> Option<&T>
{ window.as_any().downcast_ref::<T>() }

/// Returns a mutable reference to the window for the mutable reference to the dynamic window or
/// `None`.
pub fn dyn_window_mut_as_window_mut<T: Any>(window: &mut dyn Window) -> Option<&mut T>
{ window.as_any_mut().downcast_mut::<T>() }

/// Returns a reference to the focused widget of the window or `None`.
pub fn window_focused_widget<W: Window + ?Sized, T: Any>(window: &W) -> Option<&T>
{
    match window.focused_rel_widget_path().map(|rwp| rwp.clone()) {
        Some(rel_widget_path) => container_widget(window, &rel_widget_path),
        None => None,
    }
}

/// Returns a mutable reference to the focused widget of the window or `None`.
pub fn window_focused_widget_mut<W: Window + ?Sized, T: Any>(window: &mut W) -> Option<&mut T>
{
    match window.focused_rel_widget_path().map(|rwp| rwp.clone()) {
        Some(rel_widget_path) => container_widget_mut(window, &rel_widget_path),
        None => None,
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;
    
    #[test]
    fn test_window_sets_focused_relative_widget_path_for_one_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        assert_eq!(true, window.set_focused_rel_widget_path(Some(RelWidgetPath::new(WidgetIndexPair(0, 0)))));
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) =>{
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_focused_relative_widget_path_for_many_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) =>{
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_does_not_set_focused_relative_widget_path()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(3, 0));
        assert_eq!(false, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }    
    
    #[test]
    fn test_window_sets_focused_relative_widget_path_after_focoused_relative_widget_path_setting()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path1.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path1)));
        let mut rel_widget_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path2.push(WidgetIndexPair(2, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path2)));
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(2, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_does_not_set_focused_relative_widget_path_after_focoused_relative_widget_path_setting()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path1.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path1)));
        let mut rel_widget_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path2.push(WidgetIndexPair(3, 0));
        assert_eq!(false, window.set_focused_rel_widget_path(Some(rel_widget_path2)));
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_does_not_set_focused_relative_widget_path_for_unfocusable_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test5");
        widget3.set_focusable(false);
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path1.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path1)));
        let mut rel_widget_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path2.push(WidgetIndexPair(2, 0));
        assert_eq!(false, window.set_focused_rel_widget_path(Some(rel_widget_path2)));
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_next_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_focusable(false);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }        

    #[test]
    fn test_window_sets_next_focused_widget_for_nested_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget11 = MockWidget::new("test4");
        widget11.set_focusable(false);
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget12 = MockWidget::new("test5");
        widget12.set_focusable(false);
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test6");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test7");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test8");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget23 = MockWidget::new("test9");
        let path23 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget23)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(2, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path23) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_next_focused_widget_for_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_focusable(false);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(2, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_next_focused_widget_for_last_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_focusable(false);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(2, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_next_focused_widget_for_focusable_layout()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget11 = MockWidget::new("test4");
        widget11.set_focusable(false);
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget12 = MockWidget::new("test5");
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test6");
        layout2.set_focusable(true);
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test7");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget22 = MockWidget::new("test8");
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget23 = MockWidget::new("test9");
        let path23 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget23)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(1, 0));
        rel_widget_path.push(WidgetIndexPair(2, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path23) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_next_focused_widget_for_one_focusable_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }        

    #[test]
    fn test_window_sets_next_focused_widget_for_one_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_next_focused_widget_for_one_unfocusable_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let mut widget = MockWidget::new("test2");
        widget.set_focusable(false);
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_next_focused_widget_for_no_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        match window.next_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_window_sets_prevsious_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test5");
        widget3.set_focusable(false);
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_previous_focused_widget_for_nested_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget11 = MockWidget::new("test4");
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget12 = MockWidget::new("test5");
        widget12.set_focusable(false);
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget13 = MockWidget::new("test6");
        widget13.set_focusable(false);
        let path13 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget13)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test7");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test8");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test9");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path13) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_previous_focused_widget_for_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test5");
        widget3.set_focusable(false);
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }    

    #[test]
    fn test_window_sets_next_focused_widget_for_first_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test5");
        widget3.set_focusable(false);
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_previous_focused_widget_for_focusable_layout()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout1 = MockLayout::new("test3");
        layout1.set_focusable(true);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget11 = MockWidget::new("test4");
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget12 = MockWidget::new("test5");
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget13 = MockWidget::new("test6");
        widget13.set_focusable(false);
        let path13 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget13)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test7");
        layout2.set_focusable(true);
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget21 = MockWidget::new("test8");
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test9");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path13) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }    

    #[test]
    fn test_window_sets_previous_focused_widget_for_one_focusable_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }        

    #[test]
    fn test_window_sets_previous_focused_widget_for_one_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_previous_focused_widget_for_one_unfocusable_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let mut widget = MockWidget::new("test2");
        widget.set_focusable(false);
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_previous_focused_widget_for_no_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        match window.prev_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_window_sets_up_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let mut layout = MockLayout::new("test2");
        layout.set_focusable(true);
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.up_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(true),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_up_focused_widget_for_nested_focused_layout()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout1 = MockLayout::new("test3");
        layout1.set_focusable(true);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.up_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(true),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_up_focused_widget_for_upper_focused_layout()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout1 = MockLayout::new("test3");
        layout1.set_focusable(true);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.up_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(true),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_up_focused_widget_for_no_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout1 = MockLayout::new("test3");
        layout1.set_focusable(true);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.up_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_down_focused_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_focusable(false);
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget2 = MockWidget::new("test4");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget3 = MockWidget::new("test5");
        let path3 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path3) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_down_focused_widget_for_nested_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget11 = MockWidget::new("test4");
        widget11.set_focusable(false);
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget12 = MockWidget::new("test5");
        widget12.set_focusable(false);
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test6");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test7");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test8");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget23 = MockWidget::new("test9");
        let path23 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget23)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(2, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path23) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }    

    #[test]
    fn test_window_sets_down_focused_widget_for_focused_layout()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget11 = MockWidget::new("test4");
        widget11.set_focusable(false);
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget12 = MockWidget::new("test5");
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test6");
        layout2.set_focusable(true); 
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test7");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test8");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget23 = MockWidget::new("test9");
        let path23 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget23)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(2, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path23) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_down_focused_widget_for_root_focused_layout()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let mut layout = MockLayout::new("test2");
        layout.set_focusable(true);
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget11 = MockWidget::new("test4");
        widget11.set_focusable(false);
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget12 = MockWidget::new("test5");
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout2 = MockLayout::new("test6");
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test7");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test8");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget23 = MockWidget::new("test9");
        let path23 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget23)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path23) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_down_focused_widget_for_unfocusable_children()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let layout1 = MockLayout::new("test3");
        let path1 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout1)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget11 = MockWidget::new("test4");
        widget11.set_focusable(false);
        let path11 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget11)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget12 = MockWidget::new("test5");
        let path12 = match container_rel_widget_path(&mut window, &path1, |l: &mut MockLayout| l.add(widget12)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test6");
        layout2.set_focusable(true); 
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget21 = MockWidget::new("test7");
        widget21.set_focusable(false);
        let path21 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget21)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget22 = MockWidget::new("test8");
        widget22.set_focusable(false);
        let path22 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget22)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget23 = MockWidget::new("test9");
        widget23.set_focusable(false);
        let path23 = match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget23)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut rel_widget_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        rel_widget_path.push(WidgetIndexPair(1, 0));
        assert_eq!(true, window.set_focused_rel_widget_path(Some(rel_widget_path)));
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(rel_widget_path) => {
                let mut widget_index_pairs = rel_widget_path.widget_index_pairs();
                assert_eq!(Some(WidgetIndexPair(0, 0)), widget_index_pairs.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), widget_index_pairs.next());
                assert_eq!(None, widget_index_pairs.next());
            },
            None => assert!(false),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path1) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path11) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path12) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path2) {
            Some(widget) => assert_eq!(true, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path21) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path22) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
        match window.dyn_widget(&path23) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_window_sets_down_focused_widget_for_one_unfocusable_widget()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        let mut widget = MockWidget::new("test2");
        widget.set_focusable(false);
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindowWithFocusedWidget| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        match window.dyn_widget(&path) {
            Some(widget) => assert_eq!(false, widget.is_focused()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_window_sets_down_focused_widget_for_no_widgets()
    {
        let mut window = MockWindowWithFocusedWidget::new("test1");
        match window.down_focused_widget() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.focused_rel_widget_path() {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }    
}
