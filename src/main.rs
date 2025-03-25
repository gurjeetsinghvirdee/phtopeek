use eframe::egui;
use egui_extras::RetainedImage;
use std::fs;
use std::path::PathBuf;

struct PhotoPeek {
    images: Vec<PathBuf>,
    current_index: usize,
    current_image: Option<RetainedImage>,
}

impl PhotoPeek {
    fn new(image_folder: &str) -> Self {
        let images: Vec<PathBuf> = fs::read_dir(image_folder)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().map(|ext| ext == "png" || ext == "jpg" || ext == "jpeg").unwrap_or(false))
            .colllect();

        let mut viewer = Self {
            images,
            current_index: 0,
            current_image: None,
        };
        viewer.load_image();
        viewer
    }

    fn load_image(&mut self) {
        if let Some(path) = self.images.get(self.current_index) {
            if let Ok(image) = RetainedImage::from_image_path(path.to_string_lossy()) {
                self.current_image = Some(image);
            }
        }
    }

    fn next_image(&mut self) {
        if self.current_index + 1 < self.images.len() {
            self.current_index += 1;
            self.load_image();
        }
    }

    fn prev_image(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.load_image();
        }
    }
}

impl eframe::App for ImageViewer {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(image) = &self.current_image {
                image.show(ui);
            } else {
                ui.label("No images found");
            }

            ui.horizontal(|ui| {
                if ui.button("Prev").clicked() {
                    self.prev_image();
                }
                if ui.button("Next").clicked() {
                    self.next_image();
                }
            });
        });
    } 
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Photopeek",
        options,
        Box::new(|_| Box::new(PhotoPeek::new("images"))),
    )
}