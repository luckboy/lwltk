//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;

pub trait AsAny
{
    fn as_any(&self) -> &dyn Any;
    
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
