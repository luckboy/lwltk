//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::iter::FusedIterator;
use crate::call_on::*;
use crate::draw::*;
use crate::types::*;
use crate::widget::*;

pub trait Container: Draw + CallOn
{
    #[allow(unused_variables)]
    fn prev(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { None }

    #[allow(unused_variables)]
    fn next(&self, idx_pair: Option<WidgetIndexPair>) -> Option<WidgetIndexPair>
    { None }
    
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
        let mut idx_pair_iter = path.widget_index_pairs();
        match idx_pair_iter.next() {
            Some(idx_pair) => {
                let mut widget: Option<&'a dyn Widget> = self.dyn_widget_for_index_pair(idx_pair);
                for idx_pair in idx_pair_iter {
                    match widget {
                        Some(tmp_widget) => widget = tmp_widget.dyn_widget_for_index_pair(idx_pair),
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
        let mut idx_pair_iter = path.widget_index_pairs();
        match idx_pair_iter.next() {
            Some(idx_pair) => {
                let mut widget: Option<&'a mut dyn Widget> = self.dyn_widget_mut_for_index_pair(idx_pair);
                for idx_pair in idx_pair_iter {
                    match widget {
                        Some(tmp_widget) => widget = tmp_widget.dyn_widget_mut_for_index_pair(idx_pair),
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

    fn point_focusable_to_leaf(&self, pos: Pos<f64>) -> Option<RelWidgetPath>
    {
        match self.point(pos) {
            Some(idx_pair) => {
                let mut widget_path = RelWidgetPath::new(idx_pair);
                let mut focusable_widget_path: Option<RelWidgetPath> = None;
                let mut widget: Option<&'_ dyn Widget> = self.dyn_widget_for_index_pair(idx_pair);
                match widget {
                    Some(tmp_widget) if tmp_widget.is_focusable() => focusable_widget_path = Some(widget_path.clone()),
                    _ => (),
                }
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
                    match widget {
                        Some(tmp_widget) if tmp_widget.is_focusable() => focusable_widget_path = Some(widget_path.clone()),
                        _ => (),
                    }
                    widget = match widget {
                        Some(tmp_widget) => tmp_widget.dyn_widget_for_index_pair(idx_pair),
                        None => None,
                    }
                }
                focusable_widget_path
            },
            None => None,
        }
    }
}

pub struct RevWidgetIndexPairs<'a>
{
    container: &'a dyn Container,
    widget_index_pair: Option<Option<WidgetIndexPair>>,
}

impl<'a> RevWidgetIndexPairs<'a>
{
    pub fn new(container: &'a dyn Container) -> Self
    { RevWidgetIndexPairs { container, widget_index_pair: Some(None), } }
}

impl<'a> FusedIterator for RevWidgetIndexPairs<'a>
{}

impl<'a> Iterator for RevWidgetIndexPairs<'a>
{
    type Item = WidgetIndexPair;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.widget_index_pair {
            Some(idx_pair) => {
                let next_idx_pair = self.container.prev(idx_pair);
                self.widget_index_pair = match next_idx_pair {
                    Some(_) => Some(next_idx_pair),
                    None => None,
                };
                next_idx_pair
            },
            None => None,
        }
        
    }
}

pub struct WidgetIndexPairs<'a>
{
    container: &'a dyn Container,
    widget_index_pair: Option<Option<WidgetIndexPair>>,
}

impl<'a> WidgetIndexPairs<'a>
{
    pub fn new(container: &'a dyn Container) -> Self
    { WidgetIndexPairs { container, widget_index_pair: Some(None), } }
}

impl<'a> FusedIterator for WidgetIndexPairs<'a>
{}

impl<'a> Iterator for WidgetIndexPairs<'a>
{
    type Item = WidgetIndexPair;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.widget_index_pair {
            Some(idx_pair) => {
                let next_idx_pair = self.container.next(idx_pair);
                self.widget_index_pair = match next_idx_pair {
                    Some(_) => Some(next_idx_pair),
                    None => None,
                };
                next_idx_pair
            },
            None => None,
        }
        
    }
}

pub struct RevWidgets<'a>
{
    iter: RevWidgetIndexPairs<'a>,
}

impl<'a> RevWidgets<'a>
{
    pub fn new(container: &'a dyn Container) -> Self
    { RevWidgets { iter: RevWidgetIndexPairs::new(container), } }
}

impl<'a> FusedIterator for RevWidgets<'a>
{}

impl<'a> Iterator for RevWidgets<'a>
{
    type Item = &'a dyn Widget;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(idx_pair) => self.iter.container.dyn_widget_for_index_pair(idx_pair),
            None => None,
        }
    }
}

pub struct Widgets<'a>
{
    iter: WidgetIndexPairs<'a>,
}

impl<'a> Widgets<'a>
{
    pub fn new(container: &'a dyn Container) -> Self
    { Widgets { iter: WidgetIndexPairs::new(container), } }
}

impl<'a> FusedIterator for Widgets<'a>
{}

impl<'a> Iterator for Widgets<'a>
{
    type Item = &'a dyn Widget;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(idx_pair) => self.iter.container.dyn_widget_for_index_pair(idx_pair),
            None => None,
        }
    }
}

pub fn container_widget<'a, C: Container + ?Sized, T: Any>(container: &'a C, path: &RelWidgetPath) -> Option<&'a T>
{ container.dyn_widget(path).map(|wg| wg.as_any().downcast_ref::<T>()).flatten() }

pub fn container_widget_mut<'a, C: Container + ?Sized, T: Any>(container: &'a mut C, path: &RelWidgetPath) -> Option<&'a mut T>
{ container.dyn_widget_mut(path).map(|wg| wg.as_any_mut().downcast_mut::<T>()).flatten() }

pub fn container_rel_widget_path1<'a, C: Container, F>(container: &'a mut C, f: F) -> Option<RelWidgetPath>
    where F: FnOnce(&mut C) -> Option<WidgetIndexPair>
{
    match f(container) {
        Some(idx_pair) => Some(RelWidgetPath::new(idx_pair)),
        None => None,
    }
}

pub fn container_rel_widget_path<'a, C: Container + ?Sized, T: Any, F>(container: &'a mut C, path: &RelWidgetPath, f: F) -> Option<RelWidgetPath>
    where F: FnOnce(&mut T) -> Option<WidgetIndexPair>
{
    match container_widget_mut(container, path) {
        Some(widget) => {
            match f(widget) {
                Some(idx_pair) => {
                    let mut new_path = path.clone();
                    new_path.push(idx_pair);
                    Some(new_path)
                },
                None => None,
            }
        },
        None => None,
    }
}
