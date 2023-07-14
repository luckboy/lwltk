//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::types::*;

/// A trait of preferred size.
///
/// The trait of preferred size allows to have access to the preferred size. A widget with the
/// preferred size has a size as the preferred size or greater if an area size allows to it. A window
/// with the preferred size has a size as the prefferes size if a minimal size allows to it. The
/// preferred size has an optional width and an optional height.
pub trait PreferredSize: Send + Sync
{
    /// Returns the preferred size.
    fn preferred_size(&self) -> Size<Option<i32>>;
    
    /// Sets the preferred size.
    fn set_preferred_size(&mut self, size: Size<Option<i32>>);
    
    /// Returns the optional preferred width.
    fn preferred_width(&self) -> Option<i32>
    { self.preferred_size().width }

    /// Returns the optional preferred height.
    fn preferred_height(&self) -> Option<i32>
    { self.preferred_size().height }    
    
    /// Sets the optional preferred width.
    fn set_preferred_width(&mut self, width: Option<i32>)
    {
        let mut size = self.preferred_size();
        size.width = width;
        self.set_preferred_size(size);
    }

    /// Sets the optional preferred height.
    fn set_preferred_height(&mut self, height: Option<i32>)
    {
        let mut size = self.preferred_size();
        size.height = height;
        self.set_preferred_size(size);
    }
}
