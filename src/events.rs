//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
#[derive(Clone)]
pub enum Event
{
    Click,
}

#[derive(Clone)]
pub enum EventOption
{
    None,
    Default,
    Some(Event),
}
