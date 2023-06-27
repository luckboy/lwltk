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

pub const DEFAULT_TITLE_BUTTON_ICON_SIZE: i32 = 12;

pub fn draw_default_title_button_icon(cairo_context: &CairoContext, theme: &dyn Theme, pos: Pos<i32>, icon: TitleButtonIcon, state: WidgetState, is_enabled: bool, is_focused: bool, is_focused_window: bool) -> Result<(), CairoError>
{
    let x = pos.x as f64;
    let y = pos.y as f64;
    let size = DEFAULT_TITLE_BUTTON_ICON_SIZE as f64;
    cairo_context.save()?;
    cairo_context.rectangle(x, y, size, size);
    cairo_context.clip();
    match icon {
        TitleButtonIcon::Close => {
            theme.set_fg(cairo_context, state, is_enabled, is_focused, is_focused_window)?;
            cairo_context.move_to(x + 1.0, y + 1.0);
            cairo_context.line_to(x + 11.0, y + 11.0);
            cairo_context.stroke()?;
            cairo_context.move_to(x + 11.0, y + 1.0);
            cairo_context.line_to(x + 1.0, y + 11.0);
            cairo_context.stroke()?;
        },
        TitleButtonIcon::Maximize => {
            theme.set_fg(cairo_context, state, is_enabled, is_focused, is_focused_window)?;
            cairo_context.rectangle(x + 1.0, y + 1.0, 10.0, 10.0);
            cairo_context.stroke()?;
            cairo_context.move_to(x + 1.0, y + 3.0);
            cairo_context.line_to(x + 11.0, y + 3.0);
            cairo_context.stroke()?;
        },
        TitleButtonIcon::Menu => {
            theme.set_fg(cairo_context, state, is_enabled, is_focused, is_focused_window)?;
            cairo_context.move_to(x, y + 2.0);
            cairo_context.line_to(x + 12.0, y + 2.0);
            cairo_context.stroke()?;
            cairo_context.move_to(x, y + 6.0);
            cairo_context.line_to(x + 12.0, y + 6.0);
            cairo_context.stroke()?;
            cairo_context.move_to(x, y + 10.0);
            cairo_context.line_to(x + 12.0, y + 10.0);
            cairo_context.stroke()?;
        },
    }
    cairo_context.restore()?;
    Ok(())
}
