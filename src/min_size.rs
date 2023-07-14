//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::types::*;

/// A trait of minimal size.
///
/// The trait of minimal size allows to have access to the minimal size. A window can't have the
/// width or the height that is less than the minimal width or the minimal height. The minimal size
/// has an optional width and an optional height.
pub trait MinSize: Send + Sync
{
    /// Returns the minimal size.
    fn min_size(&self) -> Size<Option<i32>>;
    
    /// Sets the minimal size.
    fn set_min_size(&mut self, size: Size<Option<i32>>);
    
    /// Returns the optional minimal width.
    fn min_width(&self) -> Option<i32>
    { self.min_size().width }

    /// Returns the optional minimal height.
    fn min_height(&self) -> Option<i32>
    { self.min_size().height }

    /// Sets the optional minimal width.
    fn set_min_width(&mut self, width: Option<i32>)
    {
        let mut size = self.min_size();
        size.width = width;
        self.set_min_size(size);
    }

    /// Sets the optional minimal height.
    fn set_min_height(&mut self, height: Option<i32>)
    {
        let mut size = self.min_size();
        size.height = height;
        self.set_min_size(size);
    }    
}
