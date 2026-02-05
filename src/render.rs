use crate::{Viewport, fractal::Fractal};
use egui::Color32;

pub fn render<F: Fractal>(
    fractal: &F,
    viewport: Viewport,
    width: usize,
    height: usize,
    pixels: &mut [Color32],
) {
    for y in 0..height {
        for x in 0..width {
            let c = viewport.px_to_cmplx(x, y, width, height);
            let it = fractal.escape_time(c);

            pixels[y * width + x] = color(it, fractal.max_iter());
        }
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
