use minifb::{Key, Scale, Window, WindowOptions};

mod canvas;
use canvas::Canvas;
mod tuples;
use tuples::{Color, Tuple, TupleOperations};

const WIDTH: usize = 900;
const HEIGHT: usize = 550;

fn main() {
  // environment
  let mut position = Tuple::new_point(0.0, 1.0, 0.0);
  let mut velocity = Tuple::new_vector(1.0, 1.8, 0.0).normalize().muls(11.25);

  let gravity = Tuple::new_vector(0.0, -0.1, 0.0);
  let wind = Tuple::new_vector(-0.01, 0.0, 0.0);

  let mut canvas = Canvas::new(WIDTH, HEIGHT);
  let white = Color::new(1.0, 1.0, 1.0);

  // minifb window
  let mut window = Window::new(
    "Ray Tracer Challenge",
    WIDTH,
    HEIGHT,
    WindowOptions {
      resize: false,
      scale: Scale::X2,
      ..WindowOptions::default()
    },
  )
  .unwrap();

  loop {
    // render framebuffer
    if window.is_open() && !window.is_key_down(Key::Escape) {
      window.update_with_buffer(&canvas.to_framebuffer()).unwrap();
    } else {
      break;
    }

    // tick
    if position.y > 0.0 {
      position = position.add(&velocity);
      velocity = velocity.add(&gravity).add(&wind);
    }

    // draw on canvas
    if position.y > 0.0 {
      let canvas_x = position.x.round() as usize;
      let canvas_y = HEIGHT - position.y.round() as usize;
      canvas.write_pixel(canvas_x, canvas_y, white);
    }
  }
}
