//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashSet;
use crate::keys::*;

pub(crate) fn initialize_modifier_keys(modifier_keys: &mut HashSet<VKey>)
{
    modifier_keys.insert(VKey::LeftShift);
    modifier_keys.insert(VKey::RightShift);
    modifier_keys.insert(VKey::LeftCtrl);
    modifier_keys.insert(VKey::RightCtrl);
    modifier_keys.insert(VKey::CapsLock);
    modifier_keys.insert(VKey::ShiftLock);
    modifier_keys.insert(VKey::LeftMeta);
    modifier_keys.insert(VKey::RightMeta);
    modifier_keys.insert(VKey::LeftAlt);
    modifier_keys.insert(VKey::RightAlt);
    modifier_keys.insert(VKey::LeftSuper);
    modifier_keys.insert(VKey::RightSuper);
    modifier_keys.insert(VKey::LeftHyper);
    modifier_keys.insert(VKey::RightHyper);
}
