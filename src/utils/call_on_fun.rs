//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_context::*;
use crate::events::*;
use crate::queue_context::*;

pub struct CallOnFun
{
    pub fun: Box<dyn FnMut(&mut ClientContext, &mut QueueContext, &Event) -> Option<EventOption> + Send + Sync + 'static>,
}

impl CallOnFun
{
    pub fn new() -> Self
    { CallOnFun { fun: Box::new(|_, _, _| Some(EventOption::Default)), } }
    
    pub fn call_on(&mut self, client_context: &mut ClientContext, queue_context: &mut QueueContext, event: &Event, default_event: Option<Event>) -> Option<Option<Event>>
    {
        match (self.fun)(client_context, queue_context, event) {
            Some(EventOption::Some(ret_event)) => Some(Some(ret_event)),
            Some(EventOption::Default) => Some(default_event),
            Some(EventOption::None) => Some(None),
            None => None,
        }
    }
}
