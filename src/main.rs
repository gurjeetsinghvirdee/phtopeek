// Import necessary libraries for GUI and image handling
use eframe::egui;
use egui::{ColorImage, TextureHandle};
use egui_extras::image::load_image_bytes; // Handles images efficiently
use std::fs;
use std::path::PathBuf;

// Define the main application struct
struct PhotoPeek {
    images: Vec<PathBuf>,   // Stores paths of image in the directory
    current_index: usize,   
    current_image: Option<TextureHandle>,  // Stores the current image texture
}

impl PhotoPeek {
    // Initialize the application and load images from a given directory
    fn new(image_folder: &str, ctx: &egui::Context) -> Self {
        let images: Vec<PathBuf> = fs::read_dir(image_folder)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path())) // Extract valid paths
            .filter(|path| path.extension().map(|ext| matches!(ext.to_str(), Some("png" | "jpg" | "jpeg"))).unwrap_or(false))
            .collect();

        let mut viewer = Self {
            images,
            current_index: 0,   // Start with the first image
            current_image: None,    // No image loaded yet
        };
        viewer.load_image(ctx);
        viewer
    }

    // Function to load the current image from the list
    fn load_image(&mut self, ctx: &egui::Context) {
        if let Some(path) = self.images.get(self.current_index) {
            if let Ok(bytes) = fs::read(path) {
                if let Ok(image) = load_image_bytes(&bytes) {
                    let color_image = ColorImage::from_rgba_unmultiplied(
                        [image.width() as _, image.height() as _],
                        &image.pixels,
                    );
                    self.current_image = Some(ctx.load_texture("image", color_image));
                }
            }
        }
    }

    // Function to navigate to the next image
    fn next_image(&mut self, ctx: &egui::Context) {
        if self.current_index + 1 < self.images.len() { // Check if next image exists
            self.current_index += 1;
            self.load_image(ctx);
        }
    }

    // Function to navigate to the previous image
    fn prev_image(&mut self, ctx: &egui::Context) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.load_image(ctx);
        }
    }
}

// Implementing the GUI logic using eframe
impl eframe::App for PhotoPeek {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the current image if available
            if let Some(image) = &self.current_image {
                ui.image(image, image.size_vec2());
            } else {
                ui.label("No images found");
            }

            ui.horizontal(|ui| {
                if ui.button("Prev").clicked() {
                    self.prev_image(ctx);
                }
                if ui.button("Next").clicked() {
                    self.next_image(ctx);
                }
            });
        });
    } 
}

// Main function to run the app
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default(); // Default GUI options
    eframe::run_native(
        "Rust PhotoPeek",
        options,
        Box::new(|cc| Box::new(PhotoPeek::new("images", &cc.egui_ctx))),
    )
}