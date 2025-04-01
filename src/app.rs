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

use ratatui::DefaultTerminal;

use crate::view::prelude::*;

#[derive(Default)]
pub struct App {
    pub(crate) vertical_scroll: usize,
}

impl App {
    pub fn run<'a>(
        &mut self,
        context: &mut NavContext<'a, App>,
        initial: NavID<'a>,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        Ok(())
    }
}
