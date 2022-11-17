//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::container::*;
use crate::preferred_size::*;
use crate::types::*;

pub trait Widget: Container + PreferredSize
{
    fn margin_bounds(&self) -> Rect<i32>;
    
    fn bounds(&self) -> Rect<i32>;
    
    #[allow(unused_variables)]
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { None }

    #[allow(unused_variables)]
    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { None }

    fn margin_pos(&self) -> Pos<i32>
    { self.margin_bounds().pos() }

    fn margin_size(&self) -> Size<i32>
    { self.margin_bounds().size() }

    fn margin_x(&self) -> i32
    { self.margin_bounds().x }

    fn margin_y(&self) -> i32
    { self.margin_bounds().y }

    fn margin_width(&self) -> i32
    { self.margin_bounds().width }

    fn margin_height(&self) -> i32
    { self.margin_bounds().height }

    fn pos(&self) -> Pos<i32>
    { self.bounds().pos() }

    fn size(&self) -> Size<i32>
    { self.bounds().size() }

    fn x(&self) -> i32
    { self.bounds().x }

    fn y(&self) -> i32
    { self.bounds().y }

    fn width(&self) -> i32
    { self.bounds().width }

    fn height(&self) -> i32
    { self.bounds().height }
}
