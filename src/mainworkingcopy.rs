use eframe::{egui, epi};
use flate2::write::GzEncoder;
use flate2::Compression;
use rfd::FileDialog;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

struct MyApp {
    source_path: String,
    target_path: String,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            source_path: String::new(),
            target_path: String::new(),
            status: String::new(),
        }
    }
}

impl MyApp {
    fn compress_file(&mut self) {
        let start = Instant::now();

        let input = File::open(&self.source_path);
        let output = File::create(&self.target_path);

        if let (Ok(file), Ok(target)) = (input, output) {
            let mut reader = BufReader::new(file);
            let mut encoder = GzEncoder::new(target, Compression::default());

            if copy(&mut reader, &mut encoder).is_ok() {
                let _ = encoder.finish();
                let elapsed = start.elapsed();
                self.status = format!("Compressed successfully in {:?}", elapsed);
            } else {
                self.status = "Error during compression.".to_string();
            }
        } else {
            self.status = "Error opening files.".to_string();
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Select Source File:");
            if ui.button("Browse...").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.source_path = path.display().to_string();
                }
            }
            ui.label(&self.source_path);

            ui.label("Select Target File:");
            if ui.button("Browse...").clicked() {
                if let Some(path) = FileDialog::new().save_file() {
                    self.target_path = path.display().to_string();
                }
            }
            ui.label(&self.target_path);

            if ui.button("Compress").clicked() {
                if !self.source_path.is_empty() && !self.target_path.is_empty() {
                    self.compress_file();
                } else {
                    self.status = "Please select both source and target paths.".to_string();
                }
            }

            ui.label(&self.status);
        });
    }

    fn name(&self) -> &str {
        "File Compressor"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 200.0)),
        ..Default::default()
    };

    let app = MyApp::default();
    eframe::run_native(Box::new(app), options); // Box the app instance

    // Ok(())
}
