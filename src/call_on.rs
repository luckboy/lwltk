//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_context::*;
use crate::events::*;
use crate::queue_context::*;

/// A call-on trait.
///
/// The call-on trait allows to call the handler for an event. The call-on object is a window or a
/// widget.
pub trait CallOn: Send + Sync
{
    /// Calls the handler for an event.
    ///
    /// This method returns an event for an event propagration, `Some(None)` for no the event
    /// propagation, or `None` for an error. If the event is returned and the parent of the call-on
    /// object exists, the event propagation occurs. In other words, the returned event is called for
    /// the parent of the call-on object.
    fn call_on(&mut self, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Event>>;
}
