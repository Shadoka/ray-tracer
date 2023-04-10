use exercise_chapter5::drawCircle;

mod tuple;
mod canvas;
mod matrix;
mod ray;
mod sphere;
mod shape;
mod intersection;
mod exercise_chapter5;
mod lights;
mod material;

struct A {
  text: String
}
struct B<'a> {
  a: &'a A
}

struct B_List<'a> {
  values: Vec<B<'a>>
}

fn main() {
  // let a = A{text: String::from("test")};
  // let b = B{a: &a};
  // let b2 = B{a: &a};
  // let a2 = A{text: String::from("hallo")};
  // let b3 = B{a: &a2};

  // let mut v = Vec::new();
  // v.push(b);
  // v.push(b2);
  // v.push(b3);

  // let b_list = B_List{values: v};

  // for i in 0..b_list.values.len() {
  //   println!("current B's A-value: {}", b_list.values[i].a.text);
  // }

  //println!("text contained in a contained in b: {}", b.a.text);
  //let mut c = canvas::canvas(10, 2);
  //let color = tuple::color(1.0, 0.8, 0.6);
  //for x in 0..10 {
  //  for y in 0..2 {
  //    c.write_pixel(color, x, y);
  //  }
  //}
  //c.write_file("/Users/tigro/tmp/test.ppm");

  drawCircle();
}
