//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of events.
//!
//! The module of events contains enumerations of events and related enumerations to events. This
//! module has two event enumerations. The first event enumeration has events which are pushed to
//! an event queue expect one variant. The second event enumeration has events which directly called
//! in a Wayland event or a system event. The first event enumeration is an [`Event`] enumeration
//! and has a variant that has a client event. The second event enumeration is a [`ClientEvent`]
//! enumeration.
use crate::keys::*;
use crate::types::*;

/// An event enumeation.
///
/// The events are for example clicks and key presses. The event can be pushed to an event queue.
/// The events in the event queue are popped and called for a widget or a window.
#[derive(Clone, Debug)]
pub enum Event
{
    /// A click event.
    ///
    /// The click event is called when an user clicks a widget or a window.
    Click,
    /// An event of double click.
    ///
    /// The event of double click is called when an user doubly clicks a widget or a window.
    DoubleClick,
    /// An event of long click.
    ///
    /// The event of long click is called when an user longly clicks a widget or a window.
    LongClick,
    /// An event of popup click.
    ///
    /// The event of popup click is called when an user right clicks a widget or a window.
    PopupClick,
    /// An event of key press.
    ///
    /// The event of key press is called when an user press or repeats key. The following fields are:
    /// - a key
    /// - key modifiers
    Key(VKey, KeyModifiers),
    /// An event of character of key press.
    ///
    /// The event of key press is called when an user press or repeats key. The field is a character.
    Char(char),
    /// An event of check change.
    ///
    /// The event of check change is called when a check is changed by an user. The field is a check
    /// state.
    CheckChange(bool),
    /// An event of radio selection.
    ///
    /// An event of radio selection is called when a ratio is selected by an user. The field is a
    /// selected number.
    RadioSelection(usize),
    /// An event of combo selection.
    ///
    /// The event of combo selection is called when a combo option is selected by an user. The field
    /// is a selected number.
    ComboSelection(usize),
    /// An event of text change.
    ///
    /// The event of text change is called when an user changed the text.
    TextChange,
    /// An event of text selection
    ///
    /// The event text selection is called when an user selects the text. The following fields are:
    /// - a text start
    /// - a text end
    TextSelection(usize, usize),
    /// An event of selection of list item.
    ///
    /// The event of selection of list item is called when an user selects the list item. The field
    /// is an item index.
    ListItemSelection(usize),
    /// An event of deselection of list item.
    ///
    /// The event of deselection of list item is called when an user deselects the list item. The
    /// field is an item index.
    ListItemDeselection(usize),
    /// An event of selection of table row.
    ///
    /// The event of selection of table row is called when an user selects the table row. The field
    /// is row index.
    TableRowSelection(usize),
    /// An event of deselection of table row.
    ///
    /// The event of deselection of table row is called when an user deselects the table row. The
    /// field is a row index.
    TableRowDeselection(usize),
    /// An event of selection of table cell.
    ///
    /// The event of selection of table cell is called when an user selects the table cell. The
    /// following fields are:
    /// - a row index
    /// - a column index
    TableCellSelection(usize, usize),
    /// An event of deselection of table cell.
    ///
    /// The event of deselection of table cell is called when an user deselects the table cell. The
    /// following fields are:
    /// - a row index
    /// - a column index
    TableCellDeselection(usize, usize),
    /// An event of selection of tree node.
    ///
    /// The event of selection of tree node is called when an user selects the tree node. The field
    /// is indices of node path.
    TreeNodeSelection(Vec<usize>),
    /// An event of deselection of tree node.
    ///
    /// The event of deselection of tree node is called when an user deselects the tree node. The
    /// field is indices of node path.
    TreeNodeDeselection(Vec<usize>),
    /// An event of horizontal scroll.
    ///
    /// The event of horizontal scroll is sent by a horizontal scroll bar to a scroll when the
    /// horizontal scroll bar is changed.
    HScroll,
    /// An event of vertical scroll.
    ///
    /// The event of vertcial scroll is sent by a vertical scroll bar to a scroll when the vertical
    /// scroll bar is changed.
    VScroll,
    /// A menu event.
    ///
    /// Th menu event is called when an user selects a window menu.
    Menu,
    /// A close event.
    ///
    /// Th close event is called when an user selects a window close.
    Close,
    /// A maximization event.
    ///
    /// Th maximization event is called when an user selects a window maximization.
    Maximize,
    /// An event of client event.
    Client(ClientEvent),
}

