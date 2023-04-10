use crate::{tuple::{point, color}, canvas::canvas, sphere::Sphere, ray::ray, intersection::Intersections};

pub fn drawCircle() {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;

    let mut canvas = canvas(canvas_pixels as usize, canvas_pixels as usize);
    let color = color(1.0, 0.0, 0.0);
    let shape = Sphere::shape();

    for y in 0..(canvas_pixels as usize - 1) {
        let world_y = half - pixel_size * (y as f64);

        for x in 0..(canvas_pixels as usize - 1) {
            let world_x = -half + pixel_size * (x as f64);
            let wall_position = point(world_x, world_y, wall_z);

            let ray = ray(&ray_origin, &(wall_position - ray_origin).normalize());
            let intersections = ray.intersect(&shape);

            let xs = Intersections::convert_from_vector(intersections);

            match xs.hit() {
                None => (),
                Some(_hit) => canvas.write_pixel(color, x, y)
            }
        }
    }

    canvas.write_file("exercise_chapter5.ppm")
}