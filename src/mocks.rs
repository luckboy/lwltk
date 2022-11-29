//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
#[allow(dead_code)]
mod mock_child_window;
#[allow(dead_code)]
mod mock_empty_window;
#[allow(dead_code)]
mod mock_layout;
#[allow(dead_code)]
mod mock_parent_window;
#[allow(dead_code)]
mod mock_widget;
#[allow(dead_code)]
mod mock_window;

pub(crate) use mock_child_window::*;
pub(crate) use mock_empty_window::*;
pub(crate) use mock_layout::*;
pub(crate) use mock_parent_window::*;
pub(crate) use mock_widget::*;
pub(crate) use mock_window::*;
