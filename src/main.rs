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

use std::{env, fs, io, process::Command, thread, time::Duration};

use crate::{
    app::App,
    view::{MenuItem, MenuItemAction, NavContext},
};
use app::{spawn_worker, AsyncState};
use checks::is_kegworks_installed;
use color_eyre::Result;
use view::NavAction;

pub mod app;
pub mod checks;
pub mod keg;
pub mod keg_config;
pub mod keg_plist;
pub mod view;
pub mod views;

pub fn open_c_drive(app: &mut App, _state: &AsyncState) -> Result<()> {
    let Some(current_keg) = &app.current_keg else {
        return Ok(());
    };
    if let Ok(explorer) = env::var("EXPLORER") {
        Command::new(explorer)
            .arg(current_keg.c_drive.to_string_lossy().to_string())
            .status()?;
    } else {
        Command::new("open")
            .arg(current_keg.c_drive.to_string_lossy().to_string())
            .status()?;
    }
    Ok(())
}

pub fn edit_config(app: &mut App, _state: &AsyncState) -> Result<()> {
    if let Some(current_keg) = &mut app.current_keg {
        let toml_config =
            toml::to_string_pretty(&current_keg.plist.extract_config())?;
        let file = "/tmp/kegtui.toml";
        fs::write(file, toml_config)?;
        let editor = env::var("EDITOR").unwrap_or("vim".into());
        Command::new(editor).arg(file).status()?;
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
        io::stdin().read_line(&mut String::new())?;
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

fn main() -> Result<()> {
    let mut context = NavContext::default();

    let setup_wizard_view =
        context.view("wizard", &views::setup_wizard::SetupWizardView);

    let setup_wizard_nav = context.nav(
        "wizard",
        [MenuItem::new(
            "Setup Wizard",
            MenuItemAction::LoadView(setup_wizard_view),
        )],
    );

    let kegs_view = context.view("kegs", &views::kegs::KegsView);
    let credits_view = context.view("credits", &views::credits::CreditsView);

    let main_nav = context.nav(
        "main",
        [
            MenuItem::new("Kegs", MenuItemAction::LoadView(kegs_view)),
            MenuItem::new("Credits", MenuItemAction::LoadView(credits_view)),
        ],
    );

    let keg_nav = context.nav(
        "keg",
        [
            MenuItem::new("Back", MenuItemAction::NavAction(NavAction::Pop)),
            MenuItem::new("Launch", MenuItemAction::External(launch_keg))
                .default(),
            //MenuItem::new("Winetricks", todo!()),
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
