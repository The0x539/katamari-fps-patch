use std::cell::RefCell;
use std::time::Duration;

use eframe::egui;
use eframe::{App, EframeWinitApplication, UserEvent};
use winit::event_loop::EventLoop;
use winit::platform::{pump_events::EventLoopExtPumpEvents, windows::EventLoopBuilderExtWindows};

use crate::ps2;

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

        let opts = Default::default();
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

fn draw(ui: &mut egui::Ui) {
    let k = unsafe { &mut *ps2::katamari_array(0) };

    ui.label(&format!("{:?}", k.position));
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }

    fn new_app<E>(cc: &eframe::CreationContext<'_>) -> Result<Box<dyn App>, E> {
        Ok(Box::new(Self::new(cc)))
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
