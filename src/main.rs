// Import necessary libraries for GUI and image handling
use eframe::egui;
use egui_extras::RetainedImage; // Handles images efficiently
use std::fs;
use std::path::PathBuf;

// Define the main application struct
struct PhotoPeek {
    images: Vec<PathBuf>,   // Stores paths of image in the dir_
    current_index: usize,   
    current_image: Option<RetainedImage>,  // Stores the current image
}

impl PhotoPeek {
    // Intialize the application and load images from a given dir_
    fn new(image_folder: &str) -> Self {
        let images: Vec<PathBuf> = fs::read_dir(image_folder)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path())) // Extract Valid paths
            .filter(|path| path.extension().map(|ext| ext == "png" || ext == "jpg" || ext == "jpeg").unwrap_or(false))
            .collect();

        let mut viewer = Self {
            images,
            current_index: 0,   // Start with the first image
            current_image: None,    // No image loaded yet
        };
        viewer.load_image();
        viewer
    }

    // Function to load the current image from the list
    fn load_image(&mut self) {
        if let Some(path) = self.images.get(self.current_index) {
            if let Ok(image) = RetainedImage::from_image_path(path.to_string_lossy()) {
                self.current_image = Some(image);   // Store the loaded image
            }
        }
    }

    // Function to navigate to the next image
    fn next_image(&mut self) {
        if self.current_index + 1 < self.images.len() { // check if next image exits
            self.current_index += 1;
            self.load_image();
        }
    }

    // Function to navigate to the previous image
    fn prev_image(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.load_image();
        }
    }
}

// Implementing the GUI logic using eframe
impl eframe::App for ImageViewer {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the current image if available
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

// Main function to run the app
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default(); // Default GUI options
    eframe::run_native(
        "Rust Photopeek",
        options,
        Box::new(|_| Box::new(PhotoPeek::new("images"))),
    )
}