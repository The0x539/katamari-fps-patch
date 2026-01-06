use std::cell::RefCell;
use std::time::Duration;

use eframe::egui::{self, DragValue, Ui, ViewportBuilder, Widget};
use eframe::{App, EframeWinitApplication, NativeOptions, UserEvent};
use winit::event_loop::EventLoop;
use winit::platform::{pump_events::EventLoopExtPumpEvents, windows::EventLoopBuilderExtWindows};

use crate::ps2;
use crate::types::{MotionVectors, Vec4};

fn draw(ui: &mut Ui) {
    let k = unsafe { &mut *ps2::katamari_array(0) };

    label_vec(ui, "pos", &mut k.position);
    label_vec(ui, "pivot", &mut k.pivot_position);

    label_value(ui, "x1a0", &mut k.x1a0);
    label_value(ui, "speed", &mut k.speed);
    label_value(ui, "spin", &mut k.angular_speed);
    label_vec(ui, "ground", &mut k.ground_normal);
    label_vec(ui, "wall", &mut k.wall_normal);

    ui.label(&format!("airborne: {}", k.airborne));
    ui.label(&format!("climbing: {}", k.climbing));

    label_vec(ui, "direction", &mut k.roll_direction);
    label_vec(ui, "left", &mut k.left_direction);

    label_vec(ui, "local pivot pos", &mut k.local_pivot_pos);

    ui.label(&format!("stgr: {}", k.time_spent_standing_on_prop));

    label_value(ui, "pivot speed", &mut k.x39b8);

    ui.horizontal(|ui| {
        k.dual_a.ui(ui);
        k.dual_b.ui(ui);
    });
}

fn label_value(ui: &mut Ui, label: &str, value: &mut f32) {
    ui.horizontal(|ui| {
        ui.label(label);
        DragValue::new(value).ui(ui);
    });
}

fn label_vec(ui: &mut Ui, label: &str, value: &mut Vec4) {
    ui.horizontal(|ui| {
        ui.label(label);
        value.ui(ui);
    });
}

impl egui::Widget for &mut Vec4 {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            label_value(ui, "x", &mut self.x);
            label_value(ui, "y", &mut self.y);
            label_value(ui, "z", &mut self.z);
        })
        .response
    }
}

impl egui::Widget for &mut MotionVectors {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            for (vec, label) in self.as_slice_mut().iter_mut().zip('A'..) {
                label_vec(ui, &label.to_string(), vec);
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
        opts.window_builder = Some(Box::new(|mut b: ViewportBuilder| {
            b.resizable = Some(false);
            b.inner_size = Some((500.0, 700.0).into());
            b.active = Some(false);
            b
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
