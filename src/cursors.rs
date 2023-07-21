//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of cursors.
//!
//! The module of cursors contains a cursor enumeration.
/// A cursor enumeration.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Cursor
{
    /// A default cursor.
    Default,
    /// A text cursor.
    Text,
    /// A hand cursor.
    Hand,
    /// A pencil cursor.
    Pencil,
    /// A cross cursor.
    Cross,
    /// A wait cursor.
    Wait,
    /// A cursor of top left corner.
    TopLeftCorner,
    /// A cursor of top right corner.
    TopRightCorner,
    /// A cursor of top side.
    TopSide,
    /// A cursor of left side.
    LeftSide,
    /// A cursor of bottom left corner.
    BottomLeftCorner,
    /// A cursor of bottom right corner.
    BottomRightCorner,
    /// A cursor of bottom side.
    BottomSide,
    /// A cursor of right side.
    RightSide,
    /// A cursor of horizontal double arrow.
    HDoubleArrow,
    /// A cursor of vertical double arrow.
    VDoubleArrow,
}
