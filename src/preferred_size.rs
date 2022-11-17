//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::types::*;

pub trait PreferredSize: Send + Sync
{
    fn preferred_size(&self) -> Size<Option<i32>>;
    
    fn set_preferred_size(&mut self, size: Size<Option<i32>>);
    
    fn preferred_width(&self) -> Option<i32>
    { self.preferred_size().width }

    fn preferred_height(&self) -> Option<i32>
    { self.preferred_size().height }    
    
    fn set_preferred_width(&mut self, width: Option<i32>)
    {
        let mut size = self.preferred_size();
        size.width = width;
        self.set_preferred_size(size);
    }

    fn set_preferred_height(&mut self, height: Option<i32>)
    {
        let mut size = self.preferred_size();
        size.height = height;
        self.set_preferred_size(size);
    }
}
