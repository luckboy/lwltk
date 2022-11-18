//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use crate::call_on::*;
use crate::draw::*;
use crate::types::*;
use crate::widget::*;

pub trait Container: Draw + CallOn
{
    #[allow(unused_variables)]
    fn dyn_widget_for_index_pair(&self, idx_pair: WidgetIndexPair) -> Option<&dyn Widget>
    { None }

    #[allow(unused_variables)]
    fn dyn_widget_mut_for_index_pair(&mut self, idx_pair: WidgetIndexPair) -> Option<&mut dyn Widget>
    { None }
    
    #[allow(unused_variables)]
    fn point(&self, pos: Pos<f64>) -> Option<WidgetIndexPair>
    { None }

    fn dyn_widget<'a>(&'a self, path: &RelWidgetPath) -> Option<&'a dyn Widget>
    {
        let mut idx_pair_iter = path.widget_index_pairs().iter();
        match idx_pair_iter.next() {
            Some(idx_pair) => {
                let mut widget: Option<&'a dyn Widget> = self.dyn_widget_for_index_pair(*idx_pair);
                for idx_pair in idx_pair_iter {
                    match widget {
                        Some(tmp_widget) => widget = tmp_widget.dyn_widget_for_index_pair(*idx_pair),
                        None => break,
                    }
                }
                widget
            },
            None => None,
        }
    }

    fn dyn_widget_mut<'a>(&'a mut self, path: &RelWidgetPath) -> Option<&'a mut dyn Widget>
    {
        let mut idx_pair_iter = path.widget_index_pairs().iter();
        match idx_pair_iter.next() {
            Some(idx_pair) => {
                let mut widget: Option<&'a mut dyn Widget> = self.dyn_widget_mut_for_index_pair(*idx_pair);
                for idx_pair in idx_pair_iter {
                    match widget {
                        Some(tmp_widget) => widget = tmp_widget.dyn_widget_mut_for_index_pair(*idx_pair),
                        None => break,
                    }
                }
                widget
            },
            None => None,
        }
    }
    
    fn point_to_leaf(&self, pos: Pos<f64>) -> Option<RelWidgetPath>
    {
        match self.point(pos) {
            Some(idx_pair) => {
                let mut widget_path = RelWidgetPath::new(idx_pair);
                let mut widget: Option<&'_ dyn Widget> = self.dyn_widget_for_index_pair(idx_pair);
                loop {
                    let idx_pair = match widget {
                        Some(tmp_widget) => {
                            match tmp_widget.point(pos) {
                                Some(tmp_idx_pair) => tmp_idx_pair,
                                None => break,
                            }
                        },
                        None => break,
                    };
                    widget_path.push(idx_pair);
                    widget = match widget {
                        Some(tmp_widget) => tmp_widget.dyn_widget_for_index_pair(idx_pair),
                        None => None,
                    }
                }
                Some(widget_path)
            },
            None => None,
        }
    }
}

pub fn container_widget<'a, C: Container, T: Any>(container: &'a C, path: &RelWidgetPath) -> Option<&'a T>
{ container.dyn_widget(path).map(|wg| wg.as_any().downcast_ref::<T>()).flatten() }

pub fn container_widget_mut<'a, C: Container, T: Any>(container: &'a mut C, path: &RelWidgetPath) -> Option<&'a mut T>
{ container.dyn_widget_mut(path).map(|wg| wg.as_any_mut().downcast_mut::<T>()).flatten() }
