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
use ratatui::widgets::{List, ListItem, ListState};

use crate::{
    app::{App, AsyncState, SELECTED_FOCUSED_STYLE, SELECTED_UNFOCUSED_STYLE},
    view::prelude::*,
};

mod commands {
    pub const INSTALL_HOMEBREW: &str = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
    pub const UPGRADE_HOMEBREW: &str = "brew upgrade";
    pub const INSTALL_KEGWORKS: &str =
        "brew install --cask --no-quarantine Kegworks-App/kegworks/kegworks";
}

pub struct SetupWizardView;

impl View for SetupWizardView {
    fn draw_content(
        &self,
        app: &App,
        state: &AsyncState,
        frame: &mut Frame,
        area: Rect,
        is_focused: bool,
    ) -> Result<()> {
        // some of the worst code I've written
        //
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(0)])
            .split(area);

        let rows = vec![
            Row::new(vec![
                "brew".white().on_dark_gray(),
                match state.brew_installed {
                    Some(installed) => {
                        if installed {
                            "Installed".bold().green()
                        } else {
                            "Missing".bold().red()
                        }
                    }
                    None => "Loading status...".into(),
                },
            ]),
            Row::new(vec![
                "Kegworks".white().on_dark_gray(),
                match state.kegworks_installed {
                    Some(installed) => {
                        if installed {
                            "Installed".bold().green()
                        } else {
                            "Missing".bold().red()
                        }
                    }
                    None => "Loading status...".into(),
                },
            ]),
        ];
        let table = Table::new(
            rows,
            &[Constraint::Length(10), Constraint::Percentage(70)],
        );

        frame.render_widget(table, chunks[0]);

        let mut help_items = vec![
            ListItem::new(
                "Restart kegtui if everything is installed".green().bold(),
            ),
            ListItem::new(""),
        ];
        let brew_missing = state.brew_installed == Some(false);
        let kegworks_missing = state.kegworks_installed == Some(false);

        if brew_missing {
            help_items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "How to install: ",
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    "brew",
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                ),
            ])));

            help_items.push(ListItem::new(Line::from(vec![Span::styled(
                "[ Copy command ]",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )])));
            help_items.push(ListItem::new(commands::INSTALL_HOMEBREW));
            help_items.push(ListItem::new(" "));
        }

        if kegworks_missing {
            help_items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "How to install: ",
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    "Kegworks",
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                ),
            ])));

            help_items.push(ListItem::new(Line::from(vec![Span::styled(
                "[ Copy commands ]",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )])));
            help_items.push(ListItem::new(commands::UPGRADE_HOMEBREW));
            help_items.push(ListItem::new(commands::INSTALL_KEGWORKS));
        }

        if brew_missing && kegworks_missing {
            help_items.push(ListItem::new(" "));

            help_items.push(ListItem::new(Line::from(vec![Span::styled(
                "Install everything at once",
                Style::default().fg(Color::White),
            )])));

            help_items.push(ListItem::new(Line::from(vec![Span::styled(
                "[ Copy all commands ]",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )])));
        }

        if !help_items.is_empty() {
            let help_list = List::new(help_items.clone())
                .highlight_style(if is_focused {
                    SELECTED_FOCUSED_STYLE
                } else {
                    SELECTED_UNFOCUSED_STYLE
                })
                .highlight_symbol(">> ");

            let mut real_state = ListState::default().with_selected(Some(
                match app.interaction_state() {
                    0 => 3,
                    1 => 7,
                    2 => 12,
                    _ => unreachable!(),
                },
            ));

            frame.render_stateful_widget(help_list, chunks[1], &mut real_state);
        }

        Ok(())
    }

    fn interactivity(
        &self,
        _app: &App,
        state: &AsyncState,
    ) -> Result<ViewInteractivity> {
        Ok(match (state.brew_installed, state.kegworks_installed) {
            (Some(false), Some(false)) => ViewInteractivity::Clickables(3),
            (Some(false), _) | (_, Some(false)) => {
                ViewInteractivity::Clickables(1)
            }
            _ => ViewInteractivity::None,
        })
    }

    fn click(
        &self,
        app: &mut App,
        state: &AsyncState,
        index: usize,
    ) -> Result<Option<NavAction>> {
        let commands_to_copy = [
            commands::INSTALL_HOMEBREW,
            &format!(
                "{}\n{}",
                commands::UPGRADE_HOMEBREW,
                commands::INSTALL_KEGWORKS
            ),
            &format!(
                "{}\n{}\n{}",
                commands::INSTALL_HOMEBREW,
                commands::UPGRADE_HOMEBREW,
                commands::INSTALL_KEGWORKS
            ),
        ];
        copy_to_clipboard(
            match (state.brew_installed, state.kegworks_installed) {
                (Some(false), Some(false)) => &commands_to_copy[index],
                (Some(false), _) => &commands_to_copy[0],
                (_, Some(false)) => &commands_to_copy[1],
                _ => unreachable!(),
            },
        )?;
        Ok(None)
    }
}

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
