extern mod extra;
use color::{Color};
use netpbm::{PPM};
use viewport::{ViewPort};
mod netpbm;
mod color;
mod viewport;


//  Main
fn main() {
  let width = 1024;
  let height = 1024;

  //  Create a ppm image to store the output
  let mut ppm = PPM::new(width, height);

  //  Create a viewport into our complex plane
  let viewport = ViewPort::new(width, height, -2.5, 2.5, 2.5, -2.5);

  //  Go through each output pixel
  for y in range(0, height) {
    for x in range(0, width) {

      let mut z = viewport.pixel_to_complex(x, y);
      let one = extra::complex::Cmplx::new(1.0, 0.0);
      //let i = extra::complex::Cmplx::new(0.0, 1.0);
      //z = (z * (z  * (z / one) * z * z + one) * i);

      z = z / (one - (one / z));

      let rgb = z.to_rgb();

      ppm.set_pixel(x, y, rgb);
    }
  }

  //  Write the image to disk
  ppm.write_file("output.ppm");
}

