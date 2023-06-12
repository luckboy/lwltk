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

pub const DEFAULT_BUTTON_ICON_WIDTH: i32 = 32;
pub const DEFAULT_BUTTON_ICON_HEIGHT: i32 = 32;

pub fn draw_default_button_icon(cairo_context: &CairoContext, theme: &dyn Theme, pos: Pos<i32>, icon: ButtonIcon, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
{
    let x = pos.x as f64;
    let y = pos.y as f64;
    let width = DEFAULT_BUTTON_ICON_WIDTH as f64;
    let height = DEFAULT_BUTTON_ICON_HEIGHT as f64;
    cairo_context.save()?;
    cairo_context.rectangle(x, y, width, height);
    cairo_context.clip();
    match icon {
        ButtonIcon::Cancel => {
            cairo_context.set_line_width(4.0);
            theme.set_fg3(cairo_context, is_enabled, is_focused, is_focused_window)?;
            cairo_context.move_to(x + 2.0, y + 2.0);
            cairo_context.line_to(x + 30.0, y + 30.0);
            cairo_context.stroke()?;
            cairo_context.move_to(x + 30.0, y + 2.0);
            cairo_context.line_to(x + 2.0, y + 30.0);
            cairo_context.stroke()?;
        },
        ButtonIcon::Ok => {
            cairo_context.set_line_width(4.0);
            theme.set_fg4(cairo_context, is_enabled, is_focused, is_focused_window)?;
            cairo_context.move_to(x + 2.0, y + 16.0);
            cairo_context.line_to(x + 16.0, y + 30.0);
            cairo_context.line_to(x + 32.0, y + 2.0);
            cairo_context.stroke()?;
        },
    }
    cairo_context.restore()?;
    Ok(())
}
