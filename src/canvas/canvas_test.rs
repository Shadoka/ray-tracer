use super::*;

#[test]
fn test_create_canvas() {
  let c = canvas(10, 20);
  assert_eq!(c.width, 10);
  assert_eq!(c.height, 20);
  for pixel in &c.pixels {
    assert_eq!(pixel, &color(0.0, 0.0, 0.0));
  }
}

#[test]
fn test_write_pixel() {
  let mut c = canvas(10, 20);
  let red = color(1.0, 0., 0.);
  
  c.write_pixel(red, 0, 0);
  assert_eq!(c.pixel_at(0, 0), &red);

  c.write_pixel(red, 9, 19);
  assert_eq!(c.pixel_at(9, 19), &red)
}

#[test]
fn test_create_ppm_header() {
  let c = canvas(10, 20);
  assert_eq!(c.create_ppm_header(), String::from("P3\n10 20\n255"));
}

#[test]
fn test_create_ppm_data() {
  let mut c = canvas(5, 3);
  c.write_pixel(color(1.5, 0.0, 0.0), 0, 0);
  c.write_pixel(color(0.0, 0.5, 0.0), 2, 1);
  c.write_pixel(color(-0.5, 0.0, 1.0), 4, 2);

  let expected = r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#;

  assert_eq!(c.create_ppm_data(), expected);
}

#[test]
fn test_format() {
  let mut s = String::from("");
  s = format!("{}{}", s, 255);
  assert_eq!(s, String::from("255"));
}