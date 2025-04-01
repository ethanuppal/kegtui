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
    io::{Write},
    process::{Command, Stdio},
};

use crate::{
    app::App,
    view::{MenuItem, MenuItemAction, NavContext},
};
use app::spawn_worker;
use checks::is_kegworks_installed;
use color_eyre::{eyre::eyre, Result};

pub mod app;
pub mod checks;
pub mod keg;
pub mod keg_config;
pub mod keg_plist;
pub mod view;
pub mod views;
//#[derive(PartialEq, Eq, Clone, Copy, EnumCount)]
//#[repr(usize)]
//enum View {
//    SetupWizard,
//    Main,
//    Keg,
//}
//
//#[derive(VariantNames, AsRefStr, FromRepr)]
//enum SetupWizardMenuItem {
//    #[strum(to_string = "Setup Wizard")]
//    SetupWizard,
//}
//
//#[derive(VariantNames, AsRefStr, FromRepr)]
//enum MainMenuItem {
//    Kegs,
//    Settings,
//    Credits,
//}
//
//#[derive(VariantNames, AsRefStr, FromRepr)]
//enum KegMenuItem {
//    Back,
//    Main,
//    Winetricks,
//    #[strum(to_string = "Open C Drive")]
//    CDrive,
//    Config,
//}
//
//#[derive(PartialEq, Eq, Clone, Copy)]
//enum Focus {
//    Menu,
//    Content,
//}
//
//#[derive(PartialEq, Eq)]
//enum Modal {
//    KeybindsHelp,
//}

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

