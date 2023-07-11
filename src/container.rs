//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::iter::FusedIterator;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use crate::call_on::*;
use crate::draw::*;
use crate::types::*;
use crate::widget::*;

/// A direction enumeration.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Dir
{
    /// A previous.
    Prev,
    /// A next.
    Next,
}

/// A container trait.
///
/// The container contains widgets. This trait allows to hava access to widgets of container. The
/// widget or the window is the container.
pub trait Container: Draw + CallOn
{
    /// Returns a pair of the indices of the previous widget for the pair of widget indices or
    /// `None`.
    ///
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { None }

    /// Returns a pair of the indices of the next widget for the pair of widget indices or
    /// `None`.
    ///
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { None }
    
    /// Returns a reference to the dynamic widget for the pair of widget indices or `None`.
    ///
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    { None }

    /// Returns a mutable reference to the dynamic widget for the pair of widget indices or
    /// `None`.
    ///
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    { None }
    
    /// Returns a pair of the widget indices refers to a widget that is pointed the position or
    /// `None`.
    ///
    /// This method defaultly returns `None`.
    #[allow(unused_variables)]
    fn point_for_index_pair(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    { None }

    /// Returns a reference to the dynamic widget for the relative widget path or `None`
    ///
    /// # Examples
    /// ```
    /// use lwltk::widgets::Button;
    /// use lwltk::widgets::LinearLayout;
    /// use lwltk::Container;
    /// use lwltk::container_rel_widget_path1;
    /// use lwltk::container_rel_widget_path;
    /// use lwltk::dyn_widget_as_widget;
    ///
    /// let mut layout1 = LinearLayout::new();
    /// let layout2 = LinearLayout::new();
    /// let layout2_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(layout2)).unwrap();
    /// let button1 = Button::new("test1");
    /// let button1_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(button1)).unwrap();
    /// let button2 = Button::new("test2");
    /// let button2_path = container_rel_widget_path(&mut layout1, &layout2_path, |layout: &mut LinearLayout| layout.add(button2)).unwrap();
    /// let widget1 = layout1.dyn_widget(&button1_path);
    /// assert_eq!(Some(Some("test1")), widget1.map(|w| dyn_widget_as_widget::<Button>(w).map(|b| b.text())));
    /// let widget2 = layout1.dyn_widget(&button2_path);
    /// assert_eq!(Some(Some("test2")), widget2.map(|w| dyn_widget_as_widget::<Button>(w).map(|b| b.text())));
    /// ```
    fn dyn_widget<'a>(&'a self, path: &RelWidgetPath) -> Option<&'a dyn Widget>
    {
        let mut idx_pair_iter = path.widget_index_pairs();
        match idx_pair_iter.next() {
            Some(idx_pair) => {
                let mut widget: Option<&'a dyn Widget> = self.dyn_widget_for_index_pair(idx_pair);
                for idx_pair in idx_pair_iter {
                    match widget {
                        Some(tmp_widget) => widget = tmp_widget.dyn_widget_for_index_pair(idx_pair),
                        None => break,
                    }
                }
                widget
            },
            None => None,
        }
    }

