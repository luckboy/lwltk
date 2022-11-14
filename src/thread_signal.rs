//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::os::unix::io::RawFd;
use crate::client_error::*;

#[derive(Copy, Clone, Debug)]
pub struct ThreadSignalSender(RawFd);

#[derive(Copy, Clone, Debug)]
pub(crate) struct ThreadSignalReceiver(RawFd);

pub(crate) fn thread_signal_channel() -> Result<(ThreadSignalSender, ThreadSignalReceiver), ClientError>
{ Ok((ThreadSignalSender(0), ThreadSignalReceiver(0))) }
