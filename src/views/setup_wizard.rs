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

use std::{
    io::Write,
    process::{Command, Stdio},
};

use color_eyre::eyre::eyre;

use crate::view::prelude::*;

pub struct SetupWizardView;

impl View for SetupWizardView {}
fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut child = Command::new("pbcopy").stdin(Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;

    if status.success() {
        Ok(())
    } else {
        Err(eyre!("pbcopy failed: {status:?}"))
    }
}
//    fn draw_setup_wizard(
//        &mut self,
//        frame: &mut Frame<'_>,
//        area: Rect,
//        state: &AsyncState,
//    ) {
//        // some of the worst code I've written
//        //
//        let chunks = Layout::default()
//            .direction(Direction::Vertical)
//            .constraints([Constraint::Length(4), Constraint::Min(0)])
//            .split(area);
//
//        let rows = vec![
//            Row::new(vec![
//                "brew".white().on_dark_gray(),
//                match state.brew_installed {
//                    Some(installed) => {
//                        if installed {
//                            "Installed".bold().green()
//                        } else {
//                            "Missing".bold().red()
//                        }
//                    }
//                    None => "Loading status...".into(),
//                },
//            ]),
//            Row::new(vec![
//                "Kegworks".white().on_dark_gray(),
//                match state.kegworks_installed {
//                    Some(installed) => {
//                        if installed {
//                            "Installed".bold().green()
//                        } else {
//                            "Missing".bold().red()
//                        }
//                    }
//                    None => "Loading status...".into(),
//                },
//            ]),
//        ];
//        let table = Table::new(
//            rows,
//            &[Constraint::Length(10), Constraint::Percentage(70)],
//        );
//
//        frame.render_widget(table, chunks[0]);
//
//        let mut help_items = Vec::new();
//        let brew_missing = state.brew_installed == Some(false);
//        let kegworks_missing = state.kegworks_installed == Some(false);
//
//        if brew_missing {
//            help_items.push(ListItem::new(Line::from(vec![
//                Span::styled(
//                    "How to install: ",
//                    Style::default().fg(Color::White),
//                ),
//                Span::styled(
//                    "brew",
//                    Style::default().fg(Color::White).bg(Color::DarkGray),
//                ),
//            ])));
//            help_items.push(ListItem::new(Line::from(vec![Span::styled(
//                "[ Copy command ]",
//                Style::default()
//                    .fg(Color::White)
//                    .add_modifier(Modifier::BOLD),
//            )])));
//            help_items.push(ListItem::new(Self::INSTALL_HOMEBREW));
//            help_items.push(ListItem::new(" "));
//        }
//
//        if kegworks_missing {
//            help_items.push(ListItem::new(Line::from(vec![
//                Span::styled(
//                    "How to install: ",
//                    Style::default().fg(Color::White),
//                ),
//                Span::styled(
//                    "Kegworks",
//                    Style::default().fg(Color::White).bg(Color::DarkGray),
//                ),
//            ])));
//            help_items.push(ListItem::new(Line::from(vec![Span::styled(
//                "[ Copy commands ]",
//                Style::default()
//                    .fg(Color::White)
//                    .add_modifier(Modifier::BOLD),
//            )])));
//            help_items.push(ListItem::new(Self::UPDATE_HOMEBREW));
//            help_items.push(ListItem::new(Self::INSTALL_KEGWORKS));
//        }
//
//        if brew_missing && kegworks_missing {
//            help_items.push(ListItem::new(" "));
//            help_items.push(ListItem::new(Line::from(vec![Span::styled(
//                "Install everything at once",
//                Style::default().fg(Color::White),
//            )])));
//            help_items.push(ListItem::new(Line::from(vec![Span::styled(
//                "[ Copy all commands ]",
//                Style::default()
//                    .fg(Color::White)
//                    .add_modifier(Modifier::BOLD),
//            )])));
//        }
//
//        if !help_items.is_empty() {
//            let help_list = List::new(help_items.clone())
//                .highlight_style(Style::default().fg(
//                    if self.focus == Focus::Content {
//                        Color::Yellow
//                    } else {
//                        Color::Gray
//                    },
//                ))
//                .highlight_symbol(">> ");
//
//            if self.setup_wizard_help_list_state.selected().is_none()
//                && !help_items.is_empty()
//            {
//                self.setup_wizard_help_list_state.select(Some(0));
//            }
//
//            let mut real_state = ListState::default().with_selected(
//                self.setup_wizard_help_list_state.selected().map(|index| {
//                    match index {
//                        0 => 1,
//                        1 => 5,
//                        2 => 10,
//                        _ => unreachable!(),
//                    }
//                }),
//            );
//
//            frame.render_stateful_widget(help_list, chunks[1], &mut
// real_state);        }
//    }
