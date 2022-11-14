//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Error;
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
    Connect(ConnectError),
    Global(GlobalError),
    Nix(Errno),
}
