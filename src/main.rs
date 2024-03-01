use eframe::egui;
use tracing_subscriber::prelude::*;

fn main() {
    let (appender, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        std::env::current_dir().unwrap(),
        "log",
    ));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(appender.with_max_level(tracing::Level::DEBUG)),
        )
        .init();
    eframe::run_native(
        env!("CARGO_BIN_NAME"),
        eframe::NativeOptions::default(),
        Box::new(|_creation_context| Box::new(App)),
    )
    .unwrap();
    drop(guard);
}

pub struct App;

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        if context.input_mut(|input| input.consume_key(egui::Modifiers::CTRL, egui::Key::W)) {
            context.send_viewport_cmd(egui::ViewportCommand::Close);
            // std::process::exit(0);
        }
    }
}
