use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    time,
};

use color_eyre::{Result, eyre::eyre};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::line::VERTICAL,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph,
        Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, Wrap,
    },
};
use strum::{EnumCount, VariantNames};
use strum_macros::{AsRefStr, EnumCount, FromRepr, VariantNames};

#[derive(PartialEq, Eq, Clone, Copy, EnumCount)]
#[repr(usize)]
enum View {
    Main,
    Keg,
}

#[derive(VariantNames, AsRefStr, FromRepr)]
enum MainMenuItem {
    Kegs,
    Settings,
    Credits,
}

#[derive(VariantNames, AsRefStr, FromRepr)]
enum KegMenuItem {
    Back,
    Main,
    Winetricks,
    Config,
}

#[derive(PartialEq, Eq)]
enum Focus {
    Menu,
    Content,
}

#[derive(PartialEq, Eq)]
enum Modal {
    KeybindsHelp,
}

#[derive(Debug, Clone)]
struct Keg {
    config_file: PathBuf,
    wineskin_launcher: OsString,
}

impl Keg {
    fn from_path(path: &Path) -> Self {
        Self {
            config_file: path.join("Contents/Info.plist"),
            wineskin_launcher: path
                .join("Contents/MacOS/wineskinLauncher")
                .into_os_string(),
        }
    }
}

