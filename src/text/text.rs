//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::iter::once;
use crate::types::*;
use crate::utils::*;

#[derive(Copy, Clone, Debug)]
pub struct TextLine
{
    pub start: usize,
    pub end: usize,
    pub width: i32,
}

impl TextLine
{
    pub fn new(start: usize, end: usize, width: i32) -> Self
    { TextLine { start, end, width, } }
}

#[derive(Clone)]
pub struct Text
{
    pub text: String,
    pub align: TextAlign,
    pub ellipsize_count: Option<usize>,
    pub is_trimmed: bool,
    pub lines: Vec<TextLine>,
    pub has_dot_dot_dot: bool,
}

impl Text
{
    pub fn new(text: &str, align: TextAlign) -> Self
    {
        Text {
            text: String::from(text),
            align,
            ellipsize_count: None,
            is_trimmed: true,
            lines: Vec::new(),
            has_dot_dot_dot: false,
        }
    }
    
    pub fn update_size<F>(&mut self, cairo_context: &CairoContext, area_size: Size<Option<i32>>, font_setting_f: F) -> Result<(), CairoError>
        where F: FnOnce(&CairoContext) -> Result<(), CairoError>
    {
        cairo_context.save()?;
        font_setting_f(cairo_context)?;
        let mut tmp_end_iter = self.text.char_indices().map(|p| p.0);
        tmp_end_iter.next();
        let end_iter = tmp_end_iter.chain(once(self.text.len()));
        let mut iter = self.text.char_indices().zip(end_iter);
        let mut is_first_word = true;
        let mut is_first_char = true;
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut tmp_width = 0.0;
        let mut width = 0.0;
        let mut last_word_iter = iter.clone();
        let mut last_word_start: usize = 0;
        let mut last_word_end: usize = 0;
        let mut last_word_width = 0.0;
        let mut is_prev_combining = false;
        let mut prev_c = ' ';
        let mut line_count = 0;
        let dot_dot_dot_text_extents = cairo_context.text_extents("\u{2026}")?;
        let mut dot_dot_dot_end = 0;
        let mut dot_dot_dot_width = 0.0;
        self.lines.clear();
        self.has_dot_dot_dot = false;
        loop {
            let tmp_iter = iter.clone();
            match iter.next() {
                Some(((i, tmp_c), tmp_j)) => {
                    let mut c = tmp_c;
                    let mut j = tmp_j;
                    let mut is_combining = false;
                    loop {
                        let mut is_double_combining = false;
                        loop {
                            let tmp_iter2 = iter.clone();
                            match iter.next() {
                                Some(((_, c2), j2)) => {
                                    if is_mark_char(c2) {
                                        is_combining = true;
                                        is_double_combining |= is_mark_char(c2);
                                        j = j2;
                                    } else {
                                        iter = tmp_iter2;
                                        break;
                                    }
                                },
                                None => break,
                            }
                        }
                        if is_double_combining {
                            match iter.next() {
                                Some(((_, _), j3)) => j = j3,
                                None => break,
                            }
                        } else {
                            break;
                        }
                    }
                    let text_extents = cairo_context.text_extents(&self.text[i..j])?;
                    if self.ellipsize_count.map(|n| line_count + 1 < n).unwrap_or(true) {
                        if is_combining || c != '\n' {
                            if !self.is_trimmed || !is_first_char || is_combining || !c.is_whitespace() {
                                if !is_combining && c.is_whitespace() {
                                    if is_prev_combining || !prev_c.is_whitespace() {
                                        is_first_word = false;
                                        last_word_iter = iter.clone();
                                        last_word_start = j;
                                        last_word_end = i;
                                        last_word_width = width;
                                    } else {
                                        if self.is_trimmed {
                                            last_word_start = j;
                                        }
                                    }
                                }
                                let new_width = tmp_width + text_extents.x_advance;
                                if is_first_char || area_size.width.map(|w| new_width <= w as f64).unwrap_or(true) {
                                    tmp_width = new_width;
                                    if !self.is_trimmed || is_combining || !c.is_whitespace() {
                                        end = j;
                                        width = tmp_width;
                                    }
                                    is_first_char = false;
                                } else {
                                    if is_first_word {
                                        self.lines.push(TextLine::new(start, end, width.ceil() as i32));
                                        iter = tmp_iter.clone();
                                        start = i;
                                        end = i;
                                        dot_dot_dot_end = i;
                                    } else {
                                        self.lines.push(TextLine::new(start, last_word_end, last_word_width.ceil() as i32));
                                        iter = last_word_iter.clone();
                                        start = last_word_start;
                                        end = last_word_start;
                                        dot_dot_dot_end = last_word_start;
                                    }
                                    tmp_width = 0.0;
                                    width = 0.0;
                                    is_first_word = true;
                                    is_first_char = true;
                                    line_count += 1;
                                    is_combining = false;
                                    c = ' ';
                                }
                            } else {
                                start = j;
                                end = j;
                                last_word_iter = iter.clone();
                                last_word_start = j;
                                last_word_end = j;
                                dot_dot_dot_end = j;
                            }
                        } else {
                            self.lines.push(TextLine::new(start, end, width.ceil() as i32));
                            start = j;
                            end = j;
                            dot_dot_dot_end = j;
                            tmp_width = 0.0;
                            width = 0.0;
                            is_first_word = true;
                            is_first_char = true;
                            line_count += 1;
                            is_combining = false;
                            c = ' ';
                        }
                    } else {
                        if is_combining || c != '\n' {
                            if !self.is_trimmed || !is_first_char || is_combining || !c.is_whitespace() {
                                if area_size.width.map(|w| width + dot_dot_dot_text_extents.x_advance <= w as f64).unwrap_or(true) {
                                    dot_dot_dot_end = i;
                                    dot_dot_dot_width = width;
                                }
                                let new_width = tmp_width + text_extents.x_advance;
                                if area_size.width.map(|w| new_width <= w as f64).unwrap_or(true) {
                                    tmp_width = new_width;
                                    if !self.is_trimmed || is_combining || !c.is_whitespace() {
                                        end = j;
                                        width = tmp_width;
                                    }
                                    is_first_char = false;
                                } else {
                                    end = dot_dot_dot_end;
                                    width = dot_dot_dot_width;
                                    self.has_dot_dot_dot = true;
                                    break;
                                }
                            } else {
                                start = j;
                                end = j;
                                dot_dot_dot_end = j;
                            }
                        } else {
                            end = dot_dot_dot_end;
                            width = dot_dot_dot_width;
                            self.has_dot_dot_dot = true;
                            break;
                        }
                    }
                    is_prev_combining = is_combining;
                    prev_c = c;
                },
                None => break,
            }
        }
        self.lines.push(TextLine::new(start, end, width.ceil() as i32));
        cairo_context.restore()?;
        Ok(())
    }

