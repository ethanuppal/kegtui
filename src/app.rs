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
    widgets::{Block, Borders, List, ListItem, ListState},
    DefaultTerminal,
};
use symbols::line::VERTICAL;

use crate::{checks, keg::Keg, view::prelude::*};

#[derive(Default, PartialEq, Eq)]
enum Focus {
    #[default]
    Menu,
    Content,
}

#[derive(Default)]
pub struct App<'a> {
    // TODO: separate this state out of App into something like a NavController
    exit: bool,
    focus: Focus,
    menu_state: usize,
    current_view: Option<ViewID<'a>>,
    clickables_state: usize,
    // ENDTODO
    pub(crate) vertical_scroll: usize,
}

impl<'a> App<'a> {
    pub fn run(
        &mut self,
        context: &mut NavContext<'a>,
        initial: NavID<'a>,
        terminal: &mut DefaultTerminal,
        state: Arc<RwLock<AsyncState>>,
    ) -> Result<()> {
        context.push_nav(initial);

        let mut interval = Instant::now();
        let duration = Duration::from_millis(20);

        while !self.exit {
            interval += duration;
            let now = Instant::now();

            if now < interval {
                thread::sleep(interval - now);
            }

            if let Ok(state) = state.read() {
                terminal.draw(|frame| self.draw(context, frame, &state))?;
                self.handle_events(context, &state, terminal)?;
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        context: &mut NavContext,
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
            let menu = context.get_nav(current_nav).menu();
            self.draw_menu(frame, section_rects[0], menu);
            self.draw_vertical_separator(frame, section_rects[1]);
            //self.draw_content(frame, section_rects[2], state);
        }
    }
    fn draw_menu(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        menu: &[MenuItem],
    ) {
        let menu_items: Vec<ListItem> = menu
            .iter()
            .map(|item| ListItem::new(Span::from(item.name())))
            .collect();
        let menu = List::new(menu_items)
            .highlight_style(
                Style::default()
                    .fg(if self.focus == Focus::Menu {
                        Color::Yellow
                    } else {
                        Color::Gray
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(
            menu,
            area,
            &mut ListState::default().with_selected(Some(self.menu_state)),
        );
    }

    fn draw_vertical_separator(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let buffer = frame.buffer_mut();
        for y in area.top()..area.bottom() {
            buffer[(area.x, y)].set_symbol(VERTICAL);
        }
    }

    fn handle_events(
        &mut self,
        context: &mut NavContext<'a>,
        state: &AsyncState,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        if event::poll(Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    self.handle_key_event(context, key_event, state, terminal)?
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        context: &mut NavContext<'a>,
        key_event: KeyEvent,
        state: &AsyncState,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        let current_nav = context.top_nav().unwrap();
        let menu = context.get_nav(current_nav).menu();
        let current_menu_item = &menu[self.menu_state];

        let select_length = match self.focus {
            Focus::Menu => menu.len(),
            Focus::Content => context
                .get_view(
                    self.current_view
                        .expect("View focused but app has no view"),
                )
                .clickables(self)
                .len(),
        };

        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.focus = Focus::Menu;
            }
            KeyCode::Char('?') => {}
            KeyCode::Up | KeyCode::Char('k') => match self.focus {
                Focus::Menu => {
                    self.menu_state = self.menu_state.saturating_sub(1);
                }
                Focus::Content => {
                    self.clickables_state =
                        self.clickables_state.saturating_sub(1);
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match self.focus {
                Focus::Menu => {
                    if self.menu_state + 1 < select_length {
                        self.menu_state += 1;
                    }
                }
                Focus::Content => {
                    if self.clickables_state + 1 < select_length {
                        self.clickables_state += 1;
                    }
                }
            },
            KeyCode::Left | KeyCode::Char('h') => {
                self.focus = Focus::Menu;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.focus == Focus::Menu {
                    let menu_action = current_menu_item.action().clone();
                    self.execute_menu_action(context, menu_action);
                }
            }

            KeyCode::Enter => match self.focus {
                Focus::Menu => {
                    let menu_action = current_menu_item.action().clone();
                    self.execute_menu_action(context, menu_action);
                }
                Focus::Content => {
                    if let Some(nav_action) =
                        context
                            .get_view(self.current_view.expect(
                                "Focused view but app has no current view",
                            ))
                            .click(self, self.clickables_state)
                    {
                        self.execute_nav_action(context, nav_action);
                    }
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn execute_menu_action(
        &mut self,
        context: &mut NavContext<'a>,
        menu_action: MenuItemAction<'a>,
    ) {
        match menu_action {
            MenuItemAction::NavAction(nav_action) => {
                self.execute_nav_action(context, nav_action)
            }
            MenuItemAction::LoadView(view_id) => {
                self.load_view(view_id);
            }
        }
    }

    fn execute_nav_action(
        &mut self,
        context: &mut NavContext<'a>,
        nav_action: NavAction<'a>,
    ) {
        match nav_action {
            NavAction::Pop => context.pop_nav(),
            NavAction::Push(nav_id) => {
                context.push_nav(nav_id);
                self.menu_state = context
                    .get_nav(context.top_nav().unwrap())
                    .default_item_index();
            }
        }
    }

    fn load_view(&mut self, view_id: ViewID<'a>) {
        self.current_view = Some(view_id);
        self.focus = Focus::Content;
        self.clickables_state = 0;
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
