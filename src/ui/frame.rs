use eframe::egui::{self, ViewportBuilder};
use eframe::{App, EframeWinitApplication, NativeOptions, UserEvent};
use winit::event_loop::EventLoop;
use winit::platform::windows::EventLoopBuilderExtWindows;

pub struct UiHandle {
    pub event_loop: EventLoop<UserEvent>,
    pub app: EframeWinitApplication<'static>,
}

impl UiHandle {
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event()
            .with_any_thread(true)
            .build()
            .unwrap();

        let mut opts = NativeOptions::default();
        opts.window_builder = Some(Box::new(|mut b: ViewportBuilder| {
            b.resizable = Some(false);
            b.inner_size = Some((700.0, 960.0).into());
            b.active = Some(false);
            b
        }));

        let app = eframe::create_native("katamari", opts, Box::new(MyApp::new_app), &event_loop);

        Self { event_loop, app }
    }
}

use super::MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.draw(ui));
        ctx.request_repaint();
    }
}

impl MyApp {
    fn new_app<E>(_ctx: &eframe::CreationContext<'_>) -> Result<Box<dyn App>, E> {
        Ok(Box::new(Self::default()))
    }
}
