//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::os::unix::io::RawFd;
use nix::fcntl::OFlag;
use nix::unistd::close;
use nix::unistd::pipe2;
use nix::unistd::read;
use nix::unistd::write;
use crate::client_error::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ThreadTimer
{
    Cursor,
    Button,
    Key,
    Touch,
    TextCursor,
    PostButtonRelease,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ThreadSignal
{
    Timer(ThreadTimer),
    Other,
}

/// A structure of sender of thread signal.
///
/// The sender of thread signal allows to send thread signals to a graphic thread. A main loop
/// receives this signals and reacts on these signals. The thread signals for other threads are
/// communication method with the graphic thread. These signals are implemented by a pipe.
#[derive(Copy, Clone, Debug)]
pub struct ThreadSignalSender(RawFd);

impl ThreadSignalSender
{
    pub(crate) fn commit_timer(&self, timer: ThreadTimer) -> Result<(), ClientError>
    {
        let mut buf: [u8; 1] = [0];
        match timer {
            ThreadTimer::Cursor => buf[0] = 0,
            ThreadTimer::Button => buf[0] = 1,
            ThreadTimer::Key => buf[0] = 2,
            ThreadTimer::Touch => buf[0] = 3,
            ThreadTimer::TextCursor => buf[0] = 4,
            ThreadTimer::PostButtonRelease => buf[0] = 5,
        }
        match write(self.0, &buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(ClientError::Nix(err)),
        }
    }

    /// Sends the committing thread signal to the graphic thread.
    ///
    /// This thread signal confims committed change of windows, pushed events, and pushed callbacks
    /// for the graphic thread.
    pub fn commit(&self) -> Result<(), ClientError>
    {
        let mut buf: [u8; 1] = [4];
        match write(self.0, &mut buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(ClientError::Nix(err)),
        }
    }
    
    pub(crate) fn close(&self) -> Result<(), ClientError>
    {
        match close(self.0) {
            Ok(()) => Ok(()),
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
                    Ok(Some(ThreadSignal::Timer(ThreadTimer::Cursor)))
                } else if buf[0] == 1 {
                    Ok(Some(ThreadSignal::Timer(ThreadTimer::Button)))
                } else if buf[0] == 2 {
                    Ok(Some(ThreadSignal::Timer(ThreadTimer::Key)))
                } else if buf[0] == 3 {
                    Ok(Some(ThreadSignal::Timer(ThreadTimer::Touch)))
                } else if buf[0] == 4 {
                    Ok(Some(ThreadSignal::Timer(ThreadTimer::TextCursor)))
                } else if buf[0] == 5 {
                    Ok(Some(ThreadSignal::Timer(ThreadTimer::PostButtonRelease)))
                } else {
                    Ok(Some(ThreadSignal::Other))
                }
            },
            Err(err) => Err(ClientError::Nix(err)),
        }
    }

    pub(crate) fn fd(&self) -> RawFd
    { self.0 }

    pub(crate) fn close(&self) -> Result<(), ClientError>
    {
        match close(self.0) {
            Ok(()) => Ok(()),
            Err(err) => Err(ClientError::Nix(err)),
        }
    }
}

pub(crate) fn thread_signal_channel() -> Result<(ThreadSignalSender, ThreadSignalReceiver), ClientError>
{
    match pipe2(OFlag::O_CLOEXEC) {
        Ok((reading_fd, writing_fd)) => Ok((ThreadSignalSender(writing_fd), ThreadSignalReceiver(reading_fd))),
        Err(err) => Err(ClientError::Nix(err)),
    }
}
