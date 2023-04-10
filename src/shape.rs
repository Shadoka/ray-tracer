use crate::sphere::Sphere;
use crate::matrix::Matrix4;
use crate::tuple::Tuple;

#[derive(Clone)]
pub enum Shape {
    Sphere(Sphere)
}

impl Shape {
    pub fn get_id(&self) -> &String {
        match self {
            Shape::Sphere(s) => s.get_id()
        }
    }

    pub fn get_transform(&self) -> &Matrix4 {
        match self {
            Shape::Sphere(s) => s.get_transform()
        }
    }

    pub fn set_transform(&mut self, tm: &Matrix4) {
        match self {
            Shape::Sphere(s) => s.set_transform(tm)
        }
    }

    pub fn normal_at(&self, p: &Tuple) -> Tuple {
        match self {
            Shape::Sphere(s) => s.normal_at(p)
        }
    }
}