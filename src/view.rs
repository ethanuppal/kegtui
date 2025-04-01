// Copyright (C) 2024 Ethan Uppal.
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{borrow::Cow, collections::HashMap};

use color_eyre::eyre::Result;
use ratatui::{layout::Rect, DefaultTerminal, Frame};

use crate::app::App;

pub mod prelude {
    pub use super::*;
    pub use color_eyre::Result;
    pub use ratatui::{
        prelude::*,
        widgets::{
            Cell, Paragraph, Row, ScrollDirection, Scrollbar,
            ScrollbarOrientation, ScrollbarState, Table,
        },
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewID<'a> {
    Index(usize),
    Named(&'a str),
}

#[derive(Clone, Copy)]
pub enum NavID<'a> {
    Index(usize),
    Named(&'a str),
}

#[derive(Clone)]
pub enum NavAction<'a> {
    Pop,
    Push(NavID<'a>),
}

#[derive(Clone)]
pub enum MenuItemAction<'a> {
    NavAction(NavAction<'a>),
    LoadView(ViewID<'a>),
}

pub struct MenuItem<'a> {
    name: Cow<'a, str>,
    is_default: bool,
    action: MenuItemAction<'a>,
}

impl<'a> MenuItem<'a> {
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        action: MenuItemAction<'a>,
    ) -> Self {
        Self {
            name: name.into(),
            is_default: false,
            action,
        }
    }

    pub fn default(mut self) -> Self {
        self.is_default = true;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn action(&self) -> &MenuItemAction<'a> {
        &self.action
    }
}

pub struct Nav<'a> {
    menu: Vec<MenuItem<'a>>,
    default_item: usize,
}

impl<'a> Nav<'a> {
    pub fn menu(&self) -> &[MenuItem<'a>] {
        &self.menu
    }

    pub fn default_item(&self) -> usize {
        self.default_item
    }
}

pub trait View {
    ///// The menu items this view exposes.
    //fn menu<'a>(&self, state: &State) -> Vec<MenuItem<'a>>;

    /// Draw the views content.
    fn draw_content(
        &self,
        state: &App,
        frame: &mut Frame<'_>,
        area: Rect,
    ) -> Result<()> {
        let _ = (state, frame, area);
        Ok(())
    }

    /// Clickable objects currently drawn.
    fn clickables(&self, state: &App) -> Vec<usize> {
        let _ = state;
        vec![]
    }

    /// Notifies that a clickable has been selected.
    fn select(&mut self, state: &mut App, index: usize) {
        let _ = (state, index);
    }

    fn click(&mut self, state: &mut App, index: usize) -> Option<NavAction> {
        let _ = (state, index);
        None
    }
}

#[derive(Default)]
pub struct NavContext<'a> {
    views: Vec<&'a dyn View>,
    named_view_ids: HashMap<&'a str, usize>,
    navs: Vec<Nav<'a>>,
    named_nav_ids: HashMap<&'a str, usize>,
    stack: Vec<NavID<'a>>,
}

impl<'a> NavContext<'a> {
    pub fn view<V: View + 'a>(
        &mut self,
        name: &'a str,
        view: &'a V,
    ) -> ViewID<'a> {
        self.views.push(view);
        self.named_view_ids.insert(name, self.views.len() - 1);
        ViewID::Index(self.views.len() - 1)
    }

    pub fn nav(
        &mut self,
        name: &'a str,
        menu: impl IntoIterator<Item = MenuItem<'a>>,
    ) -> NavID<'a> {
        let menu = menu.into_iter().collect::<Vec<_>>();
        let default_item = menu
            .iter()
            .enumerate()
            .find(|(_, item)| item.is_default)
            .map(|(index, _)| index)
            .unwrap_or(0);
        assert!(!menu.is_empty());
        self.navs.push(Nav { menu, default_item });
        self.named_nav_ids.insert(name, self.navs.len() - 1);
        NavID::Index(self.navs.len() - 1)
    }

    pub fn push_nav(&mut self, nav: NavID<'a>) {
        self.stack.push(nav);
    }

    pub fn pop_nav(&mut self) {
        let _ = self.stack.pop();
    }

    pub fn top_nav(&self) -> Option<NavID<'a>> {
        self.stack.last().copied()
    }

    pub fn get_view_ref(&self, id: ViewID<'a>) -> &'a dyn View {
        self.views[self.get_view_index(id)]
    }

    pub fn get_nav_ref(&self, id: NavID<'a>) -> &Nav<'a> {
        &self.navs[self.get_nav_index(id)]
    }

    fn get_view_index(&self, id: ViewID<'a>) -> usize {
        match id {
            ViewID::Index(index) => index,
            ViewID::Named(name) => {
                self.named_view_ids.get(name).copied().unwrap()
            }
        }
    }

    fn get_nav_index(&self, id: NavID<'a>) -> usize {
        match id {
            NavID::Index(index) => index,
            NavID::Named(name) => {
                self.named_nav_ids.get(name).copied().unwrap()
            }
        }
    }
}
