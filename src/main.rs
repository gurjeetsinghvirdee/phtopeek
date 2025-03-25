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
    
}