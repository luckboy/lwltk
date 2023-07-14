//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use std::io::Error;
use nix::errno::Errno;
use wayland_client::ConnectError;
use wayland_client::GlobalError;
use crate::events::*;
use crate::types::*;

/// An enumaration of client error.
#[derive(Debug)]
pub enum ClientError
{
    /// A mutex error.
    Mutex,
    /// An error of reader-writer lock.
    RwLock,
    /// An error of channel receiving.
    Recv,
    /// An error of channel sending.
    Send,
    /// An error of thread joining.
    ThreadJoin,
    /// An input/ouput error.
    Io(Error),
    /// A Cairo error.
    Cairo(CairoError),
    /// A Wayland connection error.
    Connect(ConnectError),
    /// A Wayland global error.
    Global(GlobalError),
    /// A Nix error.
    Nix(Errno),
    /// A cursor error.
    Cursor,
    /// An error of no XDG_RUNTIME_DIR variable.
    NoXdgRuntimeDir,
    /// An error of invalid theme name.
    InvalidThemeName,
    /// An error of invalid theme.
    InvalidTheme,
    /// A data error.
    Data,
    /// An event error.
    Event(Event),
    /// A callback error.
    Callback,
    /// An error of no Wayland serial
    NoSerial,
    /// An error of window cycle.
    WindowCycle,
    /// An error of no window.
    NoWindow,
    /// An error of no client window.
    NoClientWindow,
    /// An error of no widget.
    NoWidget,
    /// An error of event preparation.
    EventPreparation,
    /// An error of different windows.
    DifferentWindows,
    /// An error of invalid Wayland button.
    InvalidButton,
    /// An error of invalid Wayland state.
    InvalidState,
    /// An error of invalid Wayland axis.
    InvalidAxis,
    /// An error of unsupported XKB keymap format.
    UnsupportedXkbKeymapFormat,
    /// An error of no XKB keymap.
    NoXkbKeymap,
    /// An error of no XKB state.
    NoXkbState,
    /// An error of no keyboard window index.
    NoKeyboardWindowIndex,
    /// An error of no current call-on path.
    NoCurrentCallOnPath,
    /// An error of no pair of widget indices.
    NoWidgetIndexPair,
    /// An error of no cursor.
    NoCursor,
    /// An error of no call-on path of post button release.
    NoPostButtonReleaseCallOnPath,
}

impl error::Error for ClientError
{}

impl fmt::Display for ClientError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            ClientError::Mutex => write!(f, "mutex error"),
            ClientError::RwLock => write!(f, "rwlock error"),
            ClientError::Recv => write!(f, "recv error"),
            ClientError::Send => write!(f, "send error"),
            ClientError::ThreadJoin => write!(f, "thread join error"),
            ClientError::Io(err) => write!(f, "io: {}", err),
            ClientError::Cairo(err) => write!(f, "cairo: {}", err),
            ClientError::Connect(err) => write!(f, "connect: {}", err),
            ClientError::Global(err) => write!(f, "global: {}", err),
            ClientError::Nix(err) => write!(f, "nix: {}", err),
            ClientError::Cursor => write!(f, "cursor loading error"),
            ClientError::NoXdgRuntimeDir => write!(f, "no XDG_RUNTIME_DIR variable"),
            ClientError::InvalidThemeName => write!(f, "invalid theme name"),
            ClientError::InvalidTheme => write!(f, "invalid theme"),
            ClientError::Data => write!(f, "data error"),
            ClientError::Event(event) => write!(f, "event error for {:?}", event),
            ClientError::Callback => write!(f, "callback error"),
            ClientError::NoSerial => write!(f, "no serial"),
            ClientError::WindowCycle => write!(f, "cycle of windows"),
            ClientError::NoWindow => write!(f, "no window"),
            ClientError::NoClientWindow => write!(f, "no client window"),
            ClientError::NoWidget => write!(f, "no widget"),
            ClientError::EventPreparation => write!(f, "event preparation error"),
            ClientError::DifferentWindows => write!(f, "different windows"),
            ClientError::InvalidButton => write!(f, "invalid button"),
            ClientError::InvalidState => write!(f, "invalid state"),
            ClientError::InvalidAxis => write!(f, "invalid axis"),
            ClientError::UnsupportedXkbKeymapFormat => write!(f, "unsupported xkb keynmap format"),
            ClientError::NoXkbKeymap => write!(f, "no xkb keymap"),
            ClientError::NoXkbState => write!(f, "no xkb state"),
            ClientError::NoKeyboardWindowIndex => write!(f, "no keyboard window index"),
            ClientError::NoCurrentCallOnPath => write!(f, "no current call on path"),
            ClientError::NoWidgetIndexPair => write!(f, "no widget index pair"),
            ClientError::NoCursor => write!(f, "no cursor"),
            ClientError::NoPostButtonReleaseCallOnPath => write!(f, "no post-button release call on path"),
        }
    }
}
