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
    fs,
    sync::{self, Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    widgets::{Block, Borders},
    DefaultTerminal,
};
use symbols::line::VERTICAL;

use crate::{checks, keg::Keg, view::prelude::*};

#[derive(Default)]
pub struct App {
    exit: bool,
    pub(crate) vertical_scroll: usize,
}

impl App {
    pub fn run<'a>(
        &mut self,
        context: &mut NavContext<'a, App>,
        initial: NavID<'a>,
        terminal: &mut DefaultTerminal,
        state: Arc<RwLock<AsyncState>>,
    ) -> Result<()> {
        context.push_nav(initial);

        let mut interval = Instant::now();
        let duration = Duration::from_millis(100);

        while !self.exit {
            interval += duration;
            let now = Instant::now();

            if now < interval {
                thread::sleep(interval - now);
            }

            if let Ok(state) = state.read() {
                terminal.draw(|frame| self.draw(context, frame, &state))?;
                self.handle_events(&state, terminal)?;
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        context: &mut NavContext<'_, App>,
        frame: &mut Frame,
        state: &AsyncState,
    ) {
        let area = frame.area();

        let main_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::from(" kegtui ").into_centered_line())
            .title_bottom(
                Line::from(vec![
                    " View keybinds ".into(),
                    "<?>".blue().bold(),
                    " | Copyright (C) 2025 Ethan Uppal ".into(),
                ])
                .centered(),
            );
        let inner_area = main_block.inner(area);

        frame.render_widget(main_block, area);

        let section_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Length(1),
                    Constraint::Percentage(74),
                ]
                .as_ref(),
            )
            .split(inner_area);

        if let Some(current_nav) = context.top_nav() {
            self.draw_menu(frame, section_rects[0]);
            self.draw_vertical_separator(frame, section_rects[1]);
            //self.draw_content(frame, section_rects[2], state);
        }
    }
    fn draw_menu(&mut self, frame: &mut Frame<'_>, area: Rect) {
        //let menu_items: Vec<ListItem> = self
        //    .menu_items()
        //    .iter()
        //    .cloned()
        //    .map(|item| ListItem::new(Span::from(item)))
        //    .collect();
        //let menu = List::new(menu_items)
        //    .highlight_style(
        //        Style::default()
        //            .fg(if self.focus == Focus::Menu {
        //                Color::Yellow
        //            } else {
        //                Color::Gray
        //            })
        //            .add_modifier(Modifier::BOLD),
        //    )
        //    .highlight_symbol(">> ");
        //frame.render_stateful_widget(menu, area, self.menu_state_mut());
    }

    fn draw_vertical_separator(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let buffer = frame.buffer_mut();
        for y in area.top()..area.bottom() {
            buffer[(area.x, y)].set_symbol(VERTICAL);
        }
    }

    fn handle_events(
        &mut self,
        state: &AsyncState,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        if event::poll(Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event, state, terminal)?
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        state: &AsyncState,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

#[derive(Default)]
pub struct AsyncState {
    kegs: Vec<Keg>,
    brew_installed: Option<bool>,
    kegworks_installed: Option<bool>,
}

pub struct TerminateWorkerGuard(sync::mpsc::Sender<()>);

impl Drop for TerminateWorkerGuard {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

pub fn spawn_worker() -> (Arc<RwLock<AsyncState>>, TerminateWorkerGuard) {
    let async_state = Arc::new(RwLock::new(AsyncState::default()));

    let (quit_tx, quit_rx) = sync::mpsc::channel();

    {
        let async_state = async_state.clone();
        thread::spawn(move || loop {
            if quit_rx.try_recv().is_ok() {
                break;
            }
            let mut kegs = vec![];
            for enclosing_location in [
                "/Applications",
                "~/Applications/",
                "~/Applications/Kegworks/",
            ] {
                if let Ok(read_dir) = fs::read_dir(enclosing_location) {
                    for entry in read_dir.flatten() {
                        if entry
                            .path()
                            .join("Contents/KegworksConfig.app")
                            .exists()
                        {
                            kegs.push(Keg::from_path(&entry.path()));
                        }
                    }
                }
            }

            let brew_installed = checks::is_brew_installed();
            let kegworks_installed = checks::is_kegworks_installed();

            if let Ok(mut lock) = async_state.try_write() {
                lock.kegs = kegs;
                lock.brew_installed = Some(brew_installed);
                lock.kegworks_installed = Some(kegworks_installed);
            }
        });
    }

    (async_state, TerminateWorkerGuard(quit_tx))
}
