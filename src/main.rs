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
    ffi::OsString,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, RwLock},
    time,
};

use color_eyre::{Result, eyre::eyre};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use kegworks_plist::KegworksPlist;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::line::VERTICAL,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph,
        Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
    },
};
use strum::{EnumCount, VariantNames};
use strum_macros::{AsRefStr, EnumCount, FromRepr, VariantNames};

mod kegworks_plist;

#[derive(PartialEq, Eq, Clone, Copy, EnumCount)]
#[repr(usize)]
enum View {
    SetupWizard,
    Main,
    Keg,
}

#[derive(VariantNames, AsRefStr, FromRepr)]
enum SetupWizardMenuItem {
    #[strum(to_string = "Setup Wizard")]
    SetupWizard,
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

#[derive(PartialEq, Eq, Clone, Copy)]
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
    name: String,
    config_file: PathBuf,
    wineskin_launcher: OsString,
}

struct CurrentKeg {
    name: String,
    wineskin_launcher: OsString,
    config: KegworksPlist,
}

impl Keg {
    fn from_path(path: &Path) -> Self {
        Self {
            name: path
                .file_name()
                .expect("Missing Keg name")
                .to_string_lossy()
                .to_string(),
            config_file: path.join("Contents/Info.plist"),
            wineskin_launcher: path
                .join("Contents/MacOS/wineskinLauncher")
                .into_os_string(),
        }
    }
}

#[derive(Default)]
struct AsyncState {
    kegs: Vec<Keg>,
    brew_installed: Option<bool>,
    kegworks_installed: Option<bool>,
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

struct App {
    exit: bool,
    current_view: View,
    menu_states: [ListState; View::COUNT],
    focus: Focus,
    current_modal: Option<Modal>,
    credits_vertical_scroll: usize,
    keg_selection: ListState,
    current_keg: Option<CurrentKeg>,
    setup_wizard_help_list_state: ListState,
}

impl App {
    fn new(needs_setup_wizard: bool) -> Self {
        Self {
            exit: false,
            current_view: if needs_setup_wizard {
                View::SetupWizard
            } else {
                View::Main
            },
            menu_states: [
                ListState::default().with_selected(Some(0)),
                ListState::default().with_selected(Some(0)),
                ListState::default().with_selected(Some(0)),
            ],
            focus: Focus::Menu,
            current_modal: None,
            credits_vertical_scroll: 0,
            keg_selection: ListState::default(),
            current_keg: None,
            setup_wizard_help_list_state: ListState::default(),
        }
    }

    fn menu_items(&self) -> &'static [&'static str] {
        match self.current_view {
            View::SetupWizard => SetupWizardMenuItem::VARIANTS,
            View::Main => MainMenuItem::VARIANTS,
            View::Keg => KegMenuItem::VARIANTS,
        }
    }

