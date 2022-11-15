//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::client_error::*;

pub trait Theme: Send + Sync
{}

pub fn theme_from_env() -> Result<Box<dyn Theme>, ClientError>
{ Err(ClientError::InvalidThemeName) }
