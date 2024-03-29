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
/// The drawing trait allows to update position, update size, and draw. The drawable object is a
/// window or a widget. The following methods are called in order:
/// - [`update_size`](Self::update_size)
/// - [`update_pos`](Self::update_pos)
/// - [`draw`](Self::draw)
pub trait Draw: AsAny + Send + Sync
{
    /// Updates the size of the drawable object.
    ///
    /// Also, this method can update the margin size and the sizes of the descendant widgets. The
    /// size and the margin size can't exceed the area size for the widget.
    fn update_size(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_size: Size<Option<i32>>) -> Result<(), CairoError>;
    
    /// Updates the position of the drawable object.
    ///
    /// Also, this method can update the margin position and the positions of the descendant widgets.
    /// The position and the margin position depend from the area bounds.
    fn update_pos(&mut self, cairo_context: &CairoContext, theme: &dyn Theme, area_bounds: Rect<i32>) -> Result<(), CairoError>;

    /// Draws the drawable object.
    ///
    /// Also, this method can draw the descendant widgets.
    fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, is_focused_window: bool) -> Result<(), CairoError>;
}
