//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use cairo::Format;
use cairo::ImageSurface;
use crate::client_error::*;
use crate::types::*;

pub fn create_dummy_cairo_surface() -> Result<ImageSurface, ClientError>
{
    match ImageSurface::create(Format::ARgb32, 1, 1) {
        Ok(surface) => Ok(surface),
        Err(err) => Err(ClientError::Cairo(err)),
    }
}

pub fn with_dummy_cairo_context<T, F>(f: F) -> Result<T, ClientError>
    where F: FnOnce(&CairoContext) -> T
{
    match create_dummy_cairo_surface() {
        Ok(cairo_surface) => {
            match CairoContext::new(&cairo_surface) {
                Ok(cairo_context) => Ok(f(&cairo_context)),
                Err(err) => Err(ClientError::Cairo(err)),
            }
        },
        Err(err) => Err(err),
    }
}
