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

#[derive(Clone, Copy)]
pub enum ViewID<'a> {
    Index(usize),
    Named(&'a str),
}

#[derive(Clone, Copy)]
pub enum NavID<'a> {
    Index(usize),
    Named(&'a str),
}

pub enum NavAction<'a> {
    Pop,
    Push(NavID<'a>),
}

pub enum MenuItemAction<'a> {
    NavAction(NavAction<'a>),
    LoadView(ViewID<'a>),
}

pub struct MenuItem<'a> {
    name: Cow<'a, str>,
    action: MenuItemAction<'a>,
}

impl<'a> MenuItem<'a> {
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        action: MenuItemAction<'a>,
    ) -> Self {
        Self {
            name: name.into(),
            action,
        }
    }
}

pub struct Nav<'a> {
    menu: Vec<MenuItem<'a>>,
}

pub trait View<State> {
    ///// The menu items this view exposes.
    //fn menu<'a>(&self, state: &State) -> Vec<MenuItem<'a>>;

    /// Draw the views content.
    fn draw_content(
        &self,
        state: &State,
        frame: &mut Frame<'_>,
        area: Rect,
    ) -> Result<()> {
        let _ = (state, frame, area);
        Ok(())
    }

    /// Clickable objects currently drawn.
    fn clickables(&self, state: &State) -> Vec<usize> {
        let _ = state;
        vec![]
    }

    /// Notifies that a clickable has been selected.
    fn select(&mut self, state: &mut State, index: usize) {
        let _ = (state, index);
    }

    fn click(&mut self, state: &mut State, index: usize) -> Option<NavAction> {
        let _ = (state, index);
        None
    }
}

#[derive(Default)]
pub struct NavContext<'a, State> {
    views: Vec<&'a dyn View<State>>,
    named_view_ids: HashMap<&'a str, usize>,
    navs: Vec<Nav<'a>>,
    named_nav_ids: HashMap<&'a str, usize>,
    stack: Vec<NavID<'a>>,
}

impl<'a, State> NavContext<'a, State> {
    pub fn view<V: View<State> + 'a>(&mut self, view: &'a V) -> ViewID<'a> {
        self.views.push(view);
        ViewID::Index(self.views.len() - 1)
    }

    pub fn nav(
        &mut self,
        menu: impl IntoIterator<Item = MenuItem<'a>>,
    ) -> NavID<'a> {
        self.navs.push(Nav {
            menu: menu.into_iter().collect(),
        });
        NavID::Index(self.navs.len() - 1)
    }

    pub fn run_ratatui(
        &mut self,
        initial: NavID<'a>,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        Ok(())
    }
}
