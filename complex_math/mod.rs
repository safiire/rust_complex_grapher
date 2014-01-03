extern mod extra;
use std::f64::{pow};


fn factorial(n: uint) -> uint {
  let mut product = 1;
  for i in range(1, n) {
    product *= i;
  }
  product
}

pub trait Sine {
  fn sine(&self) -> Self;
  fn pow(&self, exponent: uint) -> Self;
}


impl Sine for extra::complex::Cmplx<f64> {
  fn sine(&self) -> extra::complex::Cmplx<f64> {
    let mut result = extra::complex::Cmplx::new(0.0, 0.0);

    for n in range(0, 30) {
      let m = (2 * n + 1) as uint;
      let numerator = extra::complex::Cmplx::new(pow(-1.0, n as f64), 0.0);
      let denominator = extra::complex::Cmplx::new(factorial(m) as f64, 0.0);
      let scale = self.pow(m);
      result = result + ((numerator / denominator) * scale);
    }
    result
  }

  fn pow(&self, exponent: uint) -> extra::complex::Cmplx<f64> {
    let mut result = extra::complex::Cmplx::new(1.0, 0.0);
    for _ in range(0, exponent) {
      result = result * *self
    }
    result
  }
}

