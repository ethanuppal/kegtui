// Code from https://github.com/Harzu/iced_term

use std::path::PathBuf;

use iced::{
    advanced::graphics::core::Element, font::Family, widget::container, window,
    Font, Length, Size, Subscription, Task, Theme,
};
use iced_term::{ColorPalette, TerminalView};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .antialiasing(false)
        .window_size(Size {
            width: 1280.0,
            height: 720.0,
        })
        .subscription(App::subscription)
        .font(
            include_bytes!("../fonts/HackNerdFontMono-Regular.ttf").as_slice(),
        )
        .run_with(App::new)
}

#[derive(Debug, Clone)]
pub enum Event {
    Terminal(iced_term::Event),
}

struct App {
    title: String,
    term: iced_term::Terminal,
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

impl App {
    fn new() -> (Self, Task<Event>) {
        let mut executable_path = resources_root().unwrap_or_default();
        executable_path.push(TUI_EXECUTABLE);

        let oxocarbon = ColorPalette {
            foreground: String::from("#e0def4"),
            background: String::from("#191724"),
            black: String::from("#26233a"),
            red: String::from("#eb6f92"),
            green: String::from("#31748f"),
            yellow: String::from("#f6c177"),
            blue: String::from("#9ccfd8"),
            magenta: String::from("#c4a7e7"),
            cyan: String::from("#ebbcba"),
            white: String::from("#e0def4"),
            bright_black: String::from("#6e6a86"),
            bright_red: String::from("#eb6f92"),
            bright_green: String::from("#31748f"),
            bright_yellow: String::from("#f6c177"),
            bright_blue: String::from("#9ccfd8"),
            bright_magenta: String::from("#c4a7e7"),
            bright_cyan: String::from("#ebbcba"),
            bright_white: String::from("#e0def4"),
            bright_foreground: None,
            dim_foreground: String::from("#6e6a86"),
            dim_black: String::from("#191724"),
            dim_red: String::from("#b85879"),
            dim_green: String::from("#285a72"),
            dim_yellow: String::from("#c49a5f"),
            dim_blue: String::from("#7fa3ad"),
            dim_magenta: String::from("#9d85b8"),
            dim_cyan: String::from("#bc9596"),
            dim_white: String::from("#b4b1c7"),
        };

        let term_id = 0;
        let term_settings = iced_term::settings::Settings {
            font: iced_term::settings::FontSettings {
                size: 18.0,
                font_type: Font {
                    family: Family::Name("Hack Nerd Font Mono"),
                    ..Default::default()
                },
                ..Default::default()
            },
            theme: iced_term::settings::ThemeSettings {
                color_pallete: Box::new(oxocarbon),
            },
            backend: iced_term::settings::BackendSettings {
                shell: executable_path.to_string_lossy().to_string(),
                ..Default::default()
            },
        };

        (
            Self {
                title: String::from("kegtui"),
                term: iced_term::Terminal::new(term_id, term_settings),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Subscription<Event> {
        let term_subscription = iced_term::Subscription::new(self.term.id);
        let term_event_stream = term_subscription.event_stream();
        Subscription::run_with_id(self.term.id, term_event_stream)
            .map(Event::Terminal)
    }

    fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::Terminal(iced_term::Event::CommandReceived(_, cmd)) => {
                match self.term.update(cmd) {
                    iced_term::actions::Action::Shutdown => {
                        window::get_latest().and_then(window::close)
                    }
                    iced_term::actions::Action::ChangeTitle(title) => {
                        self.title = title;
                        Task::none()
                    }
                    _ => Task::none(),
                }
            }
        }
    }

    fn view(&self) -> Element<Event, Theme, iced::Renderer> {
        container(TerminalView::show(&self.term).map(Event::Terminal))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
