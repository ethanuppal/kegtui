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
    env, fs, io,
    path::PathBuf,
    sync::{self, Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    DefaultTerminal,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding},
};
use symbols::line::VERTICAL;

use crate::{
    app_config::AppConfig,
    checks,
    keg::{CurrentKeg, Engine, Keg, Wrapper},
    view::prelude::*,
};

pub const SELECTED_FOCUSED_STYLE: Style =
    Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);
pub const SELECTED_UNFOCUSED_STYLE: Style =
    Style::new().fg(Color::White).add_modifier(Modifier::BOLD);

fn make_keybinds_help_table() -> (Table<'static>, u16, u16) {
    macro_rules! make {
        ($((
            [$($lhs:literal),*],
            $rhs:literal
        )),*) => {{
            const SEPARATOR: &str = ", ";
            let mut lhs_width = 0;
            let mut rhs_width = 0;
            $(lhs_width = ::std::cmp::max(lhs_width, $($lhs.len() + SEPARATOR.len() + )* 0 - SEPARATOR.len());)*
            $(rhs_width = ::std::cmp::max(rhs_width, $rhs.len());)*
            let rows = vec![
                $(
                    Row::new(vec![
                        Line::from({
                            let mut keys = vec![];
                            for (i, key) in [$($lhs),*].into_iter().enumerate() {
                                if i > 0 {
                                    keys.push(SEPARATOR.into());
                                }
                                keys.push(key.blue().bold());
                            }
                            keys
                        }),
                        $rhs.into()
                    ])
                ),*
            ];
            let height = rows.len();
            let table = Table::new(
                rows,
                &[Constraint::Length(lhs_width as u16), Constraint::Length(rhs_width as u16)],
            );
            (table, (lhs_width + 1 + rhs_width) as u16, height as u16)
        }};
    }
    make![
        (["<?>"], "Toggle this modal"),
        (["<Esc>"], "Exit modal (in modal), focus menu (in content)"),
        (["<Left>", "<H>"], "Focus menu"),
        (["<Right>", "<L>"], "Focus content"),
        (["<Up>", "<K>"], "Navigate up"),
        (["<Down>", "<J>"], "Navigate down"),
        (
            ["<Enter>"],
            "Focus content (in menu), select button (in content)"
        ),
        (["<Z>"], "Suspend app"),
        (["<Q>"], "Exit app")
    ]
}

pub fn inspect_terminal(_app: &mut App, _state: &AsyncState) -> Result<()> {
    eprintln!("┌──────────────────────────────────┐");
    eprintln!("│ Press enter to return to the TUI │");
    eprintln!("└──────────────────────────────────┘");
    io::stdin().read_line(&mut String::new())?;
    Ok(())
}

#[derive(Default, PartialEq, Eq)]
enum Focus {
    #[default]
    Menu,
    Content,
}

pub struct App<'a> {
    // TODO: separate this state out of App into something like a NavController
    exit: bool,
    focus: Focus,
    menu_state: usize,
    current_view: Option<ViewID<'a>>,
    clickables_state: usize,
    // ENDTODO
    pub current_keg: Option<CurrentKeg>,
    pub config: &'a AppConfig,
    show_keybinds_modal: bool,
}

impl<'a> App<'a> {
    pub fn new(config: &'a AppConfig) -> Self {
        Self {
            exit: Default::default(),
            focus: Default::default(),
            menu_state: Default::default(),
            current_view: Default::default(),
            clickables_state: Default::default(),
            current_keg: Default::default(),
            config,
            show_keybinds_modal: Default::default(),
        }
    }

