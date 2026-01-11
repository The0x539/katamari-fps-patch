use std::cell::RefCell;

use eframe::egui::{self, DragValue, Ui, ViewportBuilder, Widget};
use eframe::{App, EframeWinitApplication, NativeOptions, UserEvent};
use winit::event_loop::EventLoop;
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::ps2;
use crate::types::{Katamari, MotionVectors, Vec4};

static mut MONO_CTRL_IDX: u16 = u16::MAX;

fn draw(ui: &mut Ui) {
    let idx = unsafe { MONO_CTRL_IDX };
    if idx >= 4000 {
        return;
    }

    let thing = unsafe { &mut *ps2::prop_array(idx as usize) };

    if thing.mono_name_idx >= 1718 {
        return;
    }

    let data = unsafe { &*ps2::prop_data_array(thing.mono_name_idx as usize) };

    let name = unsafe { std::ffi::CStr::from_ptr(data.name) }
        .to_str()
        .unwrap();

    ui.label(format!("{idx}: {name}"));
    ui.label(format!("{}", thing.mono_ctrl_idx));
    label_vec(ui, "position", &mut thing.position);
    label_vec(ui, "rotation", &mut thing.rotation);
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

impl egui::Widget for &mut Katamari {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        label_vec(ui, "pos", &mut self.position);
        label_vec(ui, "pivot", &mut self.pivot_position);

        label_value(ui, "mass", &mut self.mass);
        label_value(ui, "speed", &mut self.speed);
        label_value(ui, "spin", &mut self.angular_speed);

        ui.label(&format!("airborne: {}", self.airborne));
        ui.label(&format!("climbing: {}", self.climbing));

        ui.label(&format!("pivot time: {}", self.time_spent_standing_on_prop));
        ui.label(&format!("xea: {}", self.xea));

        ui.label(&format!("x10b: {}", self.x10b));
        ui.label(&format!("x10c: {:?}", self.x10c));

        label_value(ui, "pivot speed", &mut self.pivot_speed);

        label_value(ui, "angle guy", &mut self.angle_guy);

        ui.horizontal(|ui| {
            self.motion.ui(ui);
            self.prev_motion.ui(ui);
        });

        ui.response()
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
