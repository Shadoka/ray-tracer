use crate::shape::Shape;

use std::ops::{Index, Add};

#[derive(Clone)]
pub struct Intersection<'a> {
    pub intersection_t: f64,
    pub object: &'a Shape
}

#[derive(Clone)]
pub struct Intersections<'a> {
    values: Vec<Intersection<'a>>
}

pub fn intersection(intersection: f64, o: &Shape) -> Intersection {
    Intersection{intersection_t: intersection, object: o}
}

// trait impls for intersection

impl <'a> Add<Intersection<'a>> for Intersection<'a> {
    type Output = Intersections<'a>;

    fn add(self, rhs: Intersection<'a>) -> Self::Output {
        let mut v = Vec::new();
        v.push(self);
        v.push(rhs);
        Intersections{values: v}
    }
}

// trait impls for intersections

impl <'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl <'a> Add<Intersection<'a>> for Intersections<'a> {
    type Output = Intersections<'a>;

    fn add(self, rhs: Intersection<'a>) -> Self::Output {
        let mut result = self.clone();
        result.values.push(rhs);
        result
    }
}

// impl Intersections

impl <'a> Intersections<'_> {
    pub fn hit(&self) -> Option<&Intersection> {
        let mut nearest_intersection = f64::MAX;
        let mut result_index = 0;
        for i in 0..self.values.len() {
            if self[i].intersection_t >= 0.0 && self[i].intersection_t < nearest_intersection {
                nearest_intersection = self[i].intersection_t;
                result_index = i;
            }
        }
        if nearest_intersection == f64::MAX {
            return Option::None;
        }
        return Option::Some(&self[result_index]);
    }

    pub fn convert_from_vector(xs: Vec<Intersection>) -> Intersections {
        let mut result = Intersections{values: vec![]};
        for index in 0..xs.len() {
            result = result + xs[index].clone();
        }
        return result;
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod intersection_test;