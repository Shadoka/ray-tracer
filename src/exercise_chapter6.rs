use crate::{tuple::{point, color}, canvas::canvas, sphere::Sphere, ray::ray, intersection::Intersections, material::material, phong_shader::lighting, matrix::scaling};
use crate::lights::point_light;

pub fn draw_circle() {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 300.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;

    let mut canvas = canvas(canvas_pixels as usize, canvas_pixels as usize);

    let mut shape = Sphere::shape();
    let mut material = material();
    material.color = color(1.0, 0.2, 1.0);
    shape.set_material(&material);
    shape.set_transform(&scaling(1.0, 0.5, 1.0));

    let light_position = point(-10.0, 10.0, -10.0);
    let light_color = color(1.0, 1.0, 1.0);
    let light = point_light(&light_position, &light_color);

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
                Some(hit) => {
                    let point = ray.position(hit.intersection_t);
                    let normal = hit.object.normal_at(&point);
                    let eye_vector = -ray.direction;
                    let pixel_color = lighting(hit.object.get_material(), &light, &point, &eye_vector, &normal);
                    canvas.write_pixel(pixel_color, x, y);
                }
            }
        }
    }

    canvas.write_file("exercise_chapter6_scaled.ppm")
}