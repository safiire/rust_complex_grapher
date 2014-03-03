extern crate num;
use self::num::complex::{Cmplx};

pub struct ViewPort {
  width: uint
 ,height: uint
 ,left: f64
 ,top: f64
 ,right: f64
 ,bottom: f64
}


impl ViewPort {
  pub fn new(width: uint, height: uint, left: f64, top: f64, right: f64, bottom: f64) -> ViewPort {
    ViewPort{width:width, height:height, left:left, top:top, right:right, bottom:bottom}
  }

  pub fn pixel_to_complex(&self, x: uint, y: uint) -> Cmplx<f64> {
    let half_width = self.width as f64 / 2.0;
    let half_height = self.height as f64 / 2.0;
    let dx = (self.right - self.left) / self.width as f64;
    let dy = (self.bottom - self.top) / self.height as f64;
    Cmplx::new((x as f64 - half_width) * dx, (y as f64 - half_height) * dy)
  }
}
