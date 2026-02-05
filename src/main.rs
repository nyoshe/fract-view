mod fractal;
mod render;
mod viewport;

use eframe::egui;
use egui::{Color32, ColorImage, TextureHandle, TextureOptions, Vec2};
use num_complex::Complex64;

use crate::{fractal::Mandelbrot, viewport::Viewport};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Fractal",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    mandelbrot: Mandelbrot,
    viewport: Viewport,
    texture: Option<TextureHandle>,
    last_size: Vec2,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            mandelbrot: Mandelbrot { max_iter: 256 },
            viewport: Viewport {
                center: Complex64::new(-0.5, 0.0),
                scale: 0.004,
            },
            texture: None,
            last_size: Vec2::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            // Check if window size changed
            let size_changed = (available_size - self.last_size).length() > 0.1;

            if size_changed || self.texture.is_none() {
                self.last_size = available_size;

                let width = available_size.x as usize;
                let height = available_size.y as usize;

                if width > 0 && height > 0 {
                    // Render the fractal
                    let mut pixels = vec![Color32::BLACK; width * height];
                    render::render(&self.mandelbrot, self.viewport, width, height, &mut pixels);

                    // Create texture from pixels
                    let image = ColorImage {
                        size: [width, height],
                        pixels,
                    };

                    self.texture =
                        Some(ctx.load_texture("fractal", image, TextureOptions::NEAREST));
                }
            }

            // Display the texture
            if let Some(texture) = &self.texture {
                ui.image((texture.id(), available_size));
            }
        });
    }
}
