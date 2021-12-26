mod tuple;
mod canvas;
mod matrix;
mod ray;

fn main() {
  let mut c = canvas::canvas(10, 2);
  let color = tuple::color(1.0, 0.8, 0.6);
  for x in 0..10 {
    for y in 0..2 {
      c.write_pixel(color, x, y);
    }
  }
  c.write_file("/Users/tigro/tmp/test.ppm");
}