    /// Returns a mutable reference to the dynamic widget for the relative widget path or `None`
    ///
    /// # Examples
    /// ```
    /// use lwltk::widgets::Button;
    /// use lwltk::widgets::LinearLayout;
    /// use lwltk::Container;
    /// use lwltk::container_rel_widget_path1;
    /// use lwltk::container_rel_widget_path;
    /// use lwltk::dyn_widget_mut_as_widget_mut;
    ///
    /// let mut layout1 = LinearLayout::new();
    /// let layout2 = LinearLayout::new();
    /// let layout2_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(layout2)).unwrap();
    /// let button1 = Button::new("test1");
    /// let button1_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(button1)).unwrap();
    /// let button2 = Button::new("test2");
    /// let button2_path = container_rel_widget_path(&mut layout1, &layout2_path, |layout: &mut LinearLayout| layout.add(button2)).unwrap();
    /// let widget1 = layout1.dyn_widget_mut(&button1_path);
    /// assert_eq!(Some(Some("test1")), widget1.map(|w| dyn_widget_mut_as_widget_mut::<Button>(w).map(|b| b.text())));
    /// let widget2 = layout1.dyn_widget_mut(&button2_path);
    /// assert_eq!(Some(Some("test2")), widget2.map(|w| dyn_widget_mut_as_widget_mut::<Button>(w).map(|b| b.text())));
    /// ```
    fn dyn_widget_mut<'a>(&'a mut self, path: &RelWidgetPath) -> Option<&'a mut dyn Widget>
    {
        let mut idx_pair_iter = path.widget_index_pairs();
        match idx_pair_iter.next() {
            Some(idx_pair) => {
                let mut widget: Option<&'a mut dyn Widget> = self.dyn_widget_mut_for_index_pair(idx_pair);
                for idx_pair in idx_pair_iter {
                    match widget {
                        Some(tmp_widget) => widget = tmp_widget.dyn_widget_mut_for_index_pair(idx_pair),
                        None => break,
                    }
                }
                widget
            },
            None => None,
        }
    }
    
    /// Returns a relative widget path refers to a widget that is pointed the position or `None`.
    fn point(&self, pos: Pos<f64>) -> Option<RelWidgetPath>
    {
        match self.point_for_index_pair(pos) {
            Some(idx_pair) => {
                let mut widget_path = RelWidgetPath::new(idx_pair);
                let mut widget: Option<&'_ dyn Widget> = self.dyn_widget_for_index_pair(idx_pair);
                loop {
                    let idx_pair = match widget {
                        Some(tmp_widget) => {
                            match tmp_widget.point_for_index_pair(pos) {
                                Some(tmp_idx_pair) => tmp_idx_pair,
                                None => break,
                            }
                        },
                        None => break,
                    };
                    widget_path.push(idx_pair);
                    widget = match widget {
                        Some(tmp_widget) => tmp_widget.dyn_widget_for_index_pair(idx_pair),
                        None => None,
                    }
                }
                Some(widget_path)
            },
            None => None,
        }
    }

    /// Returns a relative widget path refers to a focusable widget that is pointed the position
    /// or `None`.
    fn point_focusable(&self, pos: Pos<f64>) -> Option<RelWidgetPath>
    {
        match self.point_for_index_pair(pos) {
            Some(idx_pair) => {
                let mut widget_path = RelWidgetPath::new(idx_pair);
                let mut focusable_widget_path: Option<RelWidgetPath> = None;
                let mut widget: Option<&'_ dyn Widget> = self.dyn_widget_for_index_pair(idx_pair);
                match widget {
                    Some(tmp_widget) if tmp_widget.is_focusable() => focusable_widget_path = Some(widget_path.clone()),
                    _ => (),
                }
                loop {
                    let idx_pair = match widget {
                        Some(tmp_widget) => {
                            match tmp_widget.point_for_index_pair(pos) {
                                Some(tmp_idx_pair) => tmp_idx_pair,
                                None => break,
                            }
                        },
                        None => break,
                    };
                    widget_path.push(idx_pair);
                    widget = match widget {
                        Some(tmp_widget) => tmp_widget.dyn_widget_for_index_pair(idx_pair),
                        None => None,
                    };
                    match widget {
                        Some(tmp_widget) if tmp_widget.is_focusable() => {
                            focusable_widget_path = Some(widget_path.clone())
                        },
                        _ => (),
                    }
                }
                focusable_widget_path
            },
            None => None,
        }
    }
    
    /// Returns a pair of the indices of the previous or next widget for the pair of the widget
    /// indices and the direction or `None`.
    fn prev_or_next(&self, idx_pair: Option<WidgetIndexPair>, dir: Dir) -> Option<WidgetIndexPair>
    {
        match dir {
            Dir::Prev => self.prev(idx_pair),
            Dir::Next => self.next(idx_pair),
        }
    }

    /// Resets the states for the descendant widgets.
    fn reset_descendant_states(&mut self)
    {
        let mut prev_idx_pair = None;
        loop {
            match self.next(prev_idx_pair) {
                Some(idx_pair) => {
                    match self.dyn_widget_mut_for_index_pair(idx_pair) {
                        Some(widget) => {
                            widget.set_state(WidgetState::None);
                            widget.reset_descendant_states();
                        },
                        None => (),
                    }
                    prev_idx_pair = Some(idx_pair);
                },
                None => break,
            }
        }
    }

    /// Sets the reference-counting pointers to the change flag for the descendant widgets.
    fn set_descendant_change_flag_arcs(&mut self, flag_arc: Arc<AtomicBool>)
    {
        let mut prev_idx_pair = None;
        loop {
            match self.next(prev_idx_pair) {
                Some(idx_pair) => {
                    match self.dyn_widget_mut_for_index_pair(idx_pair) {
                        Some(widget) => {
                            widget.set_only_change_flag_arc(flag_arc.clone());
                            widget.set_descendant_change_flag_arcs(flag_arc.clone());
                        },
                        None => (),
                    }
                    prev_idx_pair = Some(idx_pair);
                },
                None => break,
            }
        }
    }
}

