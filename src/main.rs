// Code from https://github.com/Harzu/iced_term

use std::{collections::HashMap, fs, path::PathBuf, process::Command};

use font_kit::source::SystemSource;
use iced::{
    Font, Length, Size, Subscription, Task, Theme,
    advanced::graphics::core::Element,
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{button, column, container, horizontal_space, row, text},
    window::{self, Settings},
};
use iced_term::{ColorPalette, TerminalView};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .antialiasing(false)
        // .window_size(Size {
        //     width: 1400.0,
        //     height: 720.0,
        // })
        .window(Settings {
            min_size: Some(Size {
                width: 640.0,
                height: 480.0,
            }),

            ..Default::default()
        })
        .subscription(App::subscription)
        .font(
            include_bytes!("../fonts/HackNerdFontMono-Regular.ttf").as_slice(),
        )
        .default_font(Font::with_name("Hack Nerd Font Mono"))
        .run_with(App::new)
}

#[derive(Debug, Clone)]
pub enum Event {
    Terminal(iced_term::Event),
    DebugEditFont,
    DebugEditEnv,
    DebugRefreshConfig,
}

struct App {
    title: String,
    fallback_font: &'static str,
    font_config_file: Option<PathBuf>,
    env_config_file: Option<PathBuf>,
    term: iced_term::Terminal,
    hide_extra_ui: bool,
    exit_on_terminal_shutdown: bool,
}

