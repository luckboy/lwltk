//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_error::*;
use crate::themes::*;
use crate::types::*;

pub trait Theme: Send + Sync
{
    fn set_cairo_context(&self, cairo_context: &CairoContext, scale: i32) -> Result<(), CairoError>;
    
    fn draw_window_bg(&self, cairo_context: &CairoContext, bounds: Rect<i32>) -> Result<(), CairoError>;
}

pub fn theme_from_env() -> Result<Box<dyn Theme>, ClientError>
{ Ok(Box::new(DefaultTheme::new())) }
