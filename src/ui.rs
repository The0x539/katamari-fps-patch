use std::cell::RefCell;

use eframe::egui::{self, Ui, Widget};

use crate::ps2;
use crate::ui::{definition_file::DefinitionFile, widgets::Research};

mod definition_file;
mod widgets;

// implementation details that I hope to worry about as little as possible
// the fix for the stupid hang-on-resize would go somewhere in that module
mod frame;
use frame::UiHandle;

static mut MONO_CTRL_IDX: u16 = u16::MAX;

#[derive(Default)]
struct MyApp {
    definitions: Option<Result<DefinitionFile, String>>,
}

impl MyApp {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("Reload fields.ini").clicked() || self.definitions.is_none() {
            self.definitions = Some(DefinitionFile::load());
        }

        let definitions = match self.definitions.as_ref().unwrap() {
            Ok(defs) => defs,
            Err(msg) => {
                ui.colored_label(egui::Color32::RED, msg);
                return;
            }
        };

        unsafe {
            ui.label("Prince");
            let prince = ps2::current_prince();
            prince.with(definitions).ui(ui);
        }

        unsafe {
            ui.label("Katamari");
            let katamari = ps2::current_katamari();
            katamari.with(definitions).ui(ui);
        }

        unsafe {
            let idx = MONO_CTRL_IDX;
            if idx >= 4000 {
                return;
            }
            let thing = &mut *ps2::prop_array(idx as usize);
            if thing.mono_name_idx >= 1718 {
                return;
            }

            thing.with(definitions).ui(ui);
        }
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
