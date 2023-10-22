use crate::{shape::{Shape}, lights::{PointLight, point_light}, tuple::{point, color}, sphere::Sphere, material::material, matrix::scaling, ray::Ray, intersection::{Intersections, Intersection}};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    objects: Vec<Shape>,
    light_source: Option<PointLight>
}

// public static functions

pub fn world() -> World {
    World {
        objects: Vec::new(), 
        light_source: None
    }
}

pub fn default_world() -> World {
    let light = point_light(&point(-10.0, 10.0, -10.0), &color(1.0, 1.0, 1.0));

    let mut s1 = Sphere::shape();
    let mut m1 = material();
    m1.color = color(0.8, 1.0, 0.6);
    m1.diffuse = 0.7;
    m1.specular = 0.2;
    s1.set_material(&m1);

    let mut s2 = Sphere::shape();
    let scaling_s2 = scaling(0.5, 0.5, 0.5);
    s2.set_transform(&scaling_s2);

    let objects = vec![s1, s2];

    World {
        objects,
        light_source: Some(light)
    }
}

// object functions

impl World {
    pub fn get_objects(&self) -> Vec<Shape> {
        self.objects.clone()
    }

    pub fn get_light(&self) -> Option<PointLight> {
        match self.light_source {
            Some(light) => Some(light.clone()),
            None => None
        }
    }

    pub fn intersect(&mut self, ray: &Ray) -> Intersections {
        let mut xs: Vec<Intersection> = Vec::new();
        for shape in self.objects.iter_mut() {
            let mut obj_intersections = ray.intersect(shape);
            xs.append(&mut obj_intersections);
        }

        xs.sort_by(|a, b| b.intersection_t.total_cmp(&a.intersection_t));
        xs.reverse();
        
        Intersections::convert_from_vector(xs)
    }
}

#[cfg(test)]
mod world_test;