/// A reversed iterator that iterates over pairs of widget indices.
#[derive(Clone)]
pub struct RevWidgetIndexPairs<'a>
{
    container: &'a dyn Container,
    widget_index_pair: Option<Option<WidgetIndexPair>>,
}

impl<'a> RevWidgetIndexPairs<'a>
{
    /// Creates a reversed iterator that iterates over pairs of widget indices.
    pub fn new(container: &'a dyn Container) -> Self
    { RevWidgetIndexPairs { container, widget_index_pair: Some(None), } }
}

impl<'a> FusedIterator for RevWidgetIndexPairs<'a>
{}

impl<'a> Iterator for RevWidgetIndexPairs<'a>
{
    type Item = WidgetIndexPair;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.widget_index_pair {
            Some(idx_pair) => {
                let next_idx_pair = self.container.prev(idx_pair);
                self.widget_index_pair = match next_idx_pair {
                    Some(_) => Some(next_idx_pair),
                    None => None,
                };
                next_idx_pair
            },
            None => None,
        }
        
    }
}

/// An iterator that iterates over pairs of widget indices.
#[derive(Clone)]
pub struct WidgetIndexPairs<'a>
{
    container: &'a dyn Container,
    widget_index_pair: Option<Option<WidgetIndexPair>>,
}

impl<'a> WidgetIndexPairs<'a>
{
    /// Creates an iterator that iterates over pairs of widget indices.
    pub fn new(container: &'a dyn Container) -> Self
    { WidgetIndexPairs { container, widget_index_pair: Some(None), } }
}

impl<'a> FusedIterator for WidgetIndexPairs<'a>
{}

impl<'a> Iterator for WidgetIndexPairs<'a>
{
    type Item = WidgetIndexPair;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.widget_index_pair {
            Some(idx_pair) => {
                let next_idx_pair = self.container.next(idx_pair);
                self.widget_index_pair = match next_idx_pair {
                    Some(_) => Some(next_idx_pair),
                    None => None,
                };
                next_idx_pair
            },
            None => None,
        }
        
    }
}

/// A reversed iterator that iterates over widgets.
#[derive(Clone)]
pub struct RevWidgets<'a>
{
    iter: RevWidgetIndexPairs<'a>,
}

impl<'a> RevWidgets<'a>
{
    /// Creates a reversed iterator that iterates over widgets.
    pub fn new(container: &'a dyn Container) -> Self
    { RevWidgets { iter: RevWidgetIndexPairs::new(container), } }
}

impl<'a> FusedIterator for RevWidgets<'a>
{}

impl<'a> Iterator for RevWidgets<'a>
{
    type Item = &'a dyn Widget;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(idx_pair) => self.iter.container.dyn_widget_for_index_pair(idx_pair),
            None => None,
        }
    }
}

