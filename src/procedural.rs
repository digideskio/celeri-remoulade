use std::f32::consts::PI;

pub fn noise2(x: f32, y: f32) -> f32 {
  let d = (x * 12.9898 + y * 78.233) * 43758.5453;
  return f32::fract(f32::sin(d));
}

pub fn color_palette(a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3], t: f32) -> [f32; 3] {
  let mut r: [f32; 3] = [0., 0., 0.];

  for i in 0..3 {
    r[i] = a[i] + b[i] * f32::cos(2. * PI * (c[i] * t + d[i]));
  }

  r
}

// A Gaussian.
pub fn gaussian(m: f32, v: f32, x: f32) -> f32 {
  f32::exp(-f32::powf(x - m, 2.) / f32::powf(2. * v, 2.)) / (v * f32::sqrt(2. * PI))
}
