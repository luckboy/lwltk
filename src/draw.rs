//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::as_any::*;
use crate::types::*;

pub trait Draw: AsAny + Send + Sync
{
    fn update_size(&mut self, cairo_context: &CairoContext, is_focused_window: bool);
    
    fn update_pos(&mut self, cairo_context: &CairoContext, is_focused_window: bool);

    fn draw(&self, cairo_context: &CairoContext, is_focused_window: bool);
}