    pub fn interaction_state(&self) -> usize {
        self.clickables_state
    }

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
                terminal
                    .draw(|frame| self.draw(context, frame, &state).unwrap())?;
                self.handle_events(context, &state, terminal)?;
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        context: &mut NavContext<'a>,
        frame: &mut Frame,
        state: &AsyncState,
    ) -> Result<()> {
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
            self.draw_content(context, state, frame, section_rects[2])?;
        } else {
            Clear.render(section_rects[2], frame.buffer_mut());
        }

        if self.show_keybinds_modal {
            let (modal_table, table_width, table_height) =
                make_keybinds_help_table();
            let modal_width = table_width + 4;
            let modal_height = table_height + 4;

            if !(modal_width > area.width - 3 || modal_height > area.height - 3)
            {
                let modal_area = Rect {
                    x: area.x + (area.width.saturating_sub(modal_width)) / 2,
                    y: area.y + (area.height.saturating_sub(modal_height)) / 2,
                    width: modal_width,
                    height: modal_height,
                };

                frame.render_widget(Clear, modal_area);

                let modal_block = Block::default()
                    .title(Span::from(" Keybinds ").into_centered_line())
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(1));
                let inner_modal_area = modal_block.inner(modal_area);

                frame.render_widget(modal_block, modal_area);
                frame.render_widget(modal_table, inner_modal_area);
            }
        }

        Ok(())
    }
    fn draw_menu(&mut self, frame: &mut Frame, area: Rect, menu: &[MenuItem]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        frame.render_widget("Menu:".bold(), chunks[0]);

        let menu_items: Vec<ListItem> = menu
            .iter()
            .map(|item| ListItem::new(Span::from(item.name())))
            .collect();
        let menu = List::new(menu_items)
            .highlight_style(if self.focus == Focus::Menu {
                SELECTED_FOCUSED_STYLE
            } else {
                SELECTED_UNFOCUSED_STYLE
            })
            .highlight_symbol(">> ");
        frame.render_stateful_widget(
            menu,
            chunks[1],
            &mut ListState::default().with_selected(Some(self.menu_state)),
        );
    }

    fn draw_vertical_separator(&mut self, frame: &mut Frame, area: Rect) {
        let buffer = frame.buffer_mut();
        for y in area.top()..area.bottom() {
            buffer[(area.x, y)].set_symbol(VERTICAL);
        }
    }

    fn draw_content(
        &mut self,
        context: &NavContext<'a>,
        state: &AsyncState,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        if let Some(view_id) = self.current_view {
            let view = context.get_view(view_id);
            view.draw_content(
                self,
                state,
                frame,
                area,
                self.focus == Focus::Content,
            )?;
        }
        Ok(())
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
        if self.show_keybinds_modal {
            if matches!(key_event.code, KeyCode::Esc | KeyCode::Char('?')) {
                self.show_keybinds_modal = false;
            }
            return Ok(());
        }
        let current_nav = context.top_nav().unwrap();
        let menu = context.get_nav(current_nav).menu();
        let current_menu_item = &menu[self.menu_state];

        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.focus = Focus::Menu;
            }
            KeyCode::Char('?') => {
                self.show_keybinds_modal = true;
            }
            KeyCode::Up | KeyCode::Char('k') => match self.focus {
                Focus::Menu => {
                    self.menu_state = self.menu_state.saturating_sub(1);
                }
                Focus::Content => {
                    let current_view = context
                        .get_view(self.current_view.expect(
                            "Focused view but app has no current view",
                        ));
                    match current_view.interactivity(self, state)? {
                        ViewInteractivity::None => {}
                        ViewInteractivity::Scrollable => {
                            self.clickables_state =
                                self.clickables_state.saturating_sub(3);
                        }
                        ViewInteractivity::Clickables(_) => {
                            self.clickables_state =
                                self.clickables_state.saturating_sub(1);
                        }
                    }
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match self.focus {
                Focus::Menu => {
                    if self.menu_state + 1 < menu.len() {
                        self.menu_state += 1;
                    }
                }
                Focus::Content => {
                    let current_view = context
                        .get_view(self.current_view.expect(
                            "Focused view but app has no current view",
                        ));
                    match current_view.interactivity(self, state)? {
                        ViewInteractivity::None => {}
                        ViewInteractivity::Scrollable => {
                            self.clickables_state += 3;
                        }
                        ViewInteractivity::Clickables(count) => {
                            if self.clickables_state + 1 < count {
                                self.clickables_state += 1;
                            }
                        }
                    }
                }
            },
            KeyCode::Left | KeyCode::Char('h') => {
                self.focus = Focus::Menu;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.focus == Focus::Menu {
                    let menu_action = current_menu_item.action().clone();
                    self.execute_menu_action(
                        context,
                        state,
                        terminal,
                        menu_action,
                    )?;
                }
            }

            KeyCode::Enter => match self.focus {
                Focus::Menu => {
                    let menu_action = current_menu_item.action().clone();
                    self.execute_menu_action(
                        context,
                        state,
                        terminal,
                        menu_action,
                    )?;
                }
                Focus::Content => {
                    if let Some(nav_action) =
                        context
                            .get_view(self.current_view.expect(
                                "Focused view but app has no current view",
                            ))
                            .click(self, state, self.clickables_state)?
                    {
                        self.execute_nav_action(context, nav_action);
                    }
                }
            },
            KeyCode::Char('z') => self.execute_menu_action(
                context,
                state,
                terminal,
                MenuItemAction::External(inspect_terminal),
            )?,
            _ => {}
        }
        Ok(())
    }

    fn execute_menu_action(
        &mut self,
        context: &mut NavContext<'a>,
        state: &AsyncState,
        terminal: &mut DefaultTerminal,
        menu_action: MenuItemAction<'a>,
    ) -> Result<()> {
        match menu_action {
            MenuItemAction::NavAction(nav_action) => {
                self.execute_nav_action(context, nav_action)
            }
            MenuItemAction::LoadView(view_id) => {
                self.load_view(view_id);
            }
            MenuItemAction::External(external) => {
                io::stdout().execute(LeaveAlternateScreen)?;
                disable_raw_mode()?;
                external(self, state)?;
                io::stdout().execute(EnterAlternateScreen)?;
                enable_raw_mode()?;
                terminal.clear()?;
            }
        }
        Ok(())
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
            }
        }
        self.focus = Focus::Menu;
        self.current_view = None;
        self.menu_state = context
            .get_nav(context.top_nav().unwrap())
            .default_item_index();
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
    pub kegs: Vec<Keg>,
    pub engines: Vec<Engine>,
    pub wrappers: Vec<Wrapper>,
}

