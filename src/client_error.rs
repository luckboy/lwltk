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

#[derive(Debug)]
pub enum ClientError
{
    Mutex,
    RwLock,
    Recv,
    Send,
    ThreadJoin,
    Io(Error),
    Cairo(CairoError),
    Connect(ConnectError),
    Global(GlobalError),
    Nix(Errno),
    Cursor,
    NoXdgRuntimeDir,
    InvalidThemeName,
    InvalidTheme,
    Data,
    Event(Event),
    Callback,
    NoSerial,
    WindowCycle,
    NoWindow,
    NoClientWindow,
    NoWidget,
    EventPreparation,
    DifferentWindows,
    InvalidButton,
    InvalidState,
    InvalidAxis,
    UnsupportedXkbKeymapFormat,
    NoXkbKeymap,
    NoXkbState,
    NoKeyboardWindowIndex,
    NoCurrentCallOnPath,
    NoWidgetIndexPair,
    NoCursor,
    NoPostButtonReleaseOnCallPath,
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
            ClientError::NoPostButtonReleaseOnCallPath => write!(f, "no post-button release on call path"),
        }
    }
}
