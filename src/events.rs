//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::keys::*;
use crate::types::*;

#[derive(Clone, Debug)]
pub enum Event
{
    Click,
    DoubleClick,
    LongClick,
    PopupClick,
    Key(VKey, KeyModifiers),
    Char(char),
    CheckChange(bool),
    RadioSelection(usize),
    ComboSelection(usize),
    TextSelection(usize, usize),
    ListItemSelection(usize),
    ListItemDeselection(usize),
    TableRowSelection(usize),
    TableRowDeselection(usize),
    TableCellSelection(usize, usize),
    TableCellDeselection(usize, usize),
    TreeNodeSelection(Vec<usize>),
    TreeNodeDeselection(Vec<usize>),
    Menu,
    Close,
    Maximize,
    Client(ClientEvent),
}

#[derive(Clone, Debug)]
pub enum ClientEvent
{
    ShellSurfaceConfigure(ClientResize, Size<i32>),
    ShellSurfacePopupDone,
    PointerEnter(Pos<f64>),
    PointerLeave,
    PointerMotion(u32, Pos<f64>),
    PointerButton(u32, ClientButton, ClientState),
    PointerAxis(u32, ClientAxis, f64),
    KeyboardEnter,
    KeyboardLeave,
    KeyboardKey(u32, Vec<VKey>, String, ClientState),
    KeyboardModifiers(KeyModifiers),
    TouchDown(u32, i32, Pos<f64>),
    TouchUp(u32, i32),
    TouchMotion(u32, i32, Pos<i64>),
    RepeatedKey(Vec<VKey>, String),
}

#[derive(Clone)]
pub enum EventOption
{
    None,
    Default,
    Some(Event),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientResize
{
    None,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientButton
{
    Left,
    Right,
    Middle,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientState
{
    Released,
    Pressed,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientAxis
{
    VScroll,
    HScroll,
}