pub struct TerminateWorkerGuard(sync::mpsc::Sender<()>);

impl Drop for TerminateWorkerGuard {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

fn read_search_paths(
    search_paths: &[PathBuf],
    home_directory: &str,
) -> impl Iterator<Item = fs::DirEntry> {
    search_paths
        .iter()
        .map(move |enclosing_location| {
            enclosing_location
                .to_string_lossy()
                .replace("~", &home_directory)
        })
        .filter_map(|fixed_enclosing_location| {
            fs::read_dir(fixed_enclosing_location).ok()
        })
        .flat_map(|read_dir| read_dir.flatten())
}

pub fn spawn_worker(
    config: Arc<AppConfig>,
) -> (Arc<RwLock<AsyncState>>, TerminateWorkerGuard) {
    let async_state = Arc::new(RwLock::new(AsyncState::default()));

    let (quit_tx, quit_rx) = sync::mpsc::channel();

    {
        let async_state = async_state.clone();
        thread::spawn(move || {
            loop {
                if quit_rx.try_recv().is_ok() {
                    break;
                }

                let mut kegs = vec![];
                let mut engines = vec![];
                let mut wrappers = vec![];

                let home_directory = env::var("HOME")
                    .expect("User missing home directory env variable");

                for entry in
                    read_search_paths(&config.keg_search_paths, &home_directory)
                {
                    if entry.path().join("Contents/KegworksConfig.app").exists()
                    {
                        kegs.push(Keg::from_path(&entry.path()));
                    }
                }
                for entry in read_search_paths(
                    &config.engine_search_paths,
                    &home_directory,
                ) {
                    if entry
                        .path()
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.ends_with(".tar.7z"))
                        .unwrap_or(false)
                    {
                        engines.push(Engine { path: entry.path() });
                    }
                }
                for entry in read_search_paths(
                    &config.wrapper_search_paths,
                    &home_directory,
                ) {
                    if entry
                        .path()
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.ends_with(".app"))
                        .unwrap_or(false)
                    {
                        wrappers.push(Wrapper { path: entry.path() });
                    }
                }

                if let Ok(mut lock) = async_state.try_write() {
                    lock.kegs = kegs;
                    lock.engines = engines;
                    lock.wrappers = wrappers;
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    (async_state, TerminateWorkerGuard(quit_tx))
}
