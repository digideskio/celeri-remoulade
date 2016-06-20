#[macro_use]
extern crate ion;
extern crate luminance;
extern crate luminance_gl;
extern crate nalgebra;

use ion::window::with_window;
use std::env;

mod demo;
mod shaders;
mod parts;

const DEMO_TITLE: &'static str = "wip";

fn main() {
  let args: Vec<_> = env::args().collect();
  let (w, h, fullscreen) = if args.len() >= 3 && args.len() < 5 {
    let w = args[1].parse().expect("width is expected");
    let h = args[2].parse().expect("height is expected");
    let fullscreen = args.len() == 4 && (args[3] == "-f" || args[3] == "--fullscreen");
    (w, h, fullscreen)
  } else {
    (800, 600, false)
  };

  with_window(w, h, DEMO_TITLE, fullscreen, demo::init);
}
