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
    fs,
    sync::{self, Arc, RwLock},
    thread,
};

use ratatui::DefaultTerminal;

use crate::{checks, keg::Keg, view::prelude::*};

#[derive(Default)]
pub struct App {
    pub(crate) vertical_scroll: usize,
}

impl App {
    pub fn run<'a>(
        &mut self,
        context: &mut NavContext<'a, App>,
        initial: NavID<'a>,
        terminal: &mut DefaultTerminal,
        state: Arc<RwLock<AsyncState>>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct AsyncState {
    kegs: Vec<Keg>,
    brew_installed: Option<bool>,
    kegworks_installed: Option<bool>,
}

pub struct TerminateWorker(sync::mpsc::Sender<()>);

impl Drop for TerminateWorker {
    fn drop(&mut self) {
        self.0.send(());
    }
}

pub fn spawn_worker() -> (Arc<RwLock<AsyncState>>, TerminateWorker) {
    let async_state = Arc::new(RwLock::new(AsyncState::default()));

    let (quit_tx, quit_rx) = sync::mpsc::channel();

    {
        let async_state = async_state.clone();
        thread::spawn(move || loop {
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

            let brew_installed = checks::is_brew_installed();
            let kegworks_installed = checks::is_kegworks_installed();

            if let Ok(mut lock) = async_state.try_write() {
                lock.kegs = kegs;
                lock.brew_installed = Some(brew_installed);
                lock.kegworks_installed = Some(kegworks_installed);
            }
        });
    }

    (async_state, TerminateWorker(quit_tx))
}
