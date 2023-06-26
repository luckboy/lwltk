//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::image::*;
use crate::theme::*;
use crate::types::*;

pub const DEFAULT_BUTTON_ICON_SIZE: i32 = 24;

pub fn draw_default_button_icon(cairo_context: &CairoContext, theme: &dyn Theme, pos: Pos<i32>, icon: ButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
{
    let x = pos.x as f64;
    let y = pos.y as f64;
    let size = DEFAULT_BUTTON_ICON_SIZE as f64;
    cairo_context.save()?;
    cairo_context.rectangle(x, y, size, size);
    cairo_context.clip();
    match icon {
        ButtonIcon::Cancel => {
            cairo_context.set_line_width(4.0);
            theme.set_fg3(cairo_context, state, is_enabled, is_focused, is_focused_window)?;
            cairo_context.move_to(x + 2.0, y + 2.0);
            cairo_context.line_to(x + 22.0, y + 22.0);
            cairo_context.stroke()?;
            cairo_context.move_to(x + 22.0, y + 2.0);
            cairo_context.line_to(x + 2.0, y + 22.0);
            cairo_context.stroke()?;
        },
        ButtonIcon::Ok => {
            cairo_context.set_line_width(4.0);
            theme.set_fg4(cairo_context, state, is_enabled, is_focused, is_focused_window)?;
            cairo_context.move_to(x + 2.0, y + 12.0);
            cairo_context.line_to(x + 12.0, y + 22.0);
            cairo_context.line_to(x + 22.0, y + 2.0);
            cairo_context.stroke()?;
        },
    }
    cairo_context.restore()?;
    Ok(())
}
