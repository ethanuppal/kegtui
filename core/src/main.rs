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
    borrow::Cow,
    collections::HashMap,
    ffi::OsStr,
    fmt::Write,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    thread,
};

use crate::{
    app::App,
    app_config::{AppConfig, app_config_file_path},
    view::{MenuItem, MenuItemAction, NavContext},
};
use app::{AsyncState, spawn_worker};
use color_eyre::Result;
use view::NavAction;

pub mod app;
pub mod app_config;
pub mod checks;
pub mod keg;
pub mod keg_config;
pub mod keg_plist;
pub mod view;
pub mod views;

fn wait_for_enter() -> Result<()> {
    io::stdin().read_line(&mut String::new())?;
    Ok(())
}

fn prompt(prompt: &str, validate: impl Fn(&str) -> bool) -> Result<String> {
    use std::io::Write;
    let mut buffer = String::new();
    loop {
        print!("{prompt}");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        if validate(&buffer) {
            break;
        }
    }
    Ok(buffer)
}

fn read_multiline_input(
    app: &App,
    initial: &str,
    editor_file: impl AsRef<OsStr>,
) -> Result<String> {
    let editor_file = editor_file.as_ref();
    fs::write(editor_file, initial)?;
    Command::new(&app.config.editor).arg(editor_file).status()?;
    let contents = fs::read_to_string(editor_file)?;
    Ok(contents)
}

fn parse_winetricks(output: &str) -> Vec<(Cow<'_, str>, &str)> {
    let mut list = vec![];
    for line in output.lines() {
        if !line.is_empty()
            && let Some((lhs, rhs)) = line.split_once(' ')
        {
            let lhs = lhs.trim();
            let rhs = rhs.trim();
            list.push((
                if lhs.chars().all(|c| c == '_' || c.is_ascii_alphanumeric()) {
                    lhs.into()
                } else {
                    format!("\"{lhs}\"").into()
                },
                rhs,
            ));
        }
    }
    list
}

const KEGWORKS_WINETRICKS_SH: &str = "/tmp/kegworks_winetricks.sh";
const KEGWORKS_WINETRICKS_CACHE_TOML: &str =
    "/tmp/kegworks_winetricks_cache.toml";
const KEGWORKS_WINETRICKS_EDITOR_TOML: &str = "/tmp/kegtui_winetricks.toml";

pub fn winetricks(app: &mut App, _state: &AsyncState) -> Result<()> {
    let Some(current_keg) = &app.current_keg else {
        return Ok(());
    };

    if !Path::new(KEGWORKS_WINETRICKS_SH).is_file() {
        eprintln!("┌────────────────────────────┐");
        eprintln!("│ Fetching latest winetricks │");
        eprintln!("└────────────────────────────┘");
        Command::new("curl").args([
        "https://raw.githubusercontent.com/ethanuppal/winetricks/refs/heads/master/src/winetricks",
        "-o",
            KEGWORKS_WINETRICKS_SH
    ]).status()?;
    }

    let initial = if let Ok(winetricks_toml_cached) =
        fs::read_to_string(KEGWORKS_WINETRICKS_CACHE_TOML)
    {
        winetricks_toml_cached
    } else {
        eprintln!("┌─────────────────────────────┐");
        eprintln!("│ Loading winetricks apps     │");
        let apps_list = String::from_utf8(
            Command::new("/bin/sh")
                .args([KEGWORKS_WINETRICKS_SH, "apps", "list"])
                .output()?
                .stdout,
        )?;
        let apps = parse_winetricks(&apps_list);

        eprintln!("│                    dlls     │");
        let dlls_list = String::from_utf8(
            Command::new("/bin/sh")
                .args([KEGWORKS_WINETRICKS_SH, "dlls", "list"])
                .output()?
                .stdout,
        )?;
        let dlls = parse_winetricks(&dlls_list);

        eprintln!("│                    fonts    │");
        let fonts_list = String::from_utf8(
            Command::new("/bin/sh")
                .args([KEGWORKS_WINETRICKS_SH, "fonts", "list"])
                .output()?
                .stdout,
        )?;
        let fonts = parse_winetricks(&fonts_list);

        eprintln!("│                    settings │");
        eprintln!("└─────────────────────────────┘");
        let settings_list = String::from_utf8(
            Command::new("/bin/sh")
                .args([KEGWORKS_WINETRICKS_SH, "settings", "list"])
                .output()?
                .stdout,
        )?;
        let settings = parse_winetricks(&settings_list);

        let mut winetricks_toml = String::from(
            "# Uncomment each winetrick to install\n# Save and quit your editor to select\n\n",
        );
        for (app, description) in apps {
            winetricks_toml
                .push_str(&format!("# app.{app} = \"{description}\"\n"));
        }
        for (dll, description) in dlls {
            winetricks_toml
                .push_str(&format!("# dll.{dll} = \"{description}\"\n"));
        }
        for (font, description) in fonts {
            winetricks_toml
                .push_str(&format!("# font.{font} = \"{description}\"\n"));
        }
        for (setting, description) in settings {
            winetricks_toml.push_str(&format!(
                "# setting.{setting} = \"{description}\"\n"
            ));
        }
        fs::write(KEGWORKS_WINETRICKS_CACHE_TOML, &winetricks_toml)?;
        winetricks_toml
    };
    let result =
        read_multiline_input(app, &initial, KEGWORKS_WINETRICKS_EDITOR_TOML)?;
    let selected_winetricks: HashMap<String, HashMap<String, String>> =
        toml::from_str(&result)?;
    let selected_winetricks =
        selected_winetricks.iter().fold(vec![], |mut list, map| {
            list.extend(map.1.keys());
            list
        });
    if !selected_winetricks.is_empty() {
        let mut console = Command::new("open")
            .arg(current_keg.log_directory.join("Winetricks.log"))
            .spawn()?;
        Command::new(&current_keg.wineskin_launcher)
            .arg("WSS-winetricks")
            .args(selected_winetricks)
            .status()?;
        console.kill()?;
    }

    Ok(())
}

