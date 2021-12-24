use crate::tuple::Tuple;
use crate::tuple::color;

pub struct Canvas {
  pixels: Vec<Tuple>,
  width: usize,
  height: usize
}

// public static functions

pub fn canvas(width: usize, height: usize) -> Canvas {
  let mut px = Vec::with_capacity(width * height);
  for _ in 0..(width*height) {
    px.push(color(0.0, 0.0, 0.0));
  }
  return Canvas{pixels: px, width: width, height: height};
}

// private static functions

fn scale_color(color: f64) -> u8 {
  let mut scaled = color * 255.0;
  scaled = scaled.round();
  if scaled < 0.0 {
    return 0;
  } else if scaled > 255.0 {
    return 255;
  } else {
    return scaled as u8;
  }
}

fn number_string_length(number: u8) -> usize {
  if number < 10 {
    return 1;
  } else if number >= 10 && number < 100 {
    return 2;
  } else {
    return 3;
  }
}

fn process_color(color: u8, mut current_row: String, rows: &mut Vec<String>) -> String {
  if current_row.len() + number_string_length(color) < 70 {
    current_row = format!("{}{}", current_row, color);
  } else {
    current_row.pop();
    current_row = format!("{}\n", current_row);
    rows.push(current_row);
    current_row = format!("{}", color);
  }
  return current_row;
}

fn add_separator(mut current_row: String, rows: &mut Vec<String>, px_index: usize, width: usize, after_blue: bool) -> String {
  if after_blue && (px_index + 1) % width == 0 {
    current_row = format!("{}\n", current_row);
    rows.push(current_row);
    current_row = String::from("");
  } else if current_row.len() == 69 {
    current_row = format!("{}\n", current_row);
    rows.push(current_row);
    current_row = String::from("");
  } else if current_row.len() != 0 {
    current_row = format!("{} ", current_row);
  }
  return current_row;
}

// canvas impl

impl Canvas {

  pub fn write_pixel(&mut self, color: Tuple, x: usize, y: usize) {
    self.pixels[x + y * self.width] = color;
  }

  pub fn pixel_at(&self, x: usize, y: usize) -> &Tuple {
    return &self.pixels[x + y * self.width];
  }

  pub fn create_ppm_header(&self) -> String {
    return String::from(format!("P3\n{:?} {:?}\n255", self.width, self.height));
  }

  pub fn create_ppm_data(&self) -> String {
    let mut rows: Vec<String> = Vec::new();
    let mut current_row = String::from("");
    for i in 0..(self.width * self.height) {
      let current_color = &self.pixels.get(i).unwrap();
      let red = scale_color(current_color.x);
      let green = scale_color(current_color.y);
      let blue = scale_color(current_color.z);

      current_row = process_color(red, current_row, &mut rows);
      current_row = add_separator(current_row, &mut rows, i, self.width, false);

      current_row = process_color(green, current_row, &mut rows);
      current_row = add_separator(current_row, &mut rows, i, self.width, false);

      current_row = process_color(blue, current_row, &mut rows);
      current_row = add_separator(current_row, &mut rows, i, self.width, true);
    }
    let mut data = String::from("");
    for row in rows {
      data = format!("{}{}", data, row);
    }
    return data
  }
}

#[cfg(test)]
mod canvas_test;