/// An iterator that iterates over widgets.
#[derive(Clone)]
pub struct Widgets<'a>
{
    iter: WidgetIndexPairs<'a>,
}

impl<'a> Widgets<'a>
{
    /// Creates an iterator that iterates over widgets.
    pub fn new(container: &'a dyn Container) -> Self
    { Widgets { iter: WidgetIndexPairs::new(container), } }
}

impl<'a> FusedIterator for Widgets<'a>
{}

impl<'a> Iterator for Widgets<'a>
{
    type Item = &'a dyn Widget;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(idx_pair) => self.iter.container.dyn_widget_for_index_pair(idx_pair),
            None => None,
        }
    }
}

/// Returns a reference to the widget for the container and the relative widget path or `None`.
///
/// # Examples
/// ```
/// use lwltk::widgets::Button;
/// use lwltk::widgets::LinearLayout;
/// use lwltk::Container;
/// use lwltk::container_rel_widget_path1;
/// use lwltk::container_rel_widget_path;
/// use lwltk::container_widget;
///
/// let mut layout1 = LinearLayout::new();
/// let layout2 = LinearLayout::new();
/// let layout2_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(layout2)).unwrap();
/// let button1 = Button::new("test1");
/// let button1_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(button1)).unwrap();
/// let button2 = Button::new("test2");
/// let button2_path = container_rel_widget_path(&mut layout1, &layout2_path, |layout: &mut LinearLayout| layout.add(button2)).unwrap();
/// let widget1: Option<&Button> = container_widget(&layout1, &button1_path);
/// assert_eq!(Some("test1"), widget1.map(|w| w.text()));
/// let widget2: Option<&Button> = container_widget(&layout1, &button2_path);
/// assert_eq!(Some("test2"), widget2.map(|w| w.text()));
/// ```
pub fn container_widget<'a, C: Container + ?Sized, T: Any>(container: &'a C, path: &RelWidgetPath) -> Option<&'a T>
{ container.dyn_widget(path).map(|wg| wg.as_any().downcast_ref::<T>()).flatten() }

/// Returns a mutable reference to the widget for the container and the relative widget path or
/// `None`.
///
/// # Examples
/// ```
/// use lwltk::widgets::Button;
/// use lwltk::widgets::LinearLayout;
/// use lwltk::Container;
/// use lwltk::container_rel_widget_path1;
/// use lwltk::container_rel_widget_path;
/// use lwltk::container_widget_mut;
///
/// let mut layout1 = LinearLayout::new();
/// let layout2 = LinearLayout::new();
/// let layout2_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(layout2)).unwrap();
/// let button1 = Button::new("test1");
/// let button1_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(button1)).unwrap();
/// let button2 = Button::new("test2");
/// let button2_path = container_rel_widget_path(&mut layout1, &layout2_path, |layout: &mut LinearLayout| layout.add(button2)).unwrap();
/// let widget1: Option<&mut Button> = container_widget_mut(&mut layout1, &button1_path);
/// assert_eq!(Some("test1"), widget1.map(|w| w.text()));
/// let widget2: Option<&mut Button> = container_widget_mut(&mut layout1, &button2_path);
/// assert_eq!(Some("test2"), widget2.map(|w| w.text()));
/// ```
pub fn container_widget_mut<'a, C: Container + ?Sized, T: Any>(container: &'a mut C, path: &RelWidgetPath) -> Option<&'a mut T>
{ container.dyn_widget_mut(path).map(|wg| wg.as_any_mut().downcast_mut::<T>()).flatten() }

