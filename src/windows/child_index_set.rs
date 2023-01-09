//
// Copyright (c) 2023 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::btree_set;
use std::collections::BTreeSet;
use crate::types::*;
use crate::window::*;

struct WindowIter<'a>
{
    iter: btree_set::Iter<'a, WindowIndex>,
}

impl<'a> WindowIter<'a>
{
    fn new(child_indices: &'a BTreeSet<WindowIndex>) -> Self
    { WindowIter { iter: child_indices.iter(), } }
}

impl<'a> WindowIterator<'a> for WindowIter<'a>
{
    fn next(&mut self) -> Option<WindowIndex>
    { self.iter.next().map(|i| *i) }
}

pub struct ChildIndexSet
{
    pub child_indices: BTreeSet<WindowIndex>,
}

impl ChildIndexSet
{
    pub fn new() -> Self
    { ChildIndexSet { child_indices: BTreeSet::new(), } }
    
    pub fn child_index_iter(&self) -> Option<Box<dyn WindowIterator + '_>>
    { Some(Box::new(WindowIter::new(&self.child_indices))) }

    pub fn add(&mut self, idx: ChildWindowIndex) -> Option<()>
    {
        if self.child_indices.insert(idx.window_index()) {
            Some(())
        } else {
            None
        }
    }

    pub fn remove(&mut self, idx: ChildWindowIndex) -> Option<()>
    {
        if self.child_indices.remove(&idx.window_index()) {
            Some(())
        } else {
            None
        }
    }
}