/// An enumeration of client event.
///
/// The client events are directly prepared from Wayland events and system events which are called.
/// The client event is called when the Wayland event or the system event is called. Other events
/// are pushed to an event queue in a default event handler or a callback.
#[derive(Clone, Debug)]
pub enum ClientEvent
{
    /// An event of shell surface configure.
    ///
    /// The following fields are:
    /// - a client resize
    /// - a size
    ShellSurfaceConfigure(ClientResize, Size<i32>),
    /// An event of shell surface popup done.
    ShellSurfacePopupDone,
    /// An event of pointer enter.
    ///
    /// The field is a position.
    PointerEnter(Pos<f64>),
    /// An event of pointer leave.
    PointerLeave,
    /// An event of pointer motion.
    ///
    /// The following fields are:
    /// - a time
    /// - a position
    PointerMotion(u32, Pos<f64>),
    /// An event of pointer button.
    ///
    /// The following fields are:
    /// - a time
    /// - a client button
    /// - a client state
    PointerButton(u32, ClientButton, ClientState),
    /// An event of pointer axis.
    ///
    /// The following fields are:
    /// - a time
    /// - a client axis
    /// - a value
    PointerAxis(u32, ClientAxis, f64),
    /// An event of keyboard enter.
    KeyboardEnter,
    /// An event of keyboard leave.
    KeyboardLeave,
    /// An event of keyboard key.
    ///
    /// The following fields are:
    /// - a time
    /// - keys
    /// - characters
    /// - a client state 
    KeyboardKey(u32, Vec<VKey>, String, ClientState),
    /// An event of keyboard key.
    ///
    /// The following fields are:
    /// - a time
    /// - key modifiers
    /// - a client state 
    KeyboardModifiers(KeyModifiers),
    /// An event of touch down.
    ///
    /// The following fields are:
    /// - a time
    /// - a touch identifier
    /// - a position
    TouchDown(u32, i32, Pos<f64>),
    /// An event of touch up.
    ///
    /// The following fields are:
    /// - a time
    /// - a touch identifier
    TouchUp(u32, i32),
    /// An event of touch motion.
    ///
    /// The following fields are:
    /// - a time
    /// - a touch identifier
    /// - a position
    TouchMotion(u32, i32, Pos<f64>),
    /// An event of repeated button.
    RepeatedButton,
    /// An event of repeated key.
    ///
    /// The following fields are:
    /// - keys
    /// - characters
    RepeatedKey(Vec<VKey>, String),
    /// An event of repeated touch.
    ///
    /// The field is a touch identifier.
    RepeatedTouch(i32),
    /// An event of post button release.
    PostButtonRelease,
}

/// An enumaration of event option.
///
/// The event option can be none, a default event, or an event. The default event is an event that
/// is returned by a default event handler. The event option is used as the returned value of the
/// event handler.
#[derive(Clone)]
pub enum EventOption
{
    /// A none.
    None,
    /// A default event.
    Default,
    /// An event.
    Some(Event),
}

impl EventOption
{
    /// Returns `true` if the event option hasn't the event, otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::events::Event;
    /// use lwltk::events::EventOption;
    ///
    /// let event1 = EventOption::None;
    /// let event2 = EventOption::Default;
    /// let event3 = EventOption::Some(Event::Click);
    /// assert_eq!(true, event1.is_none());
    /// assert_eq!(false, event2.is_none());
    /// assert_eq!(false, event3.is_none());
    /// ```
    pub fn is_none(&self) -> bool
    {
        match self {
            EventOption::None => true,
            _ => false,
        }
    }

    /// Returns `true` if the event option has the default event, otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::events::Event;
    /// use lwltk::events::EventOption;
    ///
    /// let event1 = EventOption::None;
    /// let event2 = EventOption::Default;
    /// let event3 = EventOption::Some(Event::Click);
    /// assert_eq!(false, event1.is_default());
    /// assert_eq!(true, event2.is_default());
    /// assert_eq!(false, event3.is_default());
    /// ```
    pub fn is_default(&self) -> bool
    {
        match self {
            EventOption::Default => true,
            _ => false,
        }
    }

    /// Returns `true` if the event option has the event, otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use lwltk::events::Event;
    /// use lwltk::events::EventOption;
    ///
    /// let event1 = EventOption::None;
    /// let event2 = EventOption::Default;
    /// let event3 = EventOption::Some(Event::Click);
    /// assert_eq!(false, event1.is_some());
    /// assert_eq!(false, event2.is_some());
    /// assert_eq!(true, event3.is_some());
    /// ```
    pub fn is_some(&self) -> bool
    {
        match self {
            EventOption::Some(_) => true,
            _ => false,
        }
    }
}

/// An enumeration of client resize.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientResize
{
    /// A none.
    None,
    /// A top edge.
    Top,
    /// A bottom edge.
    Bottom,
    /// A left edge.
    Left,
    /// A right edge.
    Right,
    /// A top edge and a left edge.
    TopLeft,
    /// A top edge and a right edge.
    TopRight,
    /// A bottom edge and a left edge.
    BottomLeft,
    /// A bottom edge and a right edge.
    BottomRight,
}

/// An enumeration of client button.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientButton
{
    /// A left button.
    Left,
    /// A right button.
    Right,
    /// A middle button.
    Middle,
}

/// An enumeration of client state.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientState
{
    /// A button or a key is released.
    Released,
    /// A button or a key is pressed.
    Pressed,
}

/// An enumeration of client axis.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientAxis
{
    /// A vertical axis.
    VScroll,
    /// A horizontal axis.
    HScroll,
}