    async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        state: Arc<RwLock<AsyncState>>,
    ) -> Result<()> {
        let mut interval =
            tokio::time::interval(time::Duration::from_millis(20));

        while !self.exit {
            if let Ok(state) = state.read() {
                terminal.draw(|frame| self.draw(frame, &state))?;
                self.handle_events(&state)?;
            }
            interval.tick().await;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame<'_>, state: &AsyncState) {
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
        self.draw_content(frame, section_rects[2], state);

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
        state: &AsyncState,
    ) {
        match self.current_view {
            View::SetupWizard => {
                match SetupWizardMenuItem::from_repr(
                    self.menu_state_ref().selected().expect("No item selected"),
                )
                .expect("Invalid item selected")
                {
                    SetupWizardMenuItem::SetupWizard => {
                        self.draw_setup_wizard(frame, area, state)
                    }
                }
            }
            View::Main => {
                match MainMenuItem::from_repr(
                    self.menu_state_ref().selected().expect("No item selected"),
                )
                .expect("Invalid item selected")
                {
                    MainMenuItem::Kegs => {
                        self.draw_kegs_list(frame, area, state)
                    }
                    MainMenuItem::Settings => {}
                    MainMenuItem::Credits => self.draw_credits(frame, area),
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

    const INSTALL_HOMEBREW: &str = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
    const UPDATE_HOMEBREW: &str = "brew upgrade";
    const INSTALL_KEGWORKS: &str =
        "brew install --cask --no-quarantine Kegworks-App/kegworks/kegworks";

    fn draw_setup_wizard(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        state: &AsyncState,
    ) {
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

        let mut help_items = Vec::new();
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
            help_items.push(ListItem::new(Self::INSTALL_HOMEBREW));
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
            help_items.push(ListItem::new(Self::UPDATE_HOMEBREW));
            help_items.push(ListItem::new(Self::INSTALL_KEGWORKS));
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
                .highlight_style(Style::default().fg(
                    if self.focus == Focus::Content {
                        Color::Yellow
                    } else {
                        Color::Gray
                    },
                ))
                .highlight_symbol(">> ");

            if self.setup_wizard_help_list_state.selected().is_none()
                && !help_items.is_empty()
            {
                self.setup_wizard_help_list_state.select(Some(0));
            }

            let mut real_state = ListState::default().with_selected(
                self.setup_wizard_help_list_state.selected().map(|index| {
                    match index {
                        0 => 1,
                        1 => 5,
                        2 => 10,
                        _ => unreachable!(),
                    }
                }),
            );

            frame.render_stateful_widget(help_list, chunks[1], &mut real_state);
        }
    }

    fn draw_kegs_list(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        state: &AsyncState,
    ) {
        let keg_items: Vec<ListItem> = state
            .kegs
            .iter()
            .cloned()
            .map(|keg| ListItem::new(Span::from(keg.name)))
            .collect();
        if !keg_items.is_empty() && self.keg_selection.selected().is_none() {
            self.keg_selection.select(Some(0));
        }

        let keg_title = Paragraph::new("Select a Keg:");
        frame.render_widget(keg_title, Rect { height: 1, ..area });

        let list_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: area.height.saturating_sub(1),
        };

        let keg_selector_list = List::new(keg_items)
            .highlight_style(
                Style::default()
                    .fg(if self.focus == Focus::Content {
                        Color::Yellow
                    } else {
                        Color::Gray
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(
            keg_selector_list,
            list_area,
            &mut self.keg_selection,
        );
    }

    fn draw_credits(&mut self, frame: &mut Frame<'_>, area: Rect) {
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
            .position(self.credits_vertical_scroll);

        frame.render_widget(
            credits_paragraph.scroll((self.credits_vertical_scroll as u16, 0)),
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

    fn handle_events(&mut self, state: &AsyncState) -> Result<()> {
        if event::poll(time::Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event, state)?
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
    ) -> Result<()> {
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
                    return Ok(());
                }
                if let (Some(menu_length), Some(state)) = (
                    self.current_list_length(state),
                    self.current_list_state_mut(),
                ) {
                    let current = *state.selected().get_or_insert(0);
                    let new_index = if current == 0 {
                        menu_length - 1
                    } else {
                        current - 1
                    };
                    state.select(Some(new_index));
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
                    return Ok(());
                }
                if let (Some(menu_length), Some(state)) = (
                    self.current_list_length(state),
                    self.current_list_state_mut(),
                ) {
                    let current = *state.selected().get_or_insert(0);
                    let new_index = if current >= menu_length - 1 {
                        0
                    } else {
                        current + 1
                    };
                    state.select(Some(new_index));
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
                    return Ok(());
                }
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return Ok(());
                }
                self.focus = Focus::Menu;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return Ok(());
                }
                self.focus = Focus::Content;
            }
            KeyCode::Enter => {
                if self.current_modal == Some(Modal::KeybindsHelp) {
                    return Ok(());
                }
                if self.focus == Focus::Menu {
                    let items = self.menu_items();
                    if let Some(selected) = self.menu_state_ref().selected() {
                        let item = items[selected];
                        match self.current_view {
                            View::SetupWizard | View::Main => {
                                self.focus = Focus::Content;
                            }
                            View::Keg => {
                                if item == KegMenuItem::Back.as_ref() {
                                    self.current_view = View::Main;
                                } else {
                                    self.focus = Focus::Content;
                                }
                            }
                        }
                    }
                } else if self.focus == Focus::Content
                    && (self.current_view, self.menu_state_ref().selected())
                        == (View::Main, Some(MainMenuItem::Kegs as usize))
                {
                    self.current_view = View::Keg;
                    self.focus = Focus::Menu;
                    self.menu_state_mut()
                        .select(Some(KegMenuItem::Main as usize));
                    let keg = &state.kegs[self
                        .keg_selection
                        .selected()
                        .expect("Should have been set to something")];

                    self.current_keg = Some(CurrentKeg {
                        name: keg.name.clone(),
                        wineskin_launcher: keg.wineskin_launcher.clone(),
                        config: plist::from_file(&keg.config_file)?,
                    })
                } else if self.focus == Focus::Content
                    && self.current_view == View::SetupWizard
                {
                    match self
                        .setup_wizard_help_list_state
                        .selected()
                        .expect("Nothing was selected")
                    {
                        0 => copy_to_clipboard(Self::INSTALL_HOMEBREW)?,
                        1 => copy_to_clipboard(&format!(
                            "{}\n{}",
                            Self::UPDATE_HOMEBREW,
                            Self::INSTALL_KEGWORKS
                        ))?,
                        2 => copy_to_clipboard(&format!(
                            "{}\n{}\n{}",
                            Self::INSTALL_HOMEBREW,
                            Self::UPDATE_HOMEBREW,
                            Self::INSTALL_KEGWORKS
                        ))?,
                        _ => unreachable!(),
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn menu_state_ref(&self) -> &ListState {
        &self.menu_states[self.current_view as usize]
    }

    fn menu_state_mut(&mut self) -> &mut ListState {
        &mut self.menu_states[self.current_view as usize]
    }

    //fn current_list_state_ref(&self) -> Option<&ListState> {
    //    match (self.current_view, self.focus) {
    //        (_, Focus::Menu) => Some(self.menu_state_ref()),
    //        (View::Main, Focus::Content)
    //            if self.menu_state_ref().selected()
    //                == Some(MainMenuItem::Kegs as usize) =>
    //        {
    //            Some(&self.keg_selection)
    //        }
    //        _ => None,
    //    }
    //}

    fn current_list_state_mut(&mut self) -> Option<&mut ListState> {
        match (self.current_view, self.focus) {
            (_, Focus::Menu) => Some(self.menu_state_mut()),
            (View::Main, Focus::Content)
                if self.menu_state_ref().selected()
                    == Some(MainMenuItem::Kegs as usize) =>
            {
                Some(&mut self.keg_selection)
            }
            (View::SetupWizard, Focus::Content) => {
                Some(&mut self.setup_wizard_help_list_state)
            }
            _ => None,
        }
    }

    fn current_list_length(&self, state: &AsyncState) -> Option<usize> {
        match (self.current_view, self.focus) {
            (_, Focus::Menu) => Some(self.menu_items().len()),
            (View::Main, Focus::Content)
                if self.menu_state_ref().selected()
                    == Some(MainMenuItem::Kegs as usize) =>
            {
                Some(state.kegs.len())
            }
            (View::SetupWizard, Focus::Content) => {
                Some(match (state.brew_installed, state.kegworks_installed) {
                    (Some(false), Some(false)) => 3,
                    (Some(false), _) | (_, Some(false)) => 1,
                    _ => 0,
                })
            }
            _ => None,
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_state = Arc::new(RwLock::new(AsyncState::default()));

    let (quit_tx, mut quit_rx) = tokio::sync::oneshot::channel();

    let async_state = app_state.clone();
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

            let brew_installed = Command::new("which")
                .arg("brew")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);
            let kegworks_installed =
                Path::new("/Applications/Kegworks Winery.app").exists();

            if let Ok(mut lock) = async_state.try_write() {
                lock.kegs = kegs;
                lock.brew_installed = Some(brew_installed);
                lock.kegworks_installed = Some(kegworks_installed);
            }
        }
    });

    eprintln!("Checking to see if Kegworks is installed...");
    let kegworks_installed =
        Path::new("/Applications/Kegworks Winery.app").exists();
    //Command::new("brew")
    //    .args(["list", "--cask", "kegworks"])
    //    .output()
    //    .map(|output| output.status.success())
    //    .unwrap_or(false);

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new(!kegworks_installed)
        .run(&mut terminal, app_state)
        .await;
    quit_tx.send(()).or(Err(eyre!("bug: Could not send quit message to worker thread, so you have to use CTRL-C unfortunately")))?;
    worker.abort();
    ratatui::restore();
    app_result
}