// https://web.archive.org/web/20250718013155/https://github.com/burtonageo/cargo-bundle/issues/167#issuecomment-3032588931
fn resources_root() -> Option<PathBuf> {
    if std::env::var_os("CARGO").is_some() {
        return Some(PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR")?));
    }

    // TODO: support for other platforms
    #[cfg(target_os = "macos")]
    {
        let bundle = core_foundation::bundle::CFBundle::main_bundle();
        let bundle_path = bundle.path()?;
        let resources_path = bundle.resources_path()?;
        Some(bundle_path.join(resources_path))
    }
    #[cfg(not(any(target_os = "macos")))]
    None
}

const TUI_EXECUTABLE: &str = "target/x86_64-apple-darwin/release/kegtui";

fn font_exists(font_name: &str) -> bool {
    let source = SystemSource::new();
    source.select_family_by_name(font_name).is_ok()
}

impl App {
    fn new() -> (Self, Task<Event>) {
        let mut executable_path = resources_root().unwrap_or_default();
        executable_path.push(TUI_EXECUTABLE);

        let fallback_font = if font_exists("Hack Nerd Font Mono") {
            "Hack Nerd Font Mono"
        } else {
            "Menlo"
        };

        let font_config_file =
            dirs::data_local_dir().and_then(|mut config_directory| {
                config_directory.push("com.ethanuppal.kegtui");
                fs::create_dir_all(&config_directory).ok()?;
                config_directory.push("font.txt");
                Some(config_directory)
            });
        let env_config_file =
            dirs::data_local_dir().and_then(|mut config_directory| {
                config_directory.push("com.ethanuppal.kegtui");
                fs::create_dir_all(&config_directory).ok()?;
                config_directory.push(".env");
                Some(config_directory)
            });

        let oxocarbon = ColorPalette {
            foreground: String::from("#dde1e6"),
            background: String::from("#161616"),
            black: String::from("#262626"),
            red: String::from("#ff7eb6"),
            green: String::from("#42be65"),
            yellow: String::from("#82cfff"),
            blue: String::from("#33b1ff"),
            magenta: String::from("#ee5396"),
            cyan: String::from("#3ddbd9"),
            white: String::from("#dde1e6"),
            bright_black: String::from("#393939"),
            bright_red: String::from("#ff7eb6"),
            bright_green: String::from("#42be65"),
            bright_yellow: String::from("#82cfff"),
            bright_blue: String::from("#33b1ff"),
            bright_magenta: String::from("#ee5396"),
            bright_cyan: String::from("#3ddbd9"),
            bright_white: String::from("#ffffff"),
            bright_foreground: None,
            dim_foreground: String::from("#525252"),
            dim_black: String::from("#161616"),
            dim_red: String::from("#cc6591"),
            dim_green: String::from("#359851"),
            dim_yellow: String::from("#69a7cc"),
            dim_blue: String::from("#2990cc"),
            dim_magenta: String::from("#be4378"),
            dim_cyan: String::from("#31b1ae"),
            dim_white: String::from("#b4b7ba"),
        };

        let term_id = 0;
        let term_settings = iced_term::settings::Settings {
            theme: iced_term::settings::ThemeSettings {
                color_pallete: Box::new(oxocarbon),
            },
            backend: iced_term::settings::BackendSettings {
                program: executable_path.to_string_lossy().to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        (
            Self {
                title: String::from("kegtui"),
                fallback_font,
                font_config_file,
                env_config_file,
                term: iced_term::Terminal::new(term_id, term_settings)
                    .expect("Failed to create terminal"),
                hide_extra_ui: Default::default(),
                exit_on_terminal_shutdown: Default::default(),
            }
            .refresh_config_owned(),
            Task::none(),
        )
    }

    fn refresh_config(&mut self) {
        let terminal_font = self
            .font_config_file
            .as_ref()
            .and_then(|config_file| fs::read_to_string(config_file).ok())
            .unwrap_or_else(|| self.fallback_font.into());
        let leaked: &'static str = Box::leak(Box::new(terminal_font.clone()));

        let env_variables = self
            .env_config_file
            .as_ref()
            .and_then(|env_config_file| {
                env_file_reader::read_file(env_config_file).ok()
            })
            .unwrap_or_default();

        self.term.handle(iced_term::Command::ChangeFont(
            iced_term::settings::FontSettings {
                size: env_variables
                    .get("KEGTUI_FONT_SIZE")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(24.0),
                font_type: Font {
                    family: Family::Name(leaked),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
        self.hide_extra_ui = env_variables
            .get("KEGTUI_HIDE_EXTRA_UI")
            .map(|value| value == "1")
            .unwrap_or(false);

        self.exit_on_terminal_shutdown = !env_variables
            .get("KEGTUI_EXIT_ON_TERMINAL_SHUTDOWN")
            .map(|value| value == "0")
            .unwrap_or(false);
    }

    fn refresh_config_owned(mut self) -> Self {
        self.refresh_config();
        self
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Subscription<Event> {
        Subscription::run_with_id(self.term.id, self.term.subscription())
            .map(Event::Terminal)
    }

    fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::Terminal(iced_term::Event::BackendCall(_, cmd)) => {
                match self.term.handle(iced_term::Command::ProxyToBackend(cmd))
                {
                    iced_term::actions::Action::Shutdown => {
                        if self.exit_on_terminal_shutdown {
                            window::get_latest().and_then(window::close)
                        } else {
                            Task::none()
                        }
                    }
                    iced_term::actions::Action::ChangeTitle(title) => {
                        self.title = title;
                        Task::none()
                    }
                    _ => Task::none(),
                }
            }
            Event::DebugEditFont => {
                if let Some(config_file) = &self.font_config_file {
                    if !config_file.exists() {
                        let _ = fs::write(config_file, self.fallback_font);
                    }
                    Command::new("open").arg(&config_file).spawn().ok();
                }
                Task::none()
            }
            Event::DebugEditEnv => {
                if let Some(config_file) = &self.env_config_file {
                    if !config_file.exists() {
                        let _ = fs::write(config_file, "");
                    }
                    Command::new("open").arg(&config_file).spawn().ok();
                }
                Task::none()
            }
            Event::DebugRefreshConfig => {
                self.refresh_config();
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Event, Theme, iced::Renderer> {
        let terminal_view = TerminalView::show(&self.term).map(Event::Terminal);
        container(
            if self.hide_extra_ui {
                column![terminal_view]
            } else {
                column![
                    container(
                        text("Note: This has been ported directly from a terminal app into a GUI app, so there may be issues. Click on the app to focus if keys don't work. Vim keybinds work. Keybinds won't show if window too small.")
                            .width(Length::Fill)
                            .align_x(Horizontal::Left)
                    )
                    .padding(4),
                    terminal_view,
                    container(
                        row![
                            text("Debug Config"),
                            horizontal_space(),
                            button("Edit font")
                                .on_press(Event::DebugEditFont),
                            button("Edit env")
                                .on_press(Event::DebugEditEnv),
                            button("Refresh")
                                .on_press(Event::DebugRefreshConfig),
                        ]
                            .align_y(Vertical::Center)
                            .spacing(4)
                    )
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .padding(4)
                ]
            }
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
