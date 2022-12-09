//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use nix::fcntl::OFlag;
use nix::unistd::pipe2;
use nix::unistd::read;
use nix::unistd::write;
use crate::client_error::*;

#[derive(Copy, Clone, Debug)]
pub(crate) enum ThreadSignal
{
    Timer,
    Other,
}

#[derive(Copy, Clone, Debug)]
pub struct ThreadSignalSender(RawFd);

impl ThreadSignalSender
{
    pub(crate) fn commit_timer(&self) -> Result<(), ClientError>
    {
        let mut buf: [u8; 1] = [0];
        match write(self.0, &mut buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(ClientError::Nix(err)),
        }
    }

    pub fn commit(&self) -> Result<(), ClientError>
    {
        let mut buf: [u8; 1] = [1];
        match write(self.0, &mut buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(ClientError::Nix(err)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ThreadSignalReceiver(RawFd);

impl ThreadSignalReceiver
{
    pub(crate) fn recv(&self) -> Result<Option<ThreadSignal>, ClientError>
    {
        let mut buf: [u8; 1] = [0];
        match read(self.0, &mut buf) {
            Ok(0) => Ok(None),
            Ok(_) => {
                if buf[0] == 0 {
                    Ok(Some(ThreadSignal::Timer))
                } else {
                    Ok(Some(ThreadSignal::Other))
                }
            },
            Err(err) => Err(ClientError::Nix(err)),
        }
    }
}

impl AsRawFd for ThreadSignalReceiver
{
    fn as_raw_fd(&self) -> RawFd
    { self.0 }
}

pub(crate) fn thread_signal_channel() -> Result<(ThreadSignalSender, ThreadSignalReceiver), ClientError>
{
    match pipe2(OFlag::O_CLOEXEC) {
        Ok((reading_fd, writing_fd)) => Ok((ThreadSignalSender(writing_fd), ThreadSignalReceiver(reading_fd))),
        Err(err) => Err(ClientError::Nix(err)),
    }
}
