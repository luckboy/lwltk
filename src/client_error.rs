//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use std::io::Error;
use cairo;
use nix::errno::Errno;
use wayland_client::ConnectError;
use wayland_client::GlobalError;

#[derive(Debug)]
pub enum ClientError
{
    Mutex,
    RwLock,
    Recv,
    Send,
    Io(Error),
    Cairo(cairo::Error),
    Connect(ConnectError),
    Global(GlobalError),
    Nix(Errno),
    NoXdgRuntimeDir,
    InvalidThemeName,
    InvalidTheme,
    Data,
    Event,
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
            ClientError::Io(err) => write!(f, "io: {}", err),
            ClientError::Cairo(err) => write!(f, "cairo: {}", err),
            ClientError::Connect(err) => write!(f, "connect: {}", err),
            ClientError::Global(err) => write!(f, "global: {}", err),
            ClientError::Nix(err) => write!(f, "nix: {}", err),
            ClientError::NoXdgRuntimeDir => write!(f, "no XDG_RUNTIME_DIR variable"),
            ClientError::InvalidThemeName => write!(f, "invalid theme name"),
            ClientError::InvalidTheme => write!(f, "invalid theme"),
            ClientError::Data => write!(f, "data error"),
            ClientError::Event => write!(f, "event error"),
        }
    }
}
