use std::cell::RefCell;
use std::time::Duration;

use eframe::egui::{self, DragValue, ViewportBuilder, Widget};
use eframe::{App, EframeWinitApplication, NativeOptions, UserEvent};
use winit::event_loop::EventLoop;
use winit::platform::{pump_events::EventLoopExtPumpEvents, windows::EventLoopBuilderExtWindows};

use crate::ps2;
use crate::types::{MotionVectors, Vec4};

fn draw(ui: &mut egui::Ui) {
    let k = unsafe { &mut *ps2::katamari_array(0) };

    k.position.ui(&mut *ui);

    ui.horizontal(|ui| {
        k.dual_a.ui(ui);
        k.dual_b.ui(ui);
    });
}

impl egui::Widget for &mut Vec4 {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("x");
            DragValue::new(&mut self.x).ui(ui);
            ui.label("y");
            DragValue::new(&mut self.y).ui(ui);
            ui.label("z");
            DragValue::new(&mut self.z).ui(ui);
        })
        .response
    }
}

impl egui::Widget for &mut MotionVectors {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            for (vec, label) in self.as_slice_mut().iter_mut().zip('A'..) {
                ui.horizontal(|ui| {
                    ui.label(label.to_string());
                    vec.ui(ui);
                });
            }
        })
        .response
    }
}

#[unsafe(export_name = "PreTick")]
pub unsafe extern "C" fn pre_tick() {}

#[unsafe(export_name = "PostTick")]
pub unsafe extern "C" fn post_tick() {
    thread_local! {
        static UI_HANDLE: RefCell<UiHandle> = RefCell::new(UiHandle::new());
    }

    let timeout = Some(Duration::from_millis(1));

    UI_HANDLE.with(|h| {
        let mut h = h.borrow_mut();
        let h = &mut *h;

        h.event_loop.pump_app_events(timeout, &mut h.app);
    });
}

// implementation details below

struct UiHandle {
    event_loop: EventLoop<UserEvent>,
    app: EframeWinitApplication<'static>,
}

impl UiHandle {
    fn new() -> Self {
        let event_loop = EventLoop::with_user_event()
            .with_any_thread(true)
            .build()
            .unwrap();

        let mut opts = NativeOptions::default();
        opts.window_builder = Some(Box::new(|b: ViewportBuilder| {
            b.with_resizable(false).with_inner_size((500.0, 800.0))
        }));

        let app = eframe::create_native("katamari", opts, Box::new(MyApp::new_app), &event_loop);

        Self { event_loop, app }
    }
}

struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, draw);
        ctx.request_repaint();
    }
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }

    fn new_app<E>(cc: &eframe::CreationContext<'_>) -> Result<Box<dyn App>, E> {
        Ok(Box::new(Self::new(cc)))
    }
}
