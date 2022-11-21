//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::types::*;

pub trait MinSize: Send + Sync
{
    fn min_size(&self) -> Size<Option<i32>>;
    
    fn set_min_size(&mut self, size: Size<Option<i32>>);
    
    fn min_width(&self) -> Option<i32>
    { self.min_size().width }

    fn min_height(&self) -> Option<i32>
    { self.min_size().height }

    fn set_min_width(&mut self, width: Option<i32>)
    {
        let mut size = self.min_size();
        size.width = width;
        self.set_min_size(size);
    }

    fn set_min_height(&mut self, height: Option<i32>)
    {
        let mut size = self.min_size();
        size.height = height;
        self.set_min_size(size);
    }    
}
