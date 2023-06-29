//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub struct RadioGroup
{
    selected: AtomicUsize,
    count: AtomicUsize,
}

impl RadioGroup
{
    pub fn new() -> RadioGroup
    { RadioGroup { selected: AtomicUsize::new(0), count: AtomicUsize::new(0), } }
    
    pub fn selected(&self) -> usize
    { self.selected.load(Ordering::SeqCst) }
    
    pub fn select(&self, selected: usize) -> usize
    {
        self.selected.store(selected, Ordering::SeqCst);
        selected
    }

    pub fn count(&self) -> usize
    { self.count.load(Ordering::SeqCst) }    
    
    pub fn increase_count(&self) -> usize
    { self.count.fetch_add(1, Ordering::SeqCst) + 1 }
}
