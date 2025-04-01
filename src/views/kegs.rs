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

use ratatui::widgets::{List, ListItem, ListState};

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
        let keg_title = Paragraph::new("Select a Keg:");
        frame.render_widget(keg_title, Rect { height: 1, ..area });

        if !state.kegs.is_empty() {
            let keg_items: Vec<ListItem> = state
                .kegs
                .iter()
                .cloned()
                .map(|keg| ListItem::new(Span::from(keg.name)))
                .collect();

            let list_area = Rect {
                x: area.x,
                y: area.y + 1,
                width: area.width,
                height: area.height.saturating_sub(1),
            };

            let keg_selector_list = List::new(keg_items)
                .highlight_style(if is_focused {
                    SELECTED_FOCUSED_STYLE
                } else {
                    SELECTED_UNFOCUSED_STYLE
                })
                .highlight_symbol(">> ");
            frame.render_stateful_widget(
                keg_selector_list,
                list_area,
                &mut ListState::default()
                    .with_selected(Some(app.interaction_state())),
            );
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
