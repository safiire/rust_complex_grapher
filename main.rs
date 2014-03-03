extern crate extra;
use std::task::spawn;
use std::comm::{Port};
use std::comm::{Chan};
use color::{Color, RGB};
use netpbm::{PPM};
use viewport::{ViewPort};
use complex_math::{Sine};
mod netpbm;
mod color;
mod viewport;
mod complex_math;


enum Packet {
  Pixel(uint, uint, RGB),
  Done
}


//  PPM Writer Task
fn ppm_write_worker(port: Port<Packet>) -> () {
  let width = 1024;
  let height = 1024;

  //  Create a ppm image to store the output
  let mut ppm = PPM::new(width, height);

  loop {
    match port.recv() {
      Pixel(x, y, rgb) => {
        //  Write the pixel into the PPM 
        ppm.set_pixel(x, y, rgb);
      },
      Done => {
        //  Write the image to disk
        ppm.write_file("output.ppm");
        return;
      }
    }
  }
}


//  Main
fn main() {

  let width = 1024;
  let height = 1024;

  let (port, chan): (Port<Packet>, Chan<Packet>) = Chan::new();

  spawn(proc() { ppm_write_worker(port) });

  //  Create a viewport into our complex plane
  let viewport = ViewPort::new(width, height, -3.0, 3.0, 3.0, -3.0);

  //  Go through each output pixel
  for y in range(0, height) {
    for x in range(0, width) {

      let my_chan = chan.clone();
      spawn(proc() {
        let mut z = viewport.pixel_to_complex(x, y);

        z = z.sine();
        let rgb = z.to_rgb();

        my_chan.send(Pixel(x, y, rgb));
      });
    }
  }
  chan.send(Done);
}

