//
// Copyright (c) 2022-2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;

/// A trait for downcasting.
pub trait AsAny
{
    /// Returns a reference to the object for downcasting.
    fn as_any(&self) -> &dyn Any;
    
    /// Returns a mutable reference to the object for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
