extern mod extra;
use std::fmt;
use extra::complex::{Cmplx};


//  Trait for a color
pub trait Color {
  fn to_hex(&self) -> ~str;
  fn to_rgb(&self) -> RGB;
  fn to_hsv(&self) -> HSV;
}


//  RGB is a type of color
pub struct RGB {
  r: u8
 ,g: u8
 ,b: u8
}


//  Implementation for an RGB color
impl RGB {
  pub fn new(r: u8, g: u8, b: u8) -> RGB {
    RGB{r:r, g:g, b:b}
  }
  pub fn from_fraction(r: f64, g: f64, b: f64) -> RGB {
    let r_byte = (r * 255.0) as u8;
    let g_byte = (g * 255.0) as u8;
    let b_byte = (b * 255.0) as u8;
    RGB::new(r_byte, g_byte, b_byte)
  }
}


//  Color trait implementation for RGB
impl Color for RGB {
  fn to_hex(&self) -> ~str {
    format!("{:x}{:x}{:x}", self.r as uint, self.g as uint, self.b as uint)
  }
  fn to_rgb(&self) -> RGB {
    *self
  }
  fn to_hsv(&self) -> HSV {
    println!("This is actually not implemented");
    HSV::new(0.0, 0.4, 0.5)
  }
}


//  RGB can be printed with fmt
impl fmt::Default for RGB {
  fn fmt(obj: &RGB, f: &mut fmt::Formatter) {
    write!(f.buf, "RGB({}, {}, {})", obj.r, obj.g, obj.b)
  }
}


//  HSV is a type of color
pub struct HSV {
  h: f64 
 ,s: f64 
 ,v: f64
}


//  Implementation for an HSV color
impl HSV {
  pub fn new(h: f64, s: f64, v: f64) -> HSV {
    HSV{h:h, s:s, v:v}
  }
}


//  Color trait implementation for HSV
impl Color for HSV {
  fn to_hex(&self) -> ~str {
    self.to_rgb().to_hex()
  }
  fn to_rgb(&self) -> RGB {
    //  Given a color with H ∈ [0, 360], S ∈ [0,1] and V ∈ [0,1]
    //  First modify hue to be in degrees instead of ∈ [0,1]
    let hue_degrees = self.h * 360.0;

    //  First we find Chroma
    let chroma = self.v * self.s;

    //  Find H`
    let h_prime = hue_degrees / 60.0;

    //  Find intermediate value X
    let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs() );

    //  Find intermediate R` G` B` values
    let rgb_tmp = if 0.0 <= h_prime && h_prime < 1.0 {
      [chroma, x, 0.0]
    }else if 1.0 <= h_prime && h_prime < 2.0 {
      [x, chroma, 0.0]
    }else if 2.0 <= h_prime && h_prime < 3.0 {
      [0.0, chroma, x]
    }else if 3.0 <= h_prime && h_prime < 4.0 {
      [0.0, x, chroma]
    }else if 4.0 <= h_prime && h_prime < 5.0 {
      [x, 0.0, chroma]
    }else if 5.0 <= h_prime && h_prime < 6.0 {
      [chroma, 0.0, x]
    }else{
      //println!("I suppose this means H was undefined");
      [0.0, 0.0, 0.0]
    };

    let m = self.v - chroma;
    let rgb = rgb_tmp.map(|&component| {
      component + m
    });

    RGB::from_fraction(rgb[0], rgb[1], rgb[2])
  }
  fn to_hsv(&self) -> HSV {
    *self
  }
}


//  HSV can be printed with fmt
impl fmt::Default for HSV {
  fn fmt(obj: &HSV, f: &mut fmt::Formatter) {
    write!(f.buf, "HSV({}°, {}%, {}%)", obj.h * 360.0, obj.s * 100.0, obj.v * 100.0)
  }
}


//  Transform a Cmplx number into an RGB value
//  Easiest way to do this is probably just give Cmplx 
//  The Color Trait
//  Trait for a color
impl Color for Cmplx<f64> {
  fn to_hex(&self) -> ~str {
    self.to_rgb().to_hex()
  }
  fn to_rgb(&self) -> RGB {
    self.to_hsv().to_rgb()
  }
  fn to_hsv(&self) -> HSV {
    let e = 2.718281828459045;
    let tau = 6.283185307179586;

    let mut w = self.arg();

    while w < 0.0 {
      w += tau;
    }
    while w >= tau {
      w -= tau;
    }

    let hue = w / tau;
    
    // Use the magnitude logrithmicly into the 
    // repeating interval 0 < r < 1
    let m = self.norm();
    let mut r0 = 0.0;
    let mut r1 = 1.0;
   
    while m > r1 {
      r0 = r1;
      r1 = r1 * e;
    }
    let r = (m - r0) / (r1 - r0);
    //  This puts contour lines at e^n forall n

    //  Saturation and Value come from r
    //  p and q are complimentary distances from a contour
    let p = if r < 0.5 {
      2.0 * r
    }else{
      2.0 * (1.0 - r)
    };
    let q = 1.0 - p;

    //  Only let p and q go to zero very close to zero
    //  otherwise they should stay nearly 1
    //  This keeps contours from getting thick
    let p1 = 1.0 - q * q * q;
    let q1 = 1.0 - p * p * p;

    //  Fix saturation and value from p1 and q1
    let sat = 0.4 + 0.6 * p1;
    let val = 0.6 + 0.4 * q1;

    HSV::new(hue, sat, val)
  }
}
