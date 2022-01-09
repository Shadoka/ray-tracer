use crate::sphere::Sphere;

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
}