pub fn clear_winetricks_cache(
    _app: &mut App,
    _state: &AsyncState,
) -> Result<()> {
    eprintln!("┌──────────────────────────────────┐");
    eprintln!("│ Press enter to return to the TUI │");
    eprintln!("└──────────────────────────────────┘");
    for file in [
        KEGWORKS_WINETRICKS_SH,
        KEGWORKS_WINETRICKS_CACHE_TOML,
        KEGWORKS_WINETRICKS_EDITOR_TOML,
    ] {
        if PathBuf::from(file).try_exists()? {
            fs::remove_file(file)?;
            eprintln!("rm {file}");
        }
    }
    wait_for_enter()?;

    Ok(())
}

pub fn open_c_drive(app: &mut App, _state: &AsyncState) -> Result<()> {
    let Some(current_keg) = &app.current_keg else {
        return Ok(());
    };
    Command::new(&app.config.explorer)
        .arg(current_keg.c_drive.to_string_lossy().to_string())
        .status()?;
    Ok(())
}

pub fn edit_config(app: &mut App, _state: &AsyncState) -> Result<()> {
    if let Some(current_keg) = &mut app.current_keg {
        let toml_config =
            toml::to_string_pretty(&current_keg.plist.extract_config())?;
        let file = "/tmp/kegtui.toml";
        fs::write(file, toml_config)?;
        Command::new(&app.config.editor).arg(file).status()?;
        let new_toml_config = toml::from_str(&fs::read_to_string(file)?)?;
        current_keg.plist.update_from_config(&new_toml_config);
        plist::to_file_xml(&current_keg.config_file, &current_keg.plist)?;
    }
    Ok(())
}

pub fn launch_keg(app: &mut App, _state: &AsyncState) -> Result<()> {
    if let Some(current_keg) = &app.current_keg {
        eprintln!("┌──────────────────────────────────┐");
        eprintln!("│ Launching this keg               │");
        eprintln!("│ Press enter to return to the TUI │");
        eprintln!("└──────────────────────────────────┘");
        let wrapper = current_keg.wineskin_launcher.clone();
        thread::spawn(move || {
            let _ = Command::new(wrapper).status();
        });
        wait_for_enter()?;
    }
    Ok(())
}

pub fn kill_wineserver(app: &mut App, _state: &AsyncState) -> Result<()> {
    if let Some(current_keg) = &app.current_keg {
        eprintln!("┌─────────────────────────────────────────┐");
        eprintln!("│ Killing processes spawned from this keg │");
        eprintln!("└─────────────────────────────────────────┘");
        Command::new(&current_keg.wineskin_launcher)
            .arg("WSS-wineserverkill")
            .status()?;
    }
    Ok(())
}

pub fn create_keg(app: &mut App, state: &AsyncState) -> Result<()> {
    eprintln!("┌─────────────┐");
    eprintln!("│ Keg creator │");
    eprintln!("└─────────────┘");

    let mut creator_txt = String::from(
        "# Uncomment the engine and wrapper to use\n# Save and quit your editor to select\n# Select nothing to quit\n\n",
    );
    for engine in &state.engines {
        writeln!(&mut creator_txt, "# {}", engine.path.display())?;
    }
    writeln!(&mut creator_txt, "")?;
    for wrapper in &state.wrappers {
        writeln!(&mut creator_txt, "# {}", wrapper.path.display())?;
    }

    enum Action {
        EngineAndWrapper { engine: String, wrapper: String },
        Quit,
    }

    let action;
    loop {
        let choices =
            read_multiline_input(app, &creator_txt, "/tmp/kegcreator.txt")?;

        let engine_and_wrapper = choices
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("#"))
            .collect::<Vec<_>>();

        if engine_and_wrapper.is_empty() {
            action = Action::Quit;
            break;
        } else if engine_and_wrapper.len() == 2 {
            let potential_engine = engine_and_wrapper[0];
            let potential_wrapper = engine_and_wrapper[1];
            println!("You have selected:");
            println!("  Engine:  {potential_engine}");
            println!("  Wrapper: {potential_wrapper}");
            let answer = prompt("Is this correct? [yY/nN/q] ", |answer| {
                ["y", "Y", "n", "N", "q"].contains(&answer.trim())
            })?;
            let answer = answer.trim();

            if ["y", "Y"].contains(&answer) {
                action = Action::EngineAndWrapper {
                    engine: potential_engine.to_owned(),
                    wrapper: potential_engine.to_owned(),
                };
                break;
            } else if answer == "q" {
                action = Action::Quit;
                break;
            }
        }
    }

    match action {
        Action::EngineAndWrapper { engine, wrapper } => {
            todo!("Make wrapper");
        }
        Action::Quit => {
            eprintln!("Quitting Keg creator");
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut context = NavContext::default();

    let kegs_view = context.view("kegs", &views::kegs::KegsView);
    let credits_view = context.view("credits", &views::credits::CreditsView);

    let main_nav = context.nav(
        "main",
        [
            MenuItem::new("Kegs", MenuItemAction::LoadView(kegs_view)),
            MenuItem::new("Create Keg", MenuItemAction::External(create_keg)),
            MenuItem::new(
                "Clear Winetricks Cache",
                MenuItemAction::External(clear_winetricks_cache),
            ),
            MenuItem::new("Credits", MenuItemAction::LoadView(credits_view)),
        ],
    );

    context.nav(
        "keg",
        [
            MenuItem::new("Back", MenuItemAction::NavAction(NavAction::Pop)),
            MenuItem::new("Launch", MenuItemAction::External(launch_keg))
                .default(),
            MenuItem::new("Winetricks", MenuItemAction::External(winetricks)),
            MenuItem::new(
                "Open C Drive",
                MenuItemAction::External(open_c_drive),
            ),
            MenuItem::new("Edit Config", MenuItemAction::External(edit_config)),
            MenuItem::new(
                "Kill Processes",
                MenuItemAction::External(kill_wineserver),
            )
            .default(),
        ],
    );

    let app_config_file_path = app_config_file_path();
    if !app_config_file_path.try_exists().unwrap_or_else(|_| {
        panic!(
            "Failed to check existence of {}",
            app_config_file_path.display()
        )
    }) {
        let parent_directory = app_config_file_path
            .parent()
            .expect("app_config_file_path should be a full path to the file");
        fs::create_dir_all(parent_directory).unwrap_or_else(|_| {
            panic!("Failed to create directory {}", parent_directory.display())
        });
        fs::write(&app_config_file_path, "").unwrap_or_else(|_| {
            panic!(
                "Failed to create empty config file at {}",
                app_config_file_path.display()
            )
        });
    }
    let app_config_file_contents = fs::read_to_string(&app_config_file_path)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to read config file {} as string",
                app_config_file_path.display()
            )
        });
    let app_config = Arc::new(
        toml::from_str::<AppConfig>(&app_config_file_contents).unwrap_or_else(
            |_| {
                panic!(
                    "Failed to parse config file {}",
                    app_config_file_path.display()
                )
            },
        ),
    );

    let (async_state, _terminate_worker_guard) =
        spawn_worker(app_config.clone());

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new(&app_config).run(
        &mut context,
        main_nav,
        &mut terminal,
        async_state,
    );
    ratatui::restore();
    app_result
}
