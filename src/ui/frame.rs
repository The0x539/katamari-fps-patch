use std::fs::File;
use std::time::SystemTime;

use eframe::egui;
use eframe::{App, EframeWinitApplication, NativeOptions, UserEvent};
use egui::{TextStyle, ViewportBuilder};
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
            b.position = Some((10.0, 10.0).into());
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
    fn new_app(
        ctx: &eframe::CreationContext<'_>,
    ) -> Result<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>> {
        ctx.egui_ctx.all_styles_mut(|style| {
            style.override_text_style = Some(TextStyle::Monospace);
            style.drag_value_text_style = TextStyle::Monospace;
        });

        let file = File::open("./fields.ini")?;
        let definitions = super::DefinitionFile::load(&file);
        let last_updated = SystemTime::now();
        Ok(Box::new(MyApp {
            file,
            last_updated,
            definitions,
        }))
    }
}
