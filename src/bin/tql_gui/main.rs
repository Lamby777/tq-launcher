#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod themes;

use anyhow::Result;
use eframe::egui;
use tq_launcher::TqlOptions;

fn main() -> Result<()> {
    let _opts = TqlOptions { silent: false };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        centered: true,
        persist_window: true,

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
        ctx.set_visuals(themes::dark());
        ctx.set_fonts(themes::fonts());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("TQ Launcher GUI");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.vertical_centered(|ui| {
                        let name_label = ui.label("Version: ");
                        ui.text_edit_singleline(&mut self.version)
                            .labelled_by(name_label.id);
                    });
                });
                ui.label(format!("Install version {}?", self.version));
            });
        });
    }
}
