use num_complex::Complex64;

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub center: Complex64,
    pub scale: f64,
}

impl Viewport {
    #[inline]
    pub fn px_to_cmplx(&self, x: usize, y: usize, w: usize, h: usize) -> Complex64 {
        let re = self.center.re + (x as f64 - w as f64 / 2.0) * self.scale;
        let im = self.center.im + (y as f64 - h as f64 / 2.0) * self.scale;
        Complex64::new(re, im)
    }
}
