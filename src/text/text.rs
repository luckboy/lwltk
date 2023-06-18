//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::max;
use std::iter::once;
use crate::types::*;
use crate::utils::*;

const DOT_DOT_DOT: &'static str = "\u{2026}";

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
    pub line_height: i32,
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
            line_height: 0,
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
        let mut is_prev_combination = false;
        let mut prev_c = ' ';
        let mut line_count = 0;
        let dot_dot_dot_text_extents = cairo_context.text_extents(DOT_DOT_DOT)?;
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
                    let mut is_combination = false;
                    loop {
                        let mut is_double_combination = false;
                        loop {
                            let tmp_iter2 = iter.clone();
                            match iter.next() {
                                Some(((_, c2), j2)) => {
                                    if is_mark_char(c2) {
                                        is_combination = true;
                                        is_double_combination |= is_mark_char(c2);
                                        j = j2;
                                    } else {
                                        iter = tmp_iter2;
                                        break;
                                    }
                                },
                                None => break,
                            }
                        }
                        if is_double_combination {
                            let tmp_iter3 = iter.clone();
                            match iter.next() {
                                Some(((_, '\n'), _)) => {
                                    iter = tmp_iter3;
                                    break;
                                },
                                Some(((_, _), j3)) => j = j3,
                                None => break,
                            }
                        } else {
                            break;
                        }
                    }
                    let text_extents = cairo_context.text_extents(&self.text[i..j])?;
                    if self.ellipsize_count.map(|n| line_count + 1 < n).unwrap_or(true) {
                        if is_combination || c != '\n' {
                            if !self.is_trimmed || !is_first_char || is_combination || !c.is_whitespace() {
                                if !is_combination && c.is_whitespace() {
                                    if is_prev_combination || !prev_c.is_whitespace() {
                                        is_first_word = false;
                                        last_word_iter = iter.clone();
                                        last_word_start = j;
                                        last_word_end = i;
                                        last_word_width = width;
                                    } else {
                                        if self.is_trimmed {
                                            last_word_iter = iter.clone();
                                            last_word_start = j;
                                        }
                                    }
                                }
                                let new_width = tmp_width + text_extents.x_advance;
                                if is_first_char || area_size.width.map(|w| new_width <= w as f64).unwrap_or(true) {
                                    tmp_width = new_width;
                                    if !self.is_trimmed || is_combination || !c.is_whitespace() {
                                        end = j;
                                        width = tmp_width;
                                    }
                                    is_first_char = false;
                                } else {
                                    if is_first_word {
                                        self.lines.push(TextLine::new(start, end, width.ceil() as i32));
                                        iter = tmp_iter;
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
                                    is_combination = false;
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
                            is_combination = false;
                            c = ' ';
                        }
                    } else {
                        if is_combination || c != '\n' {
                            if !self.is_trimmed || !is_first_char || is_combination || !c.is_whitespace() {
                                if area_size.width.map(|w| width + dot_dot_dot_text_extents.x_advance <= w as f64).unwrap_or(true) {
                                    dot_dot_dot_end = i;
                                    dot_dot_dot_width = width;
                                }
                                let new_width = tmp_width + text_extents.x_advance;
                                if area_size.width.map(|w| new_width <= w as f64).unwrap_or(true) {
                                    tmp_width = new_width;
                                    if !self.is_trimmed || is_combination || !c.is_whitespace() {
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
                    is_prev_combination = is_combination;
                    prev_c = c;
                },
                None => break,
            }
        }
        self.lines.push(TextLine::new(start, end, width.ceil() as i32));
        let font_extents = cairo_context.font_extents()?;
        self.line_height = font_extents.height.ceil() as i32;
        cairo_context.restore()?;
        Ok(())
    }

    pub fn draw<F, G>(&self, cairo_context: &CairoContext, area_bounds: Rect<i32>, font_setting_f: F, mut drawing_f: G) -> Result<(), CairoError>
        where F: FnOnce(&CairoContext) -> Result<(), CairoError>,
              G: FnMut(&CairoContext, Pos<i32>, &str) -> Result<(), CairoError>
    {
        cairo_context.save()?;
        font_setting_f(cairo_context)?;
        cairo_context.rectangle(area_bounds.x as f64, area_bounds.y as f64, area_bounds.width as f64, area_bounds.height as f64);
        cairo_context.clip();
        let dot_dot_dot_text_extents = cairo_context.text_extents(DOT_DOT_DOT)?;
        let mut y = area_bounds.y + (area_bounds.height - (self.line_height * self.lines.len() as i32)) / 2;
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
                drawing_f(cairo_context, Pos::new(x + line.width, y), DOT_DOT_DOT)?;
            }
            y += self.line_height;
        }
        cairo_context.restore()?;
        Ok(())
    }
    
    pub fn max_line_width(&self) -> i32
    { self.lines.iter().fold(0, |w, l| max(w, l.width)) }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use cairo::FontSlant;
    use cairo::FontWeight;
    
    #[test]
    fn test_text_updates_size()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc", TextAlign::Left);
        match text.update_size(&cairo_context, Size::new(None, None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_font_setting()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(32.0);
        let mut text = Text::new("abc", TextAlign::Left);
        match text.update_size(&cairo_context, Size::new(None, None), |cairo_context| {
                cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
                cairo_context.set_font_size(16.0);
                Ok(())
        }) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_empty_text()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("", TextAlign::Left);
        match text.update_size(&cairo_context, Size::new(None, None), |_| Ok(())) {
            Ok(()) => {
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(0, text.lines[0].end);
                assert_eq!(0, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_trimmed_text()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk abc", TextAlign::Left);
        match text.update_size(&cairo_context, Size::new(None, None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_ijk_abc_width = (a + b + c + space + i + j + k + space + a + b + c).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(11, text.lines[0].end);
                assert_eq!(expected_abc_ijk_abc_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_trimmed_text_with_spaces()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("   abc  ijk abc  ", TextAlign::Left);
        match text.update_size(&cairo_context, Size::new(None, None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_ijk_abc_width = (a + b + c + space * 2.0 + i + j + k + space + a + b + c).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(3, text.lines[0].start);
                assert_eq!(15, text.lines[0].end);
                assert_eq!(expected_abc_ijk_abc_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }    
    
    #[test]
    fn test_text_updates_size_for_trimmed_text_and_line_break()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk", TextAlign::Left);
        let width = (a + b + c + space + i).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(7, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_trimmed_text_and_line_break_by_space()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk", TextAlign::Left);
        let width = (a + b + c + space).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(7, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_trimmed_text_and_line_break_by_many_spaces()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc   ijk", TextAlign::Left);
        let width = (a + b + c + space + space).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(6, text.lines[1].start);
                assert_eq!(9, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_trimmed_text_and_word_break()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("aaabbbccc", TextAlign::Left);
        let width = (a * 3.0 + b * 3.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_aaabbb_width = (a * 3.0 + b * 3.0).ceil() as i32;
                let expected_ccc_width = (c * 3.0).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(6, text.lines[0].end);
                assert_eq!(expected_aaabbb_width, text.lines[0].width);
                assert_eq!(6, text.lines[1].start);
                assert_eq!(9, text.lines[1].end);
                assert_eq!(expected_ccc_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_trimmed_text_and_line_break_and_many_lines()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk abc ijk abc abc jik", TextAlign::Left);
        let width = (a + b + c + space + i + j + k + space + a + b + c).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_ijk_abc_width = (a + b + c + space + i + j + k + space + a + b + c).ceil() as i32;
                let expected_ijk_abc_abc_width = (i + j + k + space + a + b + c + space + a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(3, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(11, text.lines[0].end);
                assert_eq!(expected_abc_ijk_abc_width, text.lines[0].width);
                assert_eq!(12, text.lines[1].start);
                assert_eq!(23, text.lines[1].end);
                assert_eq!(expected_ijk_abc_abc_width, text.lines[1].width);
                assert_eq!(24, text.lines[2].start);
                assert_eq!(27, text.lines[2].end);
                assert_eq!(expected_ijk_width, text.lines[2].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_trimmed_text_with_newlines()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc\nijk\n  \n", TextAlign::Left);
        let width = (a + b + c + i + j + k).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(4, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(7, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(10, text.lines[2].start);
                assert_eq!(10, text.lines[2].end);
                assert_eq!(0, text.lines[2].width);
                assert_eq!(11, text.lines[3].start);
                assert_eq!(11, text.lines[3].end);
                assert_eq!(0, text.lines[3].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_trimmed_text_and_ellipsize()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk abc", TextAlign::Left);
        text.ellipsize_count = Some(2);
        let width = (a + b + c).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ij_width = (i + j).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(6, text.lines[1].end);
                assert_eq!(expected_ij_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(true, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }    
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk abc", TextAlign::Left);
        text.is_trimmed = false;
        match text.update_size(&cairo_context, Size::new(None, None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_ijk_abc_width = (a + b + c + space + i + j + k + space + a + b + c).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(11, text.lines[0].end);
                assert_eq!(expected_abc_ijk_abc_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text_with_spaces()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("   abc  ijk abc  ", TextAlign::Left);
        text.is_trimmed = false;
        match text.update_size(&cairo_context, Size::new(None, None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_ijk_abc_width = (space * 3.0 + a + b + c + space * 2.0 + i + j + k + space + a + b + c + space * 2.0).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(17, text.lines[0].end);
                assert_eq!(expected_abc_ijk_abc_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_untrimmed_text_and_line_break()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk", TextAlign::Left);
        text.is_trimmed = false;
        let width = (a + b + c + space + i).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(7, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text_and_line_break_by_many_spaces()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc   ijk", TextAlign::Left);
        text.is_trimmed = false;
        let width = (a + b + c + space + space).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (space * 2.0 + i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(9, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text_and_word_break()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("aaabbbccc", TextAlign::Left);
        text.is_trimmed = false;
        let width = (a * 3.0 + b * 3.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_aaabbb_width = (a * 3.0 + b * 3.0).ceil() as i32;
                let expected_ccc_width = (c * 3.0).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(6, text.lines[0].end);
                assert_eq!(expected_aaabbb_width, text.lines[0].width);
                assert_eq!(6, text.lines[1].start);
                assert_eq!(9, text.lines[1].end);
                assert_eq!(expected_ccc_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text_and_line_break_and_many_lines()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk abc ijk abc abc jik", TextAlign::Left);
        text.is_trimmed = false;
        let width = (a + b + c + space + i + j + k + space + a + b + c).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_ijk_abc_width = (a + b + c + space + i + j + k + space + a + b + c).ceil() as i32;
                let expected_ijk_abc_abc_width = (i + j + k + space + a + b + c + space + a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(3, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(11, text.lines[0].end);
                assert_eq!(expected_abc_ijk_abc_width, text.lines[0].width);
                assert_eq!(12, text.lines[1].start);
                assert_eq!(23, text.lines[1].end);
                assert_eq!(expected_ijk_abc_abc_width, text.lines[1].width);
                assert_eq!(24, text.lines[2].start);
                assert_eq!(27, text.lines[2].end);
                assert_eq!(expected_ijk_width, text.lines[2].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text_with_newlines()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let space = cairo_context.text_extents(" ").unwrap().x_advance;
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let k = cairo_context.text_extents("k").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc\nijk\n  \n", TextAlign::Left);
        text.is_trimmed = false;
        let width = (a + b + c + i + j + k).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ijk_width = (i + j + k).ceil() as i32;
                let expected_space2_width = (space * 2.0).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(4, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(7, text.lines[1].end);
                assert_eq!(expected_ijk_width, text.lines[1].width);
                assert_eq!(8, text.lines[2].start);
                assert_eq!(10, text.lines[2].end);
                assert_eq!(expected_space2_width, text.lines[2].width);
                assert_eq!(11, text.lines[3].start);
                assert_eq!(11, text.lines[3].end);
                assert_eq!(0, text.lines[3].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_untrimmed_text_and_ellipsize()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let b = cairo_context.text_extents("b").unwrap().x_advance;
        let c = cairo_context.text_extents("c").unwrap().x_advance;
        let i = cairo_context.text_extents("i").unwrap().x_advance;
        let j = cairo_context.text_extents("j").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("abc ijk abc", TextAlign::Left);
        text.is_trimmed = false;
        text.ellipsize_count = Some(2);
        let width = (a + b + c).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_abc_width = (a + b + c).ceil() as i32;
                let expected_ij_width = (i + j).ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(3, text.lines[0].end);
                assert_eq!(expected_abc_width, text.lines[0].width);
                assert_eq!(4, text.lines[1].start);
                assert_eq!(6, text.lines[1].end);
                assert_eq!(expected_ij_width, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(true, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_half_size_of_letter()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let a = cairo_context.text_extents("a").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("aaa", TextAlign::Left);
        let width = (a / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_a_width = a.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(3, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(1, text.lines[0].end);
                assert_eq!(expected_a_width, text.lines[0].width);
                assert_eq!(1, text.lines[1].start);
                assert_eq!(2, text.lines[1].end);
                assert_eq!(expected_a_width, text.lines[1].width);
                assert_eq!(2, text.lines[2].start);
                assert_eq!(3, text.lines[2].end);
                assert_eq!(expected_a_width, text.lines[2].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_half_size_of_unicode_letter()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let l = cairo_context.text_extents("\u{0142}").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("\u{0142}\u{0142}\u{0142}", TextAlign::Left);
        let width = (l / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_l_width = l.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(3, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(2, text.lines[0].end);
                assert_eq!(expected_l_width, text.lines[0].width);
                assert_eq!(2, text.lines[1].start);
                assert_eq!(4, text.lines[1].end);
                assert_eq!(expected_l_width, text.lines[1].width);
                assert_eq!(4, text.lines[2].start);
                assert_eq!(6, text.lines[2].end);
                assert_eq!(expected_l_width, text.lines[2].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_combination_of_characters()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let l = cairo_context.text_extents("a\u{0306}\u{0320}").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("a\u{0306}\u{0320}", TextAlign::Left);
        let width = (l / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_l_width = l.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(5, text.lines[0].end);
                assert_eq!(expected_l_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_double_combination_of_characters()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let l = cairo_context.text_extents("a\u{0306}\u{035c}\u{0320}b\u{035c}c").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("a\u{0306}\u{035c}\u{0320}b\u{035c}c", TextAlign::Left);
        let width = (l / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_l_width = l.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(11, text.lines[0].end);
                assert_eq!(expected_l_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_double_combination_of_characters_without_last_character()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let l = cairo_context.text_extents("a\u{0306}\u{035c}\u{0320}b\u{035c}").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("a\u{0306}\u{035c}\u{0320}b\u{035c}", TextAlign::Left);
        let width = (l / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_l_width = l.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(10, text.lines[0].end);
                assert_eq!(expected_l_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_text_updates_size_for_double_combination_of_characters_without_last_character_and_newline()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let l = cairo_context.text_extents("a\u{0306}\u{035c}\u{0320}b\u{035c}").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new("a\u{0306}\u{035c}\u{0320}b\u{035c}\n", TextAlign::Left);
        let width = (l / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_l_width = l.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(2, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(10, text.lines[0].end);
                assert_eq!(expected_l_width, text.lines[0].width);
                assert_eq!(11, text.lines[1].start);
                assert_eq!(11, text.lines[1].end);
                assert_eq!(0, text.lines[1].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_text_updates_size_for_combination_of_characters_with_space()
    {
        let cairo_surface = create_dummy_cairo_surface().unwrap();
        let cairo_context = CairoContext::new(&cairo_surface).unwrap();
        cairo_context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cairo_context.set_font_size(16.0);
        let l = cairo_context.text_extents(" \u{0306}\u{0320}").unwrap().x_advance;
        let height = cairo_context.font_extents().unwrap().height;
        let mut text = Text::new(" \u{0306}\u{0320}", TextAlign::Left);
        let width = (l / 2.0).ceil() as i32;
        match text.update_size(&cairo_context, Size::new(Some(width), None), |_| Ok(())) {
            Ok(()) => {
                let expected_l_width = l.ceil() as i32;
                let expected_line_height = height.ceil() as i32;
                assert_eq!(1, text.lines.len());
                assert_eq!(0, text.lines[0].start);
                assert_eq!(5, text.lines[0].end);
                assert_eq!(expected_l_width, text.lines[0].width);
                assert_eq!(expected_line_height, text.line_height);
                assert_eq!(false, text.has_dot_dot_dot);
            },
            Err(_) => assert!(false),
        }
    }    
}
