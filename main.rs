extern mod extra;
use color::{Color};
use netpbm::{PPM};
use viewport::{ViewPort};
use complex_math::{Sine};
mod netpbm;
mod color;
mod viewport;
mod complex_math;


//  Main
fn main() {
  let width = 1024;
  let height = 1024;

  //  Create a ppm image to store the output
  let mut ppm = PPM::new(width, height);

  //  Create a viewport into our complex plane
  let viewport = ViewPort::new(width, height, -3.0, 3.0, 3.0, -3.0);

  //  Go through each output pixel
  for y in range(0, height) {
    for x in range(0, width) {

      let mut z = viewport.pixel_to_complex(x, y);
      z = z.sine();

      let rgb = z.to_rgb();
      ppm.set_pixel(x, y, rgb);
    }
  }

  //  Write the image to disk
  ppm.write_file("output.ppm");
}

