use crate::{fractal::Fractal, viewport::Viewport};
use egui::Color32;

pub struct FractalImage {
    pixels: Vec<Color32>,
    width: usize,
    height: usize,
    viewport: Viewport,
}

impl FractalImage {
    pub fn new(width: usize, height: usize, viewport: Viewport) -> Self {
        Self {
            pixels: vec![Color32::BLACK; width * height],
            width,
            height,
            viewport,
        }
    }

    pub fn pixels(&self) -> &[Color32] {
        &self.pixels
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    /// Resize the image buffer
    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.pixels.resize(width * height, Color32::BLACK);
        }
    }

    /// Render the entire fractal for the current viewport
    pub fn render<F: Fractal>(&mut self, fractal: &F, viewport: Viewport) {
        self.viewport = viewport;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.viewport.px_to_cmplx(x, y, self.width, self.height);
                let it = fractal.escape_time(c);
                self.pixels[y * self.width + x] = color(it, fractal.max_iter());
            }
        }
    }

    /// Update with panning - reuse existing pixels where possible
    pub fn pan<F: Fractal>(&mut self, fractal: &F, new_viewport: Viewport) {
        // Calculate pixel offset between old and new viewport
        let old_center_px = self
            .viewport
            .cmplx_to_px(new_viewport.center, self.width, self.height);
        let new_center_px = (self.width as f64 / 2.0, self.height as f64 / 2.0);

        let dx = (new_center_px.0 - old_center_px.0).round() as isize;
        let dy = (new_center_px.1 - old_center_px.1).round() as isize;

        // If the offset is too large or scales differ, just re-render everything
        if dx.abs() >= self.width as isize
            || dy.abs() >= self.height as isize
            || (self.viewport.scale - new_viewport.scale).abs() > 1e-10
        {
            self.render(fractal, new_viewport);
            return;
        }

        // Create a new buffer for the shifted image
        let mut new_pixels = vec![Color32::BLACK; self.width * self.height];

        // Copy existing pixels to their new positions
        for y in 0..self.height {
            for x in 0..self.width {
                let old_x = x as isize - dx;
                let old_y = y as isize - dy;

                if old_x >= 0
                    && old_x < self.width as isize
                    && old_y >= 0
                    && old_y < self.height as isize
                {
                    let old_idx = old_y as usize * self.width + old_x as usize;
                    let new_idx = y * self.width + x;
                    new_pixels[new_idx] = self.pixels[old_idx];
                }
            }
        }

        self.pixels = new_pixels;
        self.viewport = new_viewport;

        // Render the newly exposed areas
        self.render_exposed_regions(fractal, dx, dy);
    }

    /// Render only the regions that were newly exposed by panning
    fn render_exposed_regions<F: Fractal>(&mut self, fractal: &F, dx: isize, dy: isize) {
        // Render left/right strips
        // dx > 0 means pixels shifted right, so left edge is newly exposed
        // dx < 0 means pixels shifted left, so right edge is newly exposed
        if dx > 0 {
            // Pixels shifted right, new content on the left
            for y in 0..self.height {
                for x in 0..(dx as usize) {
                    self.render_pixel(fractal, x, y);
                }
            }
        } else if dx < 0 {
            // Pixels shifted left, new content on the right
            for y in 0..self.height {
                for x in (self.width - (-dx) as usize)..self.width {
                    self.render_pixel(fractal, x, y);
                }
            }
        }

        // Render top/bottom strips (excluding corners already rendered)
        // dy > 0 means pixels shifted down, so top edge is newly exposed
        // dy < 0 means pixels shifted up, so bottom edge is newly exposed
        if dy > 0 {
            // Pixels shifted down, new content on the top
            let start_x = if dx > 0 { dx as usize } else { 0 };
            let end_x = if dx < 0 {
                self.width - (-dx) as usize
            } else {
                self.width
            };
            for y in 0..(dy as usize) {
                for x in start_x..end_x {
                    self.render_pixel(fractal, x, y);
                }
            }
        } else if dy < 0 {
            // Pixels shifted up, new content on the bottom
            let start_x = if dx > 0 { dx as usize } else { 0 };
            let end_x = if dx < 0 {
                self.width - (-dx) as usize
            } else {
                self.width
            };
            for y in (self.height - (-dy) as usize)..self.height {
                for x in start_x..end_x {
                    self.render_pixel(fractal, x, y);
                }
            }
        }
    }

    /// Render a single pixel
    #[inline]
    fn render_pixel<F: Fractal>(&mut self, fractal: &F, x: usize, y: usize) {
        let c = self.viewport.px_to_cmplx(x, y, self.width, self.height);
        let it = fractal.escape_time(c);
        self.pixels[y * self.width + x] = color(it, fractal.max_iter());
    }

    /// Update with zooming - for now, just re-render
    /// Could be optimized with bilinear interpolation in the future
    pub fn zoom<F: Fractal>(&mut self, fractal: &F, new_viewport: Viewport) {
        self.render(fractal, new_viewport);
    }
}

#[inline]
fn color(iter: u32, max: u32) -> Color32 {
    if iter == max {
        Color32::BLACK
    } else {
        let v = (255 * iter / max) as u8;
        Color32::from_rgb(v, v, 255)
    }
}
