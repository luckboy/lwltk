//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::theme::*;
use crate::types::*;

pub struct Image
{
    pub size_fun: Box<dyn Fn(&dyn Theme) -> Size<i32> + Send + Sync + 'static>,
    pub drawing_fun: Box<dyn Fn(&CairoContext, &dyn Theme, Pos<i32>) -> Result<(), CairoError> + Send + Sync + 'static>,
}

impl Image
{
    pub fn new<F, G>(size_f: F, drawing_f: G) -> Self
        where F: Fn(&dyn Theme) -> Size<i32> + Send + Sync + 'static,
              G: Fn(&CairoContext, &dyn Theme, Pos<i32>) -> Result<(), CairoError> + Send + Sync + 'static
    { Image { size_fun: Box::new(size_f), drawing_fun: Box::new(drawing_f), } }

    pub fn new_dyn<F, G>(size_f: Box<dyn Fn(&dyn Theme) -> Size<i32> + Send + Sync + 'static>, drawing_f: Box<dyn Fn(&CairoContext, &dyn Theme, Pos<i32>) -> Result<(), CairoError> + Send + Sync + 'static>) -> Self
    { Image { size_fun: size_f, drawing_fun: drawing_f, } }
    
    pub fn draw(&self, cairo_context: &CairoContext, theme: &dyn Theme, pos: Pos<i32>) -> Result<(), CairoError>
    {
        let size = (self.size_fun)(theme);
        cairo_context.save()?;
        cairo_context.rectangle(pos.x as f64, pos.y as f64, size.width as f64, size.height as f64);
        cairo_context.clip();
        (self.drawing_fun)(cairo_context, theme, pos)?;
        cairo_context.restore()?;
        Ok(())
    }
}
