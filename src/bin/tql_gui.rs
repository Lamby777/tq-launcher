#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use anyhow::Result;
use eframe::egui;
use tq_launcher::TqlOptions;

fn main() -> Result<()> {
    let _opts = TqlOptions { silent: false };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TerraQuest Launcher",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<App>::default()
        }),
    )
    .unwrap();

    Ok(())
}

struct App {
    version: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            version: "v0.0.1".to_owned(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TQ Launcher GUI");
            ui.horizontal(|ui| {
                let name_label = ui.label("Version: ");
                ui.text_edit_singleline(&mut self.version)
                    .labelled_by(name_label.id);
            });
            ui.label(format!("Install version {}?", self.version));
        });
    }
}
