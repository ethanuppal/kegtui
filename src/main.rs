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

use crate::{
    app::App,
    view::{MenuItem, MenuItemAction, NavContext},
};
use app::spawn_worker;
use checks::is_kegworks_installed;
use color_eyre::Result;
use view::NavAction;
use views::keg_main::{edit_config, open_c_drive};

pub mod app;
pub mod checks;
pub mod keg;
pub mod keg_config;
pub mod keg_plist;
pub mod view;
pub mod views;

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

    let keg_main_view = context.view("keg_main", &views::keg_main::KegMainView);

    let keg_nav = context.nav(
        "keg",
        [
            MenuItem::new("Back", MenuItemAction::NavAction(NavAction::Pop)),
            MenuItem::new("Main", MenuItemAction::LoadView(keg_main_view))
                .default(),
            //MenuItem::new("Winetricks", todo!()),
            MenuItem::new(
                "Open C Drive",
                MenuItemAction::External(open_c_drive),
            ),
            MenuItem::new("Edit Config", MenuItemAction::External(edit_config)),
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