struct App {
    exit: bool,
    current_view: View,
    menu_states: [ListState; View::COUNT],
    focus: Focus,
    current_modal: Option<Modal>,
    credits_vertical_scroll: usize,
    kegs_vertical_scroll: usize,
    current_keg: Option<Keg>,
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            current_view: View::Main,
            menu_states: [
                ListState::default().with_selected(Some(0)),
                ListState::default().with_selected(Some(0)),
            ],
            focus: Focus::Menu,
            current_modal: None,
            credits_vertical_scroll: 0,
            kegs_vertical_scroll: 0,
            current_keg: None,
        }
    }

    fn menu_items(&self) -> &'static [&'static str] {
        match self.current_view {
            View::Main => MainMenuItem::VARIANTS,
            View::Keg => KegMenuItem::VARIANTS,
        }
    }

    async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        list_of_kegs: Arc<RwLock<Vec<Keg>>>,
    ) -> Result<()> {
        let mut interval =
            tokio::time::interval(time::Duration::from_millis(20));

        while !self.exit {
            if let Ok(list_of_kegs) = list_of_kegs.read() {
                terminal
                    .draw(|frame| self.draw(frame, list_of_kegs.as_ref()))?;
            }
            self.handle_events()?;
            interval.tick().await;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame<'_>, list_of_kegs: &[Keg]) {
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

        self.draw_menu(frame, section_rects[0]);
        self.draw_vertical_separator(frame, section_rects[1]);
        self.draw_content(frame, section_rects[2], list_of_kegs);

        if self.current_modal == Some(Modal::KeybindsHelp) {
            let help = Self::make_keybinds_help_table();

            let modal_width = 62;
            let modal_height = 12;
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
            frame.render_widget(help, inner_modal_area);
        }
    }

    fn make_keybinds_help_table() -> Table<'static> {
        let rows = vec![
            Row::new(vec!["<?>".blue().bold(), "Toggle this modal".into()]),
            Row::new(vec![
                "<Esc>".blue().bold(),
                "Exit modal, otherwise move to menu".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Left>".blue().bold(),
                    ", ".into(),
                    "<H>".blue().bold(),
                ]),
                "Focus menu".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Right>".blue().bold(),
                    ", ".into(),
                    "<L>".blue().bold(),
                ]),
                "Focus content".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Up>".blue().bold(),
                    ", ".into(),
                    "<K>".blue().bold(),
                ]),
                "Navigate up".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Down>".blue().bold(),
                    ", ".into(),
                    "<J>".blue().bold(),
                ]),
                "Navigate down".into(),
            ]),
            Row::new(vec![
                "<Enter>".blue().bold(),
                "Select (e.g., menu item)".into(),
            ]),
            Row::new(vec!["<Q>".blue().bold(), "Exit app".into()]),
        ];
        Table::new(
            rows,
            &[Constraint::Percentage(30), Constraint::Percentage(70)],
        )
    }

    fn draw_menu(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let menu_items: Vec<ListItem> = self
            .menu_items()
            .iter()
            .cloned()
            .map(|item| ListItem::new(Span::from(item)))
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
        frame.render_stateful_widget(menu, area, self.menu_state_mut());
    }

    fn draw_vertical_separator(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let buffer = frame.buffer_mut();
        for y in area.top()..area.bottom() {
            buffer[(area.x, y)].set_symbol(VERTICAL).set_fg(Color::Gray);
        }
    }

    fn draw_content(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        list_of_kegs: &[Keg],
    ) {
        match self.current_view {
            View::Main => {
                match MainMenuItem::from_repr(
                    self.menu_state_ref().selected().expect("No item selected"),
                )
                .expect("Invalid item selected")
                {
                    MainMenuItem::Kegs => {
                        let kegs_list = list_of_kegs
                            .iter()
                            .map(|keg| Line::from(format!("{keg:?}")))
                            .collect::<Vec<_>>();
                        let kegs_paragraph = Paragraph::new(kegs_list.clone());

                        let scrollbar =
                            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                                .begin_symbol(Some("↑"))
                                .end_symbol(Some("↓"));

                        let mut scrollbar_state =
                            ScrollbarState::new(kegs_list.len())
                                .position(self.kegs_vertical_scroll);

                        frame.render_widget(
                            kegs_paragraph
                                .wrap(Wrap { trim: false })
                                .scroll((self.kegs_vertical_scroll as u16, 0)),
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
                    }
                    MainMenuItem::Settings => {}
                    MainMenuItem::Credits => {
                        macro_rules! add_credits {
                            (&mut $list:path, $source:literal) => {
                                $list.push("".into());
                                $list.push(concat!($source, ":").into());
                                $list.extend(
                                    textwrap::wrap(
                                        include_str!(concat!(
                                            env!("CARGO_MANIFEST_DIR"),
                                            concat!(
                                                "/resource/credits/",
                                                $source,
                                                ".txt"
                                            )
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
                        add_credits!(&mut credits_list, "ratatui");
                        add_credits!(&mut credits_list, "strum");
                        add_credits!(&mut credits_list, "strum_macros");
                        add_credits!(&mut credits_list, "textwrap");
                        add_credits!(&mut credits_list, "tokio");

                        let credits_paragraph =
                            Paragraph::new(credits_list.clone());

                        let scrollbar =
                            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                                .begin_symbol(Some("↑"))
                                .end_symbol(Some("↓"));

                        let mut scrollbar_state =
                            ScrollbarState::new(credits_list.len())
                                .position(self.credits_vertical_scroll);

                        frame.render_widget(
                            credits_paragraph.scroll((
                                self.credits_vertical_scroll as u16,
                                0,
                            )),
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
                    }
                }
            }
            View::Keg => {
                match KegMenuItem::from_repr(
                    self.menu_state_ref().selected().expect("No item selected"),
                )
                .expect("Invalid item selected")
                {
                    KegMenuItem::Back => {}
                    KegMenuItem::Main => {}
                    KegMenuItem::Winetricks => {}
                    KegMenuItem::Config => {}
                }
            }
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(time::Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let menu_length = self.menu_items().len();
        let current = self.menu_state_ref().selected().unwrap_or(0);

        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                if self.current_modal.is_some() {
                    self.current_modal = None;
                } else if self.focus == Focus::Content {
                    self.focus = Focus::Menu;
                }
            }
            KeyCode::Char('?') => {
                if self.current_modal.is_none() {
                    self.current_modal = Some(Modal::KeybindsHelp);
                } else if self.current_modal == Some(Modal::KeybindsHelp) {
                    self.current_modal = None;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return;
                }
                if self.focus == Focus::Menu {
                    let new_index = if current == 0 {
                        menu_length - 1
                    } else {
                        current - 1
                    };
                    self.menu_state_mut().select(Some(new_index));
                } else if self.focus == Focus::Content
                    && self.current_view == View::Main
                    && MainMenuItem::Credits as usize
                        == self
                            .menu_state_ref()
                            .selected()
                            .expect("Missing selection")
                {
                    self.credits_vertical_scroll =
                        self.credits_vertical_scroll.saturating_sub(3);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return;
                }
                if self.focus == Focus::Menu {
                    let new_index = if current >= menu_length - 1 {
                        0
                    } else {
                        current + 1
                    };
                    self.menu_state_mut().select(Some(new_index));
                } else if self.focus == Focus::Content
                    && self.current_view == View::Main
                    && MainMenuItem::Credits as usize
                        == self
                            .menu_state_ref()
                            .selected()
                            .expect("Missing selection")
                {
                    self.credits_vertical_scroll =
                        self.credits_vertical_scroll.saturating_add(3);
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return;
                }
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return;
                }
                self.focus = Focus::Menu;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return;
                }
                self.focus = Focus::Content;
            }
            KeyCode::Enter => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return;
                }
                if self.focus == Focus::Menu {
                    let items = self.menu_items();
                    if let Some(selected) = self.menu_state_ref().selected() {
                        let item = items[selected];
                        match self.current_view {
                            View::Main => {
                                self.focus = Focus::Content;
                            }
                            View::Keg => {
                                if item == KegMenuItem::Back.as_ref() {
                                    self.current_view = View::Main;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn menu_state_ref(&self) -> &ListState {
        &self.menu_states[self.current_view as usize]
    }

    fn menu_state_mut(&mut self) -> &mut ListState {
        &mut self.menu_states[self.current_view as usize]
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let list_of_kegs = Arc::new(RwLock::new(vec![]));

    let (quit_tx, mut quit_rx) = tokio::sync::oneshot::channel();

    let list_of_kegs_clone = list_of_kegs.clone();
    let worker = tokio::spawn(async move {
        loop {
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
            if let Ok(mut lock) = list_of_kegs_clone.try_write() {
                *lock = kegs;
            }
        }
    });

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal, list_of_kegs).await;
    quit_tx.send(()).or(Err(eyre!("bug: Could not send quit message to worker thread, so you have to use CTRL-C unfortunately")))?;
    ratatui::restore();
    app_result
}
