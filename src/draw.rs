//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::as_any::*;
use crate::theme::*;
use crate::types::*;

/// A drawing trait.
///
/// The drawing trait allows to update position, update size, draw. The drawable object is a window
/// or a widget.
pub trait Draw: AsAny + Send + Sync
{
    /// Updates the size of the drawable object.
    ///
    /// Also, this method can update the margin size and the sizes of the descendant widgets. 
    fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>;
    
    /// Updates the position of the drawable object.
    ///
    /// Also, this method can update the margin position and the positions of the descendant widgets.
    fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>;

    /// Draws the drawable object.
    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>;
}
