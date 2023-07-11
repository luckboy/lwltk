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
    /// This method returns an event that will be called for the parent widget, `Some(None)` for no
    /// event propagation, or `None` for an error.
    fn call_on(&mut self, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event) -> Option<Option<Event>>;
}
