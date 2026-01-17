use std::cell::RefCell;
use std::fs::File;
use std::time::{Duration, SystemTime};

use eframe::egui;
use egui::{Color32, Frame, Ui, Widget};

use crate::ps2;
use crate::ui::{definition_file::DefinitionFile, widgets::Research};

mod definition_file;
mod widgets;

// implementation details that I hope to worry about as little as possible
// the fix for the stupid hang-on-resize would go somewhere in that module
mod frame;
use frame::UiHandle;

static mut MONO_CTRL_IDX: u16 = u16::MAX;

struct MyApp {
    file: File,
    last_updated: SystemTime,
    definitions: Result<DefinitionFile, String>,
}

impl MyApp {
    fn should_reload_file(&self) -> std::io::Result<bool> {
        let mtime = self.file.metadata()?.modified()?;
        Ok(mtime >= self.last_updated)
    }

    fn draw(&mut self, ui: &mut Ui) {
        if self
            .last_updated
            .elapsed()
            .is_ok_and(|t| t > Duration::from_secs(2))
        {
            if self.should_reload_file().unwrap_or(false) {
                self.definitions = DefinitionFile::load(&mut self.file);
            }
            self.last_updated = SystemTime::now();
        }

        let definitions = match &self.definitions {
            Ok(defs) => defs,
            Err(msg) => {
                ui.colored_label(egui::Color32::RED, msg);
                return;
            }
        };

        let frame = Frame::new()
            .stroke((1.0, Color32::DARK_GRAY))
            .inner_margin(4);

        frame.show(ui, |ui| unsafe {
            ui.label("Prince");
            let prince = ps2::current_prince();
            prince.with(definitions).ui(ui);
        });

        frame.show(ui, |ui| unsafe {
            ui.label("Camera");
            let camera = ps2::current_camera();
            camera.with(definitions).ui(ui);
        });

        frame.show(ui, |ui| unsafe {
            ui.label("Katamari");
            let katamari = ps2::current_katamari();
            katamari.with(definitions).ui(ui);
        });

        frame.show(ui, |ui| unsafe {
            let idx = MONO_CTRL_IDX;
            if idx >= 4000 {
                return;
            }
            let thing = &mut *ps2::prop_array(idx as usize);
            if thing.mono_name_idx >= 1718 {
                return;
            }

            thing.with(definitions).ui(ui);
        });
    }
}

#[unsafe(export_name = "UpdateUI")]
pub unsafe extern "C" fn update_ui() {
    thread_local! {
        static UI_HANDLE: RefCell<UiHandle> = RefCell::new(UiHandle::new());
    }

    UI_HANDLE.with(|h| {
        let mut h = h.borrow_mut();
        let h = &mut *h;
        h.app.pump_eframe_app(&mut h.event_loop, None);
    });
}

#[unsafe(export_name = "PickThing")]
pub extern "C" fn pick_thing(idx: u16) {
    unsafe {
        MONO_CTRL_IDX = idx;
    }
}
