use eframe::{egui, epi};
use flate2::{write::GzEncoder, read::GzDecoder};
use flate2::Compression;
use rfd::FileDialog;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;

struct MyApp {
    source_path: String,
    target_path: String,
    status: String,
    progress: f32,
    pulse: f32,
    is_compression: bool, // Track mode (compression/decompression)
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            source_path: String::new(),
            target_path: String::new(),
            status: String::new(),
            progress: 0.0,
            pulse: 0.0,
            is_compression: true, // Default to compression
        }
    }
}

impl MyApp {
    fn compress_file(&mut self) {
        let start = Instant::now();
        let input = File::open(&self.source_path);
        
        // Create the output path with .gz extension
        let output_path = format!("{}.gz", &self.target_path);
        let output = File::create(&output_path);

        if let (Ok(file), Ok(target)) = (input, output) {
            let mut reader = BufReader::new(file);
            let mut encoder = GzEncoder::new(target, Compression::default());

            if copy(&mut reader, &mut encoder).is_ok() {
                let _ = encoder.finish();
                let elapsed = start.elapsed();
                self.status = format!("Compressed successfully in {:?}", elapsed);
                self.progress = 1.0;
            } else {
                self.status = "Error during compression.".to_string();
            }
        } else {
            self.status = "Error opening files.".to_string();
        }
    }
    
    fn decompress_file(&mut self) {
        let start = Instant::now();
        let input = File::open(&self.source_path);
        
        // Use the target path as the output file name (without .gz extension)
        let output_path = self.target_path.trim_end_matches(".gz").to_string();
        let output = File::create(&output_path);

        if let (Ok(file), Ok(target)) = (input, output) {
            let mut reader = GzDecoder::new(file);
            let mut writer = BufWriter::new(target);

            if copy(&mut reader, &mut writer).is_ok() {
                let elapsed = start.elapsed();
                self.status = format!("Decompressed successfully in {:?}", elapsed);
                self.progress = 1.0;
            } else {
                self.status = "Error during decompression.".to_string();
            }
        } else {
            self.status = "Error opening files.".to_string();
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        self.pulse = (self.pulse + 0.05) % std::f32::consts::TAU;
        let glow = (self.pulse.sin() * 0.5 + 0.5) as f32;

        let frame = egui::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(10, 10, 20, 200),
            rounding: egui::Rounding::same(15.0),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(100, 200, 255, (glow * 255.0) as u8)),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_unmultiplied(30, 30, 40, 200);
            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(40, 40, 50, 200);
            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(50, 50, 60, 200);
            ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(60, 60, 70, 200);

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("Quantum File Compressor by Marmik Mewada")
                    .color(egui::Color32::from_rgb(100, 200, 255))
                    .size(28.0)
                    .strong());
                ui.add_space(20.0);

                // Toggle between compression and decompression
                ui.horizontal(|ui| {
                    if ui.radio(self.is_compression, "Compress").clicked() {
                        self.is_compression = true;
                    }
                    if ui.radio(!self.is_compression, "Decompress").clicked() {
                        self.is_compression = false;
                    }
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Source File:").color(egui::Color32::from_rgb(150, 220, 255)));
                    if ui.button("🔍 Browse").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.source_path = path.display().to_string();
                        }
                    }
                });
                ui.add(egui::TextEdit::singleline(&mut self.source_path)
                    .hint_text("Source path...")
                    .text_color(egui::Color32::from_rgb(200, 230, 255)));

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Target File:").color(egui::Color32::from_rgb(150, 220, 255)));
                    if ui.button("💾 Save As").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            self.target_path = path.display().to_string();
                        }
                    }
                });
                ui.add(egui::TextEdit::singleline(&mut self.target_path)
                    .hint_text("Target path...")
                    .text_color(egui::Color32::from_rgb(200, 230, 255)));

                ui.add_space(20.0);

                let button_label = if self.is_compression { "🚀 Compress" } else { "📦 Decompress" };
                if ui.add_sized([ui.available_width(), 50.0], 
                    egui::Button::new(egui::RichText::new(button_label)
                        .color(egui::Color32::from_rgb(255, 255, 255))
                        .size(20.0))
                ).clicked() {
                    if !self.source_path.is_empty() && !self.target_path.is_empty() {
                        if self.is_compression {
                            self.compress_file();
                        } else {
                            self.decompress_file();
                        }
                    } else {
                        self.status = "Please select both source and target paths.".to_string();
                    }
                }

                ui.add_space(10.0);

                ui.add(egui::ProgressBar::new(self.progress)
                    .animate(true)
                    .show_percentage()
                    .desired_width(ui.available_width()));

                ui.add_space(10.0);

                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), &self.status);

                ui.add_space(20.0);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Created by Marmik Mewada from India <3")
                        .color(egui::Color32::from_rgb(150, 150, 200))
                        .size(12.0));
                });
            });
        });
    }

    fn name(&self) -> &str {
        "Quantum File Compressor 3000"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = MyApp::default();
    let icon_path = "C:\\Users\\admin\\Desktop\\rs\\compress_youtube\\src\\your_icon.ico"; // Adjust the path accordingly

    // Load the icon data
    let icon_data = std::fs::read(icon_path)?;
    let icon_size = (32, 32); // Adjust according to your icon size
    let icon_data = epi::IconData {
        rgba: icon_data,
        width: icon_size.0,
        height: icon_size.1,
    };

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(520.0, 400.0)),
        resizable: false,
        decorated: true,
        icon_data: Some(icon_data),
        ..Default::default()
    };

    eframe::run_native(Box::new(app), native_options);
    // Ok(())
}
