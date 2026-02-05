use num_complex::Complex64;

pub trait Fractal: Send + Sync {
    fn name(&self) -> &'static str;
    fn max_iter(&self) -> u32;
    fn escape_time(&self, c: Complex64) -> u32;
}
pub mod mandelbrot;

pub use mandelbrot::Mandelbrot;
