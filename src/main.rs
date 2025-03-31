use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::line::VERTICAL,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Padding, Row, Table,
    },
};
use strum::{EnumCount, VariantNames};
use strum_macros::{AsRefStr, EnumCount, FromRepr, VariantNames};

#[derive(Clone, Copy, EnumCount)]
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

struct App {
    exit: bool,
    current_view: View,
    menu_states: [ListState; View::COUNT],
    focus: Focus,
    current_modal: Option<Modal>,
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
        }
    }

    fn menu_items(&self) -> &'static [&'static str] {
        match self.current_view {
            View::Main => MainMenuItem::VARIANTS,
            View::Keg => KegMenuItem::VARIANTS,
        }
    }

    async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();

        let main_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::from(" kegtui ").into_centered_line())
            .title_bottom(
                Line::from(vec![
                    " View key bindings ".into(),
                    "<?>".blue().bold(),
                    " ".into(),
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
        self.draw_content(frame, section_rects[2]);

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
            Row::new(vec!["<?>".blue().bold(), "Toggle this modal ".into()]),
            Row::new(vec!["<Esc>".blue().bold(), "Dismiss this modal ".into()]),
            Row::new(vec![
                Line::from(vec![
                    "<Left>".blue().bold(),
                    ", ".into(),
                    "<H>".blue().bold(),
                ]),
                "Focus menu ".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Right>".blue().bold(),
                    ", ".into(),
                    "<L>".blue().bold(),
                ]),
                "Focus main view ".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Up>".blue().bold(),
                    ", ".into(),
                    "<K>".blue().bold(),
                ]),
                "Navigate up ".into(),
            ]),
            Row::new(vec![
                Line::from(vec![
                    "<Down>".blue().bold(),
                    ", ".into(),
                    "<J>".blue().bold(),
                ]),
                "Navigate down ".into(),
            ]),
            Row::new(vec!["<Enter>".blue().bold(), "Select ".into()]),
            Row::new(vec![
                "<Q>".blue().bold(),
                "Exit app (if modal is not open) ".into(),
            ]),
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
                    .fg(Color::Yellow)
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

    fn draw_content(&mut self, frame: &mut Frame<'_>, area: Rect) {
        match self.current_view {
            View::Main => {
                match MainMenuItem::from_repr(
                    self.menu_state_ref().selected().expect("No item selected"),
                )
                .expect("Invalid item selected")
                {
                    MainMenuItem::Kegs => {}
                    MainMenuItem::Settings => {}
                    MainMenuItem::Credits => {}
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
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let menu_length = self.menu_items().len();
        let current = self.menu_state_ref().selected().unwrap_or(0);

        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.current_modal = None;
            }
            KeyCode::Char('?') => {
                if self.current_modal.is_none() {
                    self.current_modal = Some(Modal::KeybindsHelp);
                } else if self.current_modal == Some(Modal::KeybindsHelp) {
                    self.current_modal = None;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.focus == Focus::Menu {
                    let new_index = if current == 0 {
                        menu_length - 1
                    } else {
                        current - 1
                    };
                    self.menu_state_mut().select(Some(new_index));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.focus == Focus::Menu {
                    let new_index = if current >= menu_length - 1 {
                        0
                    } else {
                        current + 1
                    };
                    self.menu_state_mut().select(Some(new_index));
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.focus = Focus::Menu;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.focus = Focus::Content;
            }
            KeyCode::Enter => {
                if self.focus == Focus::Menu {
                    let items = self.menu_items();
                    if let Some(selected) = self.menu_state_ref().selected() {
                        let item = items[selected];
                        match self.current_view {
                            //View::Main => {
                            //    if item == MainMenuItem::Kegs.as_ref() {
                            //        self.current_view = View::Keg;
                            //        self.menu_state_mut().select(Some(
                            //            KegMenuItem::Main as usize,
                            //        ));
                            //    }
                            //}
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
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal).await;
    ratatui::restore();
    app_result
}
