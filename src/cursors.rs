//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Cursor
{
    Default,
    Text,
    Hand,
    Pencil,
    Cross,
    Wait,
    TopLeftCorner,
    TopRightCorner,
    TopSide,
    LeftSide,
    BottomLeftCorner,
    BottomRightCorner,
    BottomSide,
    RightSide,
    HDoubleArrow,
    VDoubleArrow,
}
