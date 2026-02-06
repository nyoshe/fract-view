mod fractal;
mod fractal_image;
mod render;
mod viewport;

use eframe::egui;
use egui::{ColorImage, TextureHandle, TextureOptions, Vec2};
use num_complex::Complex64;

use crate::{fractal::Mandelbrot, fractal_image::FractalImage, viewport::Viewport};

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
    fractal_image: Option<FractalImage>,
    texture: Option<TextureHandle>,
    last_size: Vec2,
    show_debug: bool,
    mouse_pos: Option<egui::Pos2>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            mandelbrot: Mandelbrot { max_iter: 256 },
            fractal_image: None,
            texture: None,
            last_size: Vec2::ZERO,
            show_debug: true,
            mouse_pos: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Debug window
        if self.show_debug {
            egui::Window::new("Debug")
                .default_pos([10.0, 10.0])
                .show(ctx, |ui| {
                    if let Some(ref fimg) = self.fractal_image {
                        let viewport = fimg.viewport();
                        ui.label(format!("Zoom: {:.2e}", 1.0 / viewport.scale));
                        ui.label(format!("Scale: {:.6}", viewport.scale));
                        ui.label(format!(
                            "Center: {:.6} + {:.6}i",
                            viewport.center.re, viewport.center.im
                        ));

                        ui.separator();

                        if let Some(pos) = self.mouse_pos {
                            ui.label(format!("Mouse (app): ({:.1}, {:.1})", pos.x, pos.y));

                            // Convert mouse position to complex coordinates
                            let width = fimg.width();
                            let height = fimg.height();
                            if width > 0 && height > 0 {
                                let complex = viewport.px_to_cmplx(
                                    pos.x as usize,
                                    pos.y as usize,
                                    width,
                                    height,
                                );
                                ui.label(format!(
                                    "Mouse (complex): {:.6} + {:.6}i",
                                    complex.re, complex.im
                                ));
                            }
                        } else {
                            ui.label("Mouse: N/A");
                        }
                    }

                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_debug = false;
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            // Track mouse position
            if let Some(pos) = ctx.pointer_hover_pos() {
                self.mouse_pos = Some(pos);
            }

            let width = available_size.x as usize;
            let height = available_size.y as usize;

            // Initialize fractal image if needed
            if self.fractal_image.is_none() && width > 0 && height > 0 {
                let viewport = Viewport {
                    center: Complex64::new(-0.5, 0.0),
                    scale: 0.004,
                };
                let mut fimg = FractalImage::new(width, height, viewport);
                fimg.render(&self.mandelbrot, viewport);
                self.fractal_image = Some(fimg);
            }

            // Check if window size changed
            let size_changed = (available_size - self.last_size).length() > 0.1;

            if size_changed && width > 0 && height > 0 {
                self.last_size = available_size;

                if let Some(ref mut fimg) = self.fractal_image {
                    let viewport = fimg.viewport();
                    fimg.resize(width, height);
                    fimg.render(&self.mandelbrot, viewport);
                }
            }

            // Handle mouse interactions
            let response = ui.allocate_rect(
                egui::Rect::from_min_size(ui.min_rect().min, available_size),
                egui::Sense::click_and_drag(),
            );

            // Handle panning with left mouse drag
            if response.dragged_by(egui::PointerButton::Primary) {
                if let Some(ref mut fimg) = self.fractal_image {
                    let drag_delta = response.drag_delta();
                    let viewport = fimg.viewport();

                    // Calculate new center based on drag
                    let new_center = Complex64::new(
                        viewport.center.re - drag_delta.x as f64 * viewport.scale,
                        viewport.center.im - drag_delta.y as f64 * viewport.scale,
                    );

                    let new_viewport = Viewport {
                        center: new_center,
                        scale: viewport.scale,
                    };

                    fimg.pan(&self.mandelbrot, new_viewport);
                }
            }

            // Handle zooming with scroll wheel
            let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
            if scroll_delta.abs() > 0.1 {
                if let Some(ref mut fimg) = self.fractal_image {
                    let viewport = fimg.viewport();

                    // Zoom factor: positive scroll = zoom in, negative = zoom out
                    let zoom_factor = if scroll_delta > 0.0 { 0.9 } else { 1.1 };

                    // Get mouse position for zoom center
                    let zoom_center = if let Some(mouse_pos) = response.hover_pos() {
                        // Zoom towards mouse position
                        viewport.px_to_cmplx(
                            mouse_pos.x as usize,
                            mouse_pos.y as usize,
                            fimg.width(),
                            fimg.height(),
                        )
                    } else {
                        // Zoom towards viewport center
                        viewport.center
                    };

                    // Calculate new viewport
                    let new_scale = viewport.scale * zoom_factor;
                    let new_center = Complex64::new(
                        zoom_center.re + (viewport.center.re - zoom_center.re) * zoom_factor,
                        zoom_center.im + (viewport.center.im - zoom_center.im) * zoom_factor,
                    );

                    let new_viewport = Viewport {
                        center: new_center,
                        scale: new_scale,
                    };

                    fimg.zoom(&self.mandelbrot, new_viewport);
                }
            }

            // Update texture if fractal image changed
            if let Some(ref fimg) = self.fractal_image {
                let image = ColorImage {
                    size: [fimg.width(), fimg.height()],
                    pixels: fimg.pixels().to_vec(),
                };

                self.texture = Some(ctx.load_texture("fractal", image, TextureOptions::NEAREST));
            }

            // Display the texture
            if let Some(texture) = &self.texture {
                ui.painter().image(
                    texture.id(),
                    egui::Rect::from_min_size(response.rect.min, available_size),
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}