    pub fn draw<F, G>(&self, cairo_context: &CairoContext, area_bounds: Rect<i32>, font_setting_f: F, mut drawing_f: G) -> Result<(), CairoError>
        where F: FnOnce(&CairoContext) -> Result<(), CairoError>,
              G: FnMut(&CairoContext, Pos<i32>, &str) -> Result<(), CairoError>
    {
        cairo_context.save()?;
        font_setting_f(cairo_context)?;
        let font_extents = cairo_context.font_extents()?;
        let dot_dot_dot_text_extents = cairo_context.text_extents("\u{2026}")?;
        let font_height = font_extents.height.ceil() as i32;
        let mut y = area_bounds.y + (area_bounds.height - (font_height * self.lines.len() as i32)) / 2;
        for (i, line) in self.lines.iter().enumerate() {
            let mut width = line.width;
            if i + 1 >= self.lines.len() && self.has_dot_dot_dot {
                width += dot_dot_dot_text_extents.x_advance.ceil() as i32;
            }
            let x = match self.align {
                TextAlign::Left => area_bounds.x,
                TextAlign::Center => area_bounds.x + (area_bounds.width - width) / 2,
                TextAlign::Right => area_bounds.x + area_bounds.width - width,
            };
            drawing_f(cairo_context, Pos::new(x, y), &self.text[line.start..line.end])?;
            if i + 1 >= self.lines.len() && self.has_dot_dot_dot {
                drawing_f(cairo_context, Pos::new(x + line.width, y), "\u{2026}")?;
            }
            y += font_height;
        }
        cairo_context.restore()?;
        Ok(())
    }
}
