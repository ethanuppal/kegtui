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

fn oxford_comma(items: Vec<String>, if_empty: impl Into<String>) -> String {
    match items.len() {
        0 => if_empty.into(),
        1 => items.join(""),
        2 => format!("{} and {}", items[0], items[1]),
        _ => format!(
            "{}, and {}",
            items[..items.len() - 1].join(", "),
            items[items.len() - 1]
        ),
    }
}

impl View for KegsView {
    fn draw_content(
        &self,
        app: &App,
        state: &AsyncState,
        frame: &mut Frame,
        area: Rect,
        is_focused: bool,
    ) -> Result<()> {
        let search_locations = oxford_comma(
            app.config
                .keg_search_paths
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect(),
            "nowhere (you'll need to specify paths in the config file)",
        );
        let text = format!(
            "Select a Keg (kegs are searched under {search_locations}):"
        );
        let wrapped = textwrap::wrap(&text, area.width as usize);

        let title_paragraph =
            Paragraph::new(text.clone()).wrap(Wrap { trim: false });
        frame.render_widget(
            title_paragraph,
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: wrapped.len() as u16,
            },
        );

        let list_area = Rect {
            x: area.x,
            y: area.y + (wrapped.len() as u16),
            width: area.width,
            height: area.height.saturating_sub(wrapped.len() as u16),
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
    ) -> Result<Option<NavAction<'_>>> {
        Ok(if !state.kegs.is_empty() {
            app.current_keg = Some((&state.kegs[index]).try_into()?);
            Some(NavAction::Push(NavID::Named("keg")))
        } else {
            None
        })
    }
}
