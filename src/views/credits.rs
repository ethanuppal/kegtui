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

use crate::{
    app::{App, AsyncState},
    view::prelude::*,
};

pub struct CreditsView;

impl View for CreditsView {
    fn draw_content(
        &self,
        app: &App,
        _state: &AsyncState,
        frame: &mut ratatui::Frame<'_>,
        area: ratatui::prelude::Rect,
    ) -> Result<()> {
        macro_rules! add_credits {
            (&mut $list:path, $source:literal) => {
                $list.push("".into());
                $list.push(concat!($source, ":").into());
                $list.extend(
                    textwrap::wrap(
                        include_str!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            concat!("/resource/credits/", $source, ".txt")
                        )),
                        area.width as usize,
                    )
                    .iter()
                    .cloned()
                    .map(Line::from)
                    .map(|line| line.blue().into()),
                );
            };
        }
        let mut credits_list = vec![Line::from(
            "The following open-source projects were used to create this app.",
        )];
        add_credits!(&mut credits_list, "color-eyre");
        add_credits!(&mut credits_list, "crossterm");
        add_credits!(&mut credits_list, "plist");
        add_credits!(&mut credits_list, "ratatui");
        add_credits!(&mut credits_list, "serde");
        add_credits!(&mut credits_list, "strum");
        add_credits!(&mut credits_list, "strum_macros");
        add_credits!(&mut credits_list, "textwrap");
        add_credits!(&mut credits_list, "tokio");

        let credits_paragraph = Paragraph::new(credits_list.clone());

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

        let mut scrollbar_state = ScrollbarState::new(credits_list.len())
            .position(app.interaction_state());

        frame.render_widget(
            credits_paragraph.scroll((app.interaction_state() as u16, 0)),
            area,
        );
        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );

        Ok(())
    }

    fn interactivity(
        &self,
        _app: &App,
        _state: &AsyncState,
    ) -> ViewInteractivity {
        ViewInteractivity::Scrollable
    }
}
