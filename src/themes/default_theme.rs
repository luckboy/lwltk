//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::theme::*;
use crate::types::*;

pub struct DefaultTheme
{}

impl DefaultTheme
{
    pub fn new() -> Self
    { DefaultTheme {} }
}

impl Theme for DefaultTheme
{
    fn set_cairo_context(&self, cairo_context: &CairoContext, scale: i32) -> Result<(), CairoError>
    {
        cairo_context.scale(scale as f64, scale as f64);
        Ok(())
    }
    
    fn draw_window_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>) -> Result<(), CairoError>
    {
        cairo_context.set_source_rgba(0.0, 0.0, 1.0, 0.5);
        cairo_context.rectangle(bounds.x as f64, bounds.y as f64, bounds.width as f64, bounds.height as f64);
        cairo_context.fill()?;
        Ok(())
    }

    fn window_edges(&self) -> Edges<i32>
    { Edges::new(0, 0, 0, 0) }

    fn window_corners(&self) -> Corners<i32>
    { Corners::new(0, 0, 0, 0) }
}