/// Returns a relative widget path that contains a pair of widget indices from the closure for the
/// container or `None`.
///
/// The closure can be used to add the widget to the container. The returned relative widget path
/// refers to an added widget.
///
/// # Examples
/// ```
/// use lwltk::widgets::Button;
/// use lwltk::widgets::LinearLayout;
/// use lwltk::Container;
/// use lwltk::RelWidgetPath;
/// use lwltk::WidgetIndexPair;
/// use lwltk::container_rel_widget_path1;
///
/// let mut layout = LinearLayout::new();
/// let button = Button::new("test");
/// let button_path = container_rel_widget_path1(&mut layout, |layout: &mut LinearLayout| layout.add(button)).unwrap();
/// let expected_button_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
/// assert_eq!(expected_button_path, button_path);
/// ```
pub fn container_rel_widget_path1<'a, C: Container, F>(container: &'a mut C, f: F) -> Option<RelWidgetPath>
    where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
{
    match f(container) {
        Some(idx_pair) => Some(RelWidgetPath::new(idx_pair)),
        None => None,
    }
}

/// Returns a relative widget path that is joint the specified relative widget path with a pair
/// of widget indices from the closure for the container or `None`.
///
/// The closure can be used to add the widget to the descendant container. The returned relative
/// widget path refers to an added widget.
////
/// # Examples
/// ```
/// use lwltk::widgets::Button;
/// use lwltk::widgets::LinearLayout;
/// use lwltk::Container;
/// use lwltk::RelWidgetPath;
/// use lwltk::WidgetIndexPair;
/// use lwltk::container_rel_widget_path1;
/// use lwltk::container_rel_widget_path;
///
/// let mut layout1 = LinearLayout::new();
/// let layout2 = LinearLayout::new();
/// let layout2_path = container_rel_widget_path1(&mut layout1, |layout: &mut LinearLayout| layout.add(layout2)).unwrap();
/// let button = Button::new("test");
/// let button_path = container_rel_widget_path(&mut layout1, &layout2_path, |layout: &mut LinearLayout| layout.add(button)).unwrap();
/// let mut expected_button_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
/// expected_button_path.push(WidgetIndexPair(0, 0));
/// assert_eq!(expected_button_path, button_path);
/// ```
pub fn container_rel_widget_path<'a, C: Container + ?Sized, T: Any, F>(container: &'a mut C, path: &RelWidgetPath, f: F) -> Option<RelWidgetPath>
    where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
{
    match container_widget_mut(container, path) {
        Some(widget) => {
            match f(widget) {
                Some(idx_pair) => {
                    let mut new_path = path.clone();
                    new_path.push(idx_pair);
                    Some(new_path)
                },
                None => None,
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::mocks::*;
    
    #[test]
    fn test_container_sets_one_widget()
    {
        let mut window = MockWindow::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let expected_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let widget: Option<&MockWidget> = container_widget(&window, &path);
        match widget {
            Some(widget) => assert_eq!("test2", widget.text()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_adds_widgets()
    {
        let mut window = MockWindow::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
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
        let expected_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let mut expected_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        expected_path1.push(WidgetIndexPair(0, 0));
        assert_eq!(expected_path1, path1);
        let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        expected_path2.push(WidgetIndexPair(1, 0));
        assert_eq!(expected_path2, path2);
        let mut expected_path3 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        expected_path3.push(WidgetIndexPair(2, 0));
        assert_eq!(expected_path3, path3);
        let layout: Option<&MockLayout> = container_widget(&window, &path);
        match layout {
            Some(layout) => assert_eq!("test2", layout.text()),
            None => assert!(false),
        }
        let widget1: Option<&MockWidget> = container_widget(&window, &path1);
        match widget1 {
            Some(widget) => assert_eq!("test3", widget.text()),
            None => assert!(false),
        }
        let widget2: Option<&MockWidget> = container_widget(&window, &path2);
        match widget2 {
            Some(widget) => assert_eq!("test4", widget.text()),
            None => assert!(false),
        }
        let widget3: Option<&MockWidget> = container_widget(&window, &path3);
        match widget3 {
            Some(widget) => assert_eq!("test5", widget.text()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_sets_one_widget_for_mutable()
    {
        let mut window = MockWindow::new("test1");
        let widget = MockWidget::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(widget)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let expected_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let widget: Option<&mut MockWidget> = container_widget_mut(&mut window, &path);
        match widget {
            Some(widget) => assert_eq!("test2", widget.text()),
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_adds_widgets_for_mutable()
    {
        let mut window = MockWindow::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
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
        let expected_path = RelWidgetPath::new(WidgetIndexPair(0, 0));
        assert_eq!(expected_path, path);
        let mut expected_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        expected_path1.push(WidgetIndexPair(0, 0));
        assert_eq!(expected_path1, path1);
        let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        expected_path2.push(WidgetIndexPair(1, 0));
        assert_eq!(expected_path2, path2);
        let mut expected_path3 = RelWidgetPath::new(WidgetIndexPair(0, 0));
        expected_path3.push(WidgetIndexPair(2, 0));
        assert_eq!(expected_path3, path3);
        let layout: Option<&mut MockLayout> = container_widget_mut(&mut window, &path);
        match layout {
            Some(layout) => assert_eq!("test2", layout.text()),
            None => assert!(false),
        }
        let widget1: Option<&mut MockWidget> = container_widget_mut(&mut window, &path1);
        match widget1 {
            Some(widget) => assert_eq!("test3", widget.text()),
            None => assert!(false),
        }
        let widget2: Option<&mut MockWidget> = container_widget_mut(&mut window, &path2);
        match widget2 {
            Some(widget) => assert_eq!("test4", widget.text()),
            None => assert!(false),
        }
        let widget3: Option<&mut MockWidget> = container_widget_mut(&mut window, &path3);
        match widget3 {
            Some(widget) => assert_eq!("test5", widget.text()),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_container_points_widgets()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(200, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 190, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_margin_bounds(Rect::new(10, 10, 80, 90));
        widget1.set_bounds(Rect::new(15, 15, 70, 80));
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let mut widget2 = MockWidget::new("test4");
        widget2.set_margin_bounds(Rect::new(110, 10, 80, 90));
        widget2.set_bounds(Rect::new(115, 15, 70, 80));
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point(Pos::new(20.0, 20.0)) {
            Some(path1) => {
                let mut expected_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path1.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path1, path1);
            },
            None => assert!(false),
        }
        match window.point(Pos::new(125.0, 25.0)) {
            Some(path2) => {
                let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path2.push(WidgetIndexPair(1, 0));
                assert_eq!(expected_path2, path2);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_points_widget_in_nested_layout()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(100, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 90, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test3");
        layout2.set_bounds(Rect::new(10, 10, 80, 90));
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test3");
        widget3.set_margin_bounds(Rect::new(15, 15, 70, 80));
        widget3.set_bounds(Rect::new(20, 20, 60, 70));
        match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point(Pos::new(20.0, 20.0)) {
            Some(path3) => {
                let mut expected_path3 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path3.push(WidgetIndexPair(0, 0));
                expected_path3.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path3, path3);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_points_nested_layout()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(100, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 90, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test3");
        layout2.set_bounds(Rect::new(10, 10, 80, 90));
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test3");
        widget3.set_margin_bounds(Rect::new(15, 15, 70, 80));
        widget3.set_bounds(Rect::new(20, 20, 60, 70));
        match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point(Pos::new(15.0, 15.0)) {
            Some(path2) => {
                let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path2.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path2, path2);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_does_not_point_widgets()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(200, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 190, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_margin_bounds(Rect::new(10, 10, 80, 90));
        widget1.set_bounds(Rect::new(15, 15, 70, 80));
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let mut widget2 = MockWidget::new("test4");
        widget2.set_margin_bounds(Rect::new(110, 10, 80, 90));
        widget2.set_bounds(Rect::new(115, 15, 70, 80));
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point(Pos::new(0.0, 0.0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }    
    
    #[test]
    fn test_container_points_focusable_widgets()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(200, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 190, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget1 = MockWidget::new("test3");
        widget1.set_margin_bounds(Rect::new(10, 10, 80, 90));
        widget1.set_bounds(Rect::new(15, 15, 70, 80));
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let mut widget2 = MockWidget::new("test4");
        widget2.set_margin_bounds(Rect::new(110, 10, 80, 90));
        widget2.set_bounds(Rect::new(115, 15, 70, 80));
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point_focusable(Pos::new(20.0, 20.0)) {
            Some(path1) => {
                let mut expected_path1 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path1.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path1, path1);
            },
            None => assert!(false),
        }
        match window.point_focusable(Pos::new(125.0, 25.0)) {
            Some(path2) => {
                let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path2.push(WidgetIndexPair(1, 0));
                assert_eq!(expected_path2, path2);
            },
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_container_points_focusable_widget_in_nested_layout()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(100, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 90, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test3");
        layout2.set_bounds(Rect::new(10, 10, 80, 90));
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test3");
        widget3.set_margin_bounds(Rect::new(15, 15, 70, 80));
        widget3.set_bounds(Rect::new(20, 20, 60, 70));
        match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point_focusable(Pos::new(20.0, 20.0)) {
            Some(path3) => {
                let mut expected_path3 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path3.push(WidgetIndexPair(0, 0));
                expected_path3.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path3, path3);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_points_focusable_nested_layout()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(100, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 90, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test3");
        layout2.set_focusable(true);
        layout2.set_bounds(Rect::new(10, 10, 80, 90));
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test3");
        widget3.set_margin_bounds(Rect::new(15, 15, 70, 80));
        widget3.set_bounds(Rect::new(20, 20, 60, 70));
        match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point_focusable(Pos::new(15.0, 15.0)) {
            Some(path2) => {
                let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path2.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path2, path2);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_points_focusable_nested_layout_with_unfocusable_widget()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(100, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 90, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test3");
        layout2.set_focusable(true);
        layout2.set_bounds(Rect::new(10, 10, 80, 90));
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test3");
        widget3.set_focusable(false);
        widget3.set_margin_bounds(Rect::new(15, 15, 70, 80));
        widget3.set_bounds(Rect::new(20, 20, 60, 70));
        match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point_focusable(Pos::new(20.0, 20.0)) {
            Some(path2) => {
                let mut expected_path2 = RelWidgetPath::new(WidgetIndexPair(0, 0));
                expected_path2.push(WidgetIndexPair(0, 0));
                assert_eq!(expected_path2, path2);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_does_not_point_unfocusable_nested_layout_and_unfocusable_widget()
    {
        let mut window = MockWindow::new("test1");
        window.set_size(Size::new(100, 110));
        let mut layout = MockLayout::new("test2");
        layout.set_bounds(Rect::new(5, 5, 90, 100));
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut layout2 = MockLayout::new("test3");
        layout2.set_bounds(Rect::new(10, 10, 80, 90));
        let path2 = match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(layout2)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let mut widget3 = MockWidget::new("test3");
        widget3.set_focusable(false);
        widget3.set_margin_bounds(Rect::new(15, 15, 70, 80));
        widget3.set_bounds(Rect::new(20, 20, 60, 70));
        match container_rel_widget_path(&mut window, &path2, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        match window.point_focusable(Pos::new(20.0, 20.0)) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
    
    #[test]
    fn test_container_gives_reversed_widget_index_pair_iterator()
    {
        let mut window = MockWindow::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget2 = MockWidget::new("test4");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget3 = MockWidget::new("test5");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let layout: Option<&MockLayout> = container_widget(&window, &path);
        match layout {
            Some(layout) => {
                let mut iter = RevWidgetIndexPairs::new(layout);
                assert_eq!(Some(WidgetIndexPair(2, 0)), iter.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), iter.next());
                assert_eq!(Some(WidgetIndexPair(0, 0)), iter.next());
                assert_eq!(None, iter.next());
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_container_gives_widget_index_pair_iterator()
    {
        let mut window = MockWindow::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget2 = MockWidget::new("test4");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget3 = MockWidget::new("test5");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let layout: Option<&MockLayout> = container_widget(&window, &path);
        match layout {
            Some(layout) => {
                let mut iter = WidgetIndexPairs::new(layout);
                assert_eq!(Some(WidgetIndexPair(0, 0)), iter.next());
                assert_eq!(Some(WidgetIndexPair(1, 0)), iter.next());
                assert_eq!(Some(WidgetIndexPair(2, 0)), iter.next());
                assert_eq!(None, iter.next());
            },
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_container_gives_reversed_dynamic_widget_iterator()
    {
        let mut window = MockWindow::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget2 = MockWidget::new("test4");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget3 = MockWidget::new("test5");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let layout: Option<&MockLayout> = container_widget(&window, &path);
        match layout {
            Some(layout) => {
                let mut iter = RevWidgets::new(layout);
                match iter.next() {
                    Some(widget) => {
                        let widget: Option<&MockWidget> = dyn_widget_as_widget(widget);
                        match widget {
                            Some(widget) => assert_eq!("test5", widget.text()),
                            None => assert!(false),
                        }
                    },
                    None => assert!(false),
                }
                match iter.next() {
                    Some(widget) => {
                        let widget: Option<&MockWidget> = dyn_widget_as_widget(widget);
                        match widget {
                            Some(widget) => assert_eq!("test4", widget.text()),
                            None => assert!(false),
                        }
                    },
                    None => assert!(false),
                }
                match iter.next() {
                    Some(widget) => {
                        let widget: Option<&MockWidget> = dyn_widget_as_widget(widget);
                        match widget {
                            Some(widget) => assert_eq!("test3", widget.text()),
                            None => assert!(false),
                        }
                    },
                    None => assert!(false),
                }
                match iter.next() {
                    Some(_) => assert!(false),
                    None => assert!(true),
                }
            },
            None => assert!(false),
        }
    }    

    #[test]
    fn test_container_gives_dynamic_widget_iterator()
    {
        let mut window = MockWindow::new("test1");
        let layout = MockLayout::new("test2");
        let path = match container_rel_widget_path1(&mut window, |w: &mut MockWindow| w.set(layout)) {
            Some(tmp_path) => tmp_path,
            None => {
                assert!(false);
                unreachable!()
            },
        };
        let widget1 = MockWidget::new("test3");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget1)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget2 = MockWidget::new("test4");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget2)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let widget3 = MockWidget::new("test5");
        match container_rel_widget_path(&mut window, &path, |l: &mut MockLayout| l.add(widget3)) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
        let layout: Option<&MockLayout> = container_widget(&window, &path);
        match layout {
            Some(layout) => {
                let mut iter = Widgets::new(layout);
                match iter.next() {
                    Some(widget) => {
                        let widget: Option<&MockWidget> = dyn_widget_as_widget(widget);
                        match widget {
                            Some(widget) => assert_eq!("test3", widget.text()),
                            None => assert!(false),
                        }
                    },
                    None => assert!(false),
                }
                match iter.next() {
                    Some(widget) => {
                        let widget: Option<&MockWidget> = dyn_widget_as_widget(widget);
                        match widget {
                            Some(widget) => assert_eq!("test4", widget.text()),
                            None => assert!(false),
                        }
                    },
                    None => assert!(false),
                }
                match iter.next() {
                    Some(widget) => {
                        let widget: Option<&MockWidget> = dyn_widget_as_widget(widget);
                        match widget {
                            Some(widget) => assert_eq!("test5", widget.text()),
                            None => assert!(false),
                        }
                    },
                    None => assert!(false),
                }
                match iter.next() {
                    Some(_) => assert!(false),
                    None => assert!(true),
                }
            },
            None => assert!(false),
        }
    }
}
