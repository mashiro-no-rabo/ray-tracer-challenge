use crate::tuples::Color;

type Row = Vec<Color>;

pub struct Canvas {
  pixels: Vec<Row>,
}

#[allow(dead_code)]
impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
    }
  }

  pub fn width(&self) -> usize {
    match self.pixels.len() {
      0 => 0,
      _ => self.pixels[0].len(),
    }
  }

  pub fn height(&self) -> usize {
    self.pixels.len()
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, new_color: Color) {
    self.pixels[y][x] = new_color;
  }

  pub fn pixel_at(&self, x: usize, y: usize) -> Color {
    self.pixels[y][x]
  }

  pub fn to_framebuffer(&self) -> Vec<u32> {
    let mut vec = Vec::new();

    for y in 0..self.height() {
      for x in 0..self.width() {
        vec.push(self.pixel_at(x, y).to_argb_32());
      }
    }

    vec
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use approx::*;

  #[test]
  fn new_canvas() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 20);

    let black = Color::new(0.0, 0.0, 0.0);

    for x in 0..10 {
      for y in 0..20 {
        assert_abs_diff_eq!(c.pixel_at(x, y), black);
      }
    }
  }

  #[test]
  fn write_pixel() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);

    assert_abs_diff_eq!(c.pixel_at(2, 3), red);
  }
}
