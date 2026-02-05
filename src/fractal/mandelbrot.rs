use crate::fractal::Fractal;
use num_complex::Complex64;

pub struct Mandelbrot {
    pub max_iter: u32,
}

impl Fractal for Mandelbrot {
    fn name(&self) -> &'static str {
        "Mandelbrot"
    }

    fn max_iter(&self) -> u32 {
        self.max_iter
    }

    #[inline(always)]
    fn escape_time(&self, c: Complex64) -> u32 {
        let mut z = Complex64::new(0.0, 0.0);

        for i in 0..self.max_iter {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return i;
            }
        }
        self.max_iter
    }
}
