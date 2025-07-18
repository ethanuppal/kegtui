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

use ratatui::widgets::{List, ListItem, ListState, Wrap};

use crate::{
    app::{App, AsyncState, SELECTED_FOCUSED_STYLE, SELECTED_UNFOCUSED_STYLE},
    view::prelude::*,
};

pub struct KegsView;

impl View for KegsView {
    fn draw_content(
        &self,
        app: &App,
        state: &AsyncState,
        frame: &mut Frame,
        area: Rect,
        is_focused: bool,
    ) -> Result<()> {
        let text = "Select a Keg (kegs are searched under /Applications/, ~/Applications/, and ~/Applications/Kegworks/):";
        let wrapped = textwrap::wrap(&text, area.width as usize);
        let para_height = wrapped.len().max(1) as u16; // number of lines

        let title_para = Paragraph::new(text).wrap(Wrap { trim: false });
        frame.render_widget(
            title_para,
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: para_height,
            },
        );

        let list_area = Rect {
            x: area.x,
            y: area.y + para_height,
            width: area.width,
            height: area.height.saturating_sub(para_height),
        };

        if !state.kegs.is_empty() {
            let keg_items = state
                .kegs
                .iter()
                .cloned()
                .map(|keg| {
                    ListItem::new(Span::from(format!(
                        "{} (under {})",
                        keg.name,
                        keg.enclosing_location.display()
                    )))
                })
                .collect::<Vec<_>>();

            let mut list_state = ListState::default();
            list_state.select(Some(app.interaction_state()));
            let list = List::new(keg_items)
                .highlight_style(if is_focused {
                    SELECTED_FOCUSED_STYLE
                } else {
                    SELECTED_UNFOCUSED_STYLE
                })
                .highlight_symbol(">> ");
            frame.render_stateful_widget(list, list_area, &mut list_state);
        }

        Ok(())
    }

    fn interactivity(
        &self,
        _app: &App,
        state: &AsyncState,
    ) -> Result<ViewInteractivity> {
        Ok(if state.kegs.is_empty() {
            ViewInteractivity::None
        } else {
            ViewInteractivity::Clickables(state.kegs.len())
        })
    }

    fn click(
        &self,
        app: &mut App,
        state: &AsyncState,
        index: usize,
    ) -> Result<Option<NavAction>> {
        Ok(if !state.kegs.is_empty() {
            app.current_keg = Some((&state.kegs[index]).try_into()?);
            Some(NavAction::Push(NavID::Named("keg")))
        } else {
            None
        })
    }
}