//struct App {
//    exit: bool,
//    current_view: View,
//    menu_states: [ListState; View::COUNT],
//    focus: Focus,
//    current_modal: Option<Modal>,
//    credits_vertical_scroll: usize,
//    keg_selection: ListState,
//    current_keg: Option<CurrentKeg>,
//    setup_wizard_help_list_state: ListState,
//}
//
//impl App {
//    fn new(needs_setup_wizard: bool) -> Self {
//        Self {
//            exit: false,
//            current_view: if needs_setup_wizard {
//                View::SetupWizard
//            } else {
//                View::Main
//            },
//            menu_states: [
//                ListState::default().with_selected(Some(0)),
//                ListState::default().with_selected(Some(0)),
//                ListState::default().with_selected(Some(0)),
//            ],
//            focus: Focus::Menu,
//            current_modal: None,
//            credits_vertical_scroll: 0,
//            keg_selection: ListState::default(),
//            current_keg: None,
//            setup_wizard_help_list_state: ListState::default(),
//        }
//    }
//
//    fn menu_items(&self) -> &'static [&'static str] {
//        match self.current_view {
//            View::SetupWizard => SetupWizardMenuItem::VARIANTS,
//            View::Main => MainMenuItem::VARIANTS,
//            View::Keg => KegMenuItem::VARIANTS,
//        }
//    }
//
//    async fn run(
//        &mut self,
//        terminal: &mut DefaultTerminal,
//        state: Arc<RwLock<AsyncState>>,
//    ) -> Result<()> {
//        let mut interval =
//            tokio::time::interval(time::Duration::from_millis(20));
//
//        while !self.exit {
//            if let Ok(state) = state.read() {
//                terminal.draw(|frame| self.draw(frame, &state))?;
//                self.handle_events(&state, terminal)?;
//            }
//            interval.tick().await;
//        }
//        Ok(())
//    }
//
//    fn draw(&mut self, frame: &mut Frame<'_>, state: &AsyncState) {
//        let area = frame.area();
//
//        let main_block = Block::default()
//            .borders(Borders::ALL)
//            .title(Span::from(" kegtui ").into_centered_line())
//            .title_bottom(
//                Line::from(vec![
//                    " View keybinds ".into(),
//                    "<?>".blue().bold(),
//                    " | Copyright (C) 2025 Ethan Uppal ".into(),
//                ])
//                .centered(),
//            );
//        let inner_area = main_block.inner(area);
//
//        frame.render_widget(main_block, area);
//
//        let section_rects = Layout::default()
//            .direction(Direction::Horizontal)
//            .constraints(
//                [
//                    Constraint::Percentage(25),
//                    Constraint::Length(1),
//                    Constraint::Percentage(74),
//                ]
//                .as_ref(),
//            )
//            .split(inner_area);
//
//        self.draw_menu(frame, section_rects[0]);
//        self.draw_vertical_separator(frame, section_rects[1]);
//        self.draw_content(frame, section_rects[2], state);
//
//        if self.current_modal == Some(Modal::KeybindsHelp) {
//            let help = Self::make_keybinds_help_table();
//
//            let modal_width = 62;
//            let modal_height = 12;
//            let modal_area = Rect {
//                x: area.x + (area.width.saturating_sub(modal_width)) / 2,
//                y: area.y + (area.height.saturating_sub(modal_height)) / 2,
//                width: modal_width,
//                height: modal_height,
//            };
//
//            frame.render_widget(Clear, modal_area);
//
//            let modal_block = Block::default()
//                .title(Span::from(" Keybinds ").into_centered_line())
//                .borders(Borders::ALL)
//                .padding(Padding::uniform(1));
//            let inner_modal_area = modal_block.inner(modal_area);
//
//            frame.render_widget(modal_block, modal_area);
//            frame.render_widget(help, inner_modal_area);
//        }
//    }
//
//    fn make_keybinds_help_table() -> Table<'static> {
//        let rows = vec![
//            Row::new(vec!["<?>".blue().bold(), "Toggle this modal".into()]),
//            Row::new(vec![
//                "<Esc>".blue().bold(),
//                "Exit modal, otherwise move to menu".into(),
//            ]),
//            Row::new(vec![
//                Line::from(vec![
//                    "<Left>".blue().bold(),
//                    ", ".into(),
//                    "<H>".blue().bold(),
//                ]),
//                "Focus menu".into(),
//            ]),
//            Row::new(vec![
//                Line::from(vec![
//                    "<Right>".blue().bold(),
//                    ", ".into(),
//                    "<L>".blue().bold(),
//                ]),
//                "Focus content".into(),
//            ]),
//            Row::new(vec![
//                Line::from(vec![
//                    "<Up>".blue().bold(),
//                    ", ".into(),
//                    "<K>".blue().bold(),
//                ]),
//                "Navigate up".into(),
//            ]),
//            Row::new(vec![
//                Line::from(vec![
//                    "<Down>".blue().bold(),
//                    ", ".into(),
//                    "<J>".blue().bold(),
//                ]),
//                "Navigate down".into(),
//            ]),
//            Row::new(vec![
//                "<Enter>".blue().bold(),
//                "Select (e.g., menu item)".into(),
//            ]),
//            Row::new(vec!["<Q>".blue().bold(), "Exit app".into()]),
//        ];
//        Table::new(
//            rows,
//            &[Constraint::Percentage(30), Constraint::Percentage(70)],
//        )
//    }
//
//    fn draw_menu(&mut self, frame: &mut Frame<'_>, area: Rect) {
//        let menu_items: Vec<ListItem> = self
//            .menu_items()
//            .iter()
//            .cloned()
//            .map(|item| ListItem::new(Span::from(item)))
//            .collect();
//        let menu = List::new(menu_items)
//            .highlight_style(
//                Style::default()
//                    .fg(if self.focus == Focus::Menu {
//                        Color::Yellow
//                    } else {
//                        Color::Gray
//                    })
//                    .add_modifier(Modifier::BOLD),
//            )
//            .highlight_symbol(">> ");
//        frame.render_stateful_widget(menu, area, self.menu_state_mut());
//    }
//
//    fn draw_vertical_separator(&mut self, frame: &mut Frame<'_>, area: Rect) {
//        let buffer = frame.buffer_mut();
//        for y in area.top()..area.bottom() {
//            buffer[(area.x, y)].set_symbol(VERTICAL).set_fg(Color::Gray);
//        }
//    }
//
//    fn draw_content(
//        &mut self,
//        frame: &mut Frame<'_>,
//        area: Rect,
//        state: &AsyncState,
//    ) {
//        match self.current_view {
//            View::SetupWizard => {
//                match SetupWizardMenuItem::from_repr(
//                    self.menu_state_ref().selected().expect("No item
// selected"),                )
//                .expect("Invalid item selected")
//                {
//                    SetupWizardMenuItem::SetupWizard => {
//                        self.draw_setup_wizard(frame, area, state)
//                    }
//                }
//            }
//            View::Main => {
//                match MainMenuItem::from_repr(
//                    self.menu_state_ref().selected().expect("No item
// selected"),                )
//                .expect("Invalid item selected")
//                {
//                    MainMenuItem::Kegs => {
//                        self.draw_kegs_list(frame, area, state)
//                    }
//                    MainMenuItem::Settings => {}
//                    MainMenuItem::Credits => self.draw_credits(frame, area),
//                }
//            }
//            View::Keg => {
//                match KegMenuItem::from_repr(
//                    self.menu_state_ref().selected().expect("No item
// selected"),                )
//                .expect("Invalid item selected")
//                {
//                    KegMenuItem::Back => {}
//                    KegMenuItem::Main => {}
//                    KegMenuItem::Winetricks => {}
//                    KegMenuItem::CDrive => {}
//                    KegMenuItem::Config => {}
//                }
//            }
//        }
//    }
//
//    const INSTALL_HOMEBREW: &str = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
//    const UPDATE_HOMEBREW: &str = "brew upgrade";
//    const INSTALL_KEGWORKS: &str =
//        "brew install --cask --no-quarantine Kegworks-App/kegworks/kegworks";
//
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
//
//    fn draw_kegs_list(
//        &mut self,
//        frame: &mut Frame<'_>,
//        area: Rect,
//        state: &AsyncState,
//    ) {
//        let keg_items: Vec<ListItem> = state
//            .kegs
//            .iter()
//            .cloned()
//            .map(|keg| ListItem::new(Span::from(keg.name)))
//            .collect();
//        if !keg_items.is_empty() && self.keg_selection.selected().is_none() {
//            self.keg_selection.select(Some(0));
//        }
//
//        let keg_title = Paragraph::new("Select a Keg:");
//        frame.render_widget(keg_title, Rect { height: 1, ..area });
//
//        let list_area = Rect {
//            x: area.x,
//            y: area.y + 1,
//            width: area.width,
//            height: area.height.saturating_sub(1),
//        };
//
//        let keg_selector_list = List::new(keg_items)
//            .highlight_style(
//                Style::default()
//                    .fg(if self.focus == Focus::Content {
//                        Color::Yellow
//                    } else {
//                        Color::Gray
//                    })
//                    .add_modifier(Modifier::BOLD),
//            )
//            .highlight_symbol(">> ");
//        frame.render_stateful_widget(
//            keg_selector_list,
//            list_area,
//            &mut self.keg_selection,
//        );
//    }
//
//    fn draw_credits(&mut self, frame: &mut Frame<'_>, area: Rect) {}
//
//    fn handle_events(
//        &mut self,
//        state: &AsyncState,
//        terminal: &mut DefaultTerminal,
//    ) -> Result<()> {
//        if event::poll(time::Duration::from_millis(5))? {
//            match event::read()? {
//                Event::Key(key_event)
//                    if key_event.kind == KeyEventKind::Press =>
//                {
//                    self.handle_key_event(key_event, state, terminal)?
//                }
//                _ => {}
//            };
//        }
//        Ok(())
//    }
//
//    fn handle_key_event(
//        &mut self,
//        key_event: KeyEvent,
//        state: &AsyncState,
//        terminal: &mut DefaultTerminal,
//    ) -> Result<()> {
//        match key_event.code {
//            KeyCode::Char('q') => self.exit(),
//            KeyCode::Esc => {
//                if self.current_modal.is_some() {
//                    self.current_modal = None;
//                } else if self.focus == Focus::Content {
//                    self.focus = Focus::Menu;
//                }
//            }
//            KeyCode::Char('?') => {
//                if self.current_modal.is_none() {
//                    self.current_modal = Some(Modal::KeybindsHelp);
//                } else if self.current_modal == Some(Modal::KeybindsHelp) {
//                    self.current_modal = None;
//                }
//            }
//            KeyCode::Up | KeyCode::Char('k') => {
//                if self.current_modal == Some(Modal::KeybindsHelp) {
//                    return Ok(());
//                }
//                if let (Some(menu_length), Some(state)) = (
//                    self.current_list_length(state),
//                    self.current_list_state_mut(),
//                ) {
//                    let current = *state.selected().get_or_insert(0);
//                    let new_index = if current == 0 {
//                        menu_length - 1
//                    } else {
//                        current - 1
//                    };
//                    state.select(Some(new_index));
//                } else if self.focus == Focus::Content
//                    && self.current_view == View::Main
//                    && MainMenuItem::Credits as usize
//                        == self
//                            .menu_state_ref()
//                            .selected()
//                            .expect("Missing selection")
//                {
//                    self.credits_vertical_scroll =
//                        self.credits_vertical_scroll.saturating_sub(3);
//                }
//            }
//            KeyCode::Down | KeyCode::Char('j') => {
//                if self.current_modal == Some(Modal::KeybindsHelp) {
//                    return Ok(());
//                }
//                if let (Some(menu_length), Some(state)) = (
//                    self.current_list_length(state),
//                    self.current_list_state_mut(),
//                ) {
//                    let current = *state.selected().get_or_insert(0);
//                    let new_index = if current >= menu_length - 1 {
//                        0
//                    } else {
//                        current + 1
//                    };
//                    state.select(Some(new_index));
//                } else if self.focus == Focus::Content
//                    && self.current_view == View::Main
//                    && MainMenuItem::Credits as usize
//                        == self
//                            .menu_state_ref()
//                            .selected()
//                            .expect("Missing selection")
//                {
//                    self.credits_vertical_scroll =
//                        self.credits_vertical_scroll.saturating_add(3);
//                }
//            }
//            KeyCode::Left | KeyCode::Char('h') => {
//                if self.current_modal == Some(Modal::KeybindsHelp) {
//                    return Ok(());
//                }
//                if self.current_modal == Some(Modal::KeybindsHelp) {
//                    return Ok(());
//                }
//                self.focus = Focus::Menu;
//            }
//            KeyCode::Right | KeyCode::Char('l') => {
//                if self.current_modal == Some(Modal::KeybindsHelp) {
//                    return Ok(());
//                }
//                self.focus = Focus::Content;
//            }
//            KeyCode::Enter => {
//                if self.current_modal == Some(Modal::KeybindsHelp) {
//                    return Ok(());
//                }
//                if self.focus == Focus::Menu {
//                    let items = self.menu_items();
//                    if let Some(selected) = self.menu_state_ref().selected() {
//                        let item = items[selected];
//                        match self.current_view {
//                            View::SetupWizard | View::Main => {
//                                self.focus = Focus::Content;
//                            }
//                            View::Keg => {
//                                if item == KegMenuItem::Back.as_ref() {
//                                    self.current_view = View::Main;
//                                } else if item == KegMenuItem::Config.as_ref()
// {                                    if let Some(current_keg) =
//                                        self.current_keg.as_mut()
//                                    {
//                                        let toml_config =
//                                            toml::to_string_pretty(
//                                                &current_keg
//                                                    .plist
//                                                    .extract_config(),
//                                            )?;
//                                        let new_toml_config =
// Self::run_editor(                                            terminal,
//                                            "/tmp/kegtui.toml",
//                                            toml_config,
//                                        )?;
//                                        let new_config =
//                                            toml::from_str(&new_toml_config)?;
//                                        current_keg
//                                            .plist
//                                            .update_from_config(&new_config);
//                                        //fs::write(current_keg.config_file,
// plist)                                        plist::to_file_xml(
//                                            &current_keg.config_file,
//                                            &current_keg.plist,
//                                        )?;
//                                    }
//                                } else if item == KegMenuItem::CDrive.as_ref()
// {                                    if let Ok(explorer) =
// env::var("EXPLORER") {
// Self::run_program(                                            terminal,
//                                            Command::new(explorer).arg(
//                                                self.current_keg
//                                                    .as_ref()
//                                                    .unwrap()
//                                                    .c_drive
//                                                    .to_string_lossy()
//                                                    .to_string(),
//                                            ),
//                                        )?;
//                                    } else {
//                                        Self::run_program(
//                                            terminal,
//                                            Command::new("open").arg(
//                                                self.current_keg
//                                                    .as_ref()
//                                                    .unwrap()
//                                                    .c_drive
//                                                    .to_string_lossy()
//                                                    .to_string(),
//                                            ),
//                                        )?;
//                                    }
//                                } else {
//                                    self.focus = Focus::Content;
//                                }
//                            }
//                        }
//                    }
//                } else if self.focus == Focus::Content
//                    && (self.current_view, self.menu_state_ref().selected())
//                        == (View::Main, Some(MainMenuItem::Kegs as usize))
//                {
//                    self.current_view = View::Keg;
//                    self.focus = Focus::Menu;
//                    self.menu_state_mut()
//                        .select(Some(KegMenuItem::Main as usize));
//                    let keg = &state.kegs[self
//                        .keg_selection
//                        .selected()
//                        .expect("Should have been set to something")];
//
//                    self.current_keg = Some(CurrentKeg {
//                        name: keg.name.clone(),
//                        wineskin_launcher: keg.wineskin_launcher.clone(),
//                        c_drive: keg.c_drive.clone(),
//                        plist: plist::from_file(&keg.config_file)?,
//                        config_file: keg.config_file.clone(),
//                    })
//                } else if self.focus == Focus::Content
//                    && self.current_view == View::SetupWizard
//                {
//                    match self
//                        .setup_wizard_help_list_state
//                        .selected()
//                        .expect("Nothing was selected")
//                    {
//                        0 => copy_to_clipboard(Self::INSTALL_HOMEBREW)?,
//                        1 => copy_to_clipboard(&format!(
//                            "{}\n{}",
//                            Self::UPDATE_HOMEBREW,
//                            Self::INSTALL_KEGWORKS
//                        ))?,
//                        2 => copy_to_clipboard(&format!(
//                            "{}\n{}\n{}",
//                            Self::INSTALL_HOMEBREW,
//                            Self::UPDATE_HOMEBREW,
//                            Self::INSTALL_KEGWORKS
//                        ))?,
//                        _ => unreachable!(),
//                    }
//                }
//            }
//            _ => {}
//        }
//
//        Ok(())
//    }
//
//    fn menu_state_ref(&self) -> &ListState {
//        &self.menu_states[self.current_view as usize]
//    }
//
//    fn menu_state_mut(&mut self) -> &mut ListState {
//        &mut self.menu_states[self.current_view as usize]
//    }
//
//    fn current_list_state_mut(&mut self) -> Option<&mut ListState> {
//        match (self.current_view, self.focus) {
//            (_, Focus::Menu) => Some(self.menu_state_mut()),
//            (View::Main, Focus::Content)
//                if self.menu_state_ref().selected()
//                    == Some(MainMenuItem::Kegs as usize) =>
//            {
//                Some(&mut self.keg_selection)
//            }
//            (View::SetupWizard, Focus::Content) => {
//                Some(&mut self.setup_wizard_help_list_state)
//            }
//            _ => None,
//        }
//    }
//
//    fn current_list_length(&self, state: &AsyncState) -> Option<usize> {
//        match (self.current_view, self.focus) {
//            (_, Focus::Menu) => Some(self.menu_items().len()),
//            (View::Main, Focus::Content)
//                if self.menu_state_ref().selected()
//                    == Some(MainMenuItem::Kegs as usize) =>
//            {
//                Some(state.kegs.len())
//            }
//            (View::SetupWizard, Focus::Content) => {
//                Some(match (state.brew_installed, state.kegworks_installed) {
//                    (Some(false), Some(false)) => 3,
//                    (Some(false), _) | (_, Some(false)) => 1,
//                    _ => 0,
//                })
//            }
//            _ => None,
//        }
//    }
//
//    fn exit(&mut self) {
//        self.exit = true;
//    }
//
//    fn run_editor(
//        terminal: &mut DefaultTerminal,
//        file: &str,
//        initial: impl Into<String>,
//    ) -> Result<String> {
//        fs::write(file, initial.into())?;
//        io::stdout().execute(LeaveAlternateScreen)?;
//        disable_raw_mode()?;
//        let editor = env::var("EDITOR").unwrap_or("vim".into());
//        Command::new(editor).arg(file).status()?;
//        io::stdout().execute(EnterAlternateScreen)?;
//        enable_raw_mode()?;
//        terminal.clear()?;
//        Ok(fs::read_to_string(file)?)
//    }
//
//    fn run_program(
//        terminal: &mut DefaultTerminal,
//        command: &mut Command,
//    ) -> Result<()> {
//        io::stdout().execute(LeaveAlternateScreen)?;
//        disable_raw_mode()?;
//        command.status()?;
//        io::stdout().execute(EnterAlternateScreen)?;
//        enable_raw_mode()?;
//        terminal.clear()?;
//        Ok(())
//    }
//}

fn main() -> Result<()> {
    let mut context = NavContext::default();

    let kegs_view = context.view("kegs", &views::kegs::KegsView);
    let credits_view = context.view("credits", &views::credits::CreditsView);
    let setup_wizard_view =
        context.view("wizard", &views::setup_wizard::SetupWizardView);

    let setup_wizard_nav = context.nav(
        "wizard",
        [MenuItem::new(
            "Setup Wizard",
            MenuItemAction::LoadView(setup_wizard_view),
        )],
    );
    let main_nav = context.nav(
        "main",
        [
            MenuItem::new("Kegs", MenuItemAction::LoadView(kegs_view)),
            MenuItem::new("Credits", MenuItemAction::LoadView(credits_view)),
        ],
    );

    let (async_state, _terminate_worker_guard) = spawn_worker();

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(
        &mut context,
        if is_kegworks_installed() {
            main_nav
        } else {
            setup_wizard_nav
        },
        &mut terminal,
        async_state,
    );
    ratatui::restore();
    app_result
}
