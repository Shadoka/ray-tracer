use crate::shape::Shape;

pub struct Intersection<'a> {
    pub intersections: Vec<f64>,
    pub object: &'a dyn Shape
}

pub fn intersection(intersections: Vec<f64>, o:  &dyn Shape) -> Intersection {
    Intersection{intersections, object: o}
}