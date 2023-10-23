use crate::{shape::Shape, ray::Ray};
use crate::tuple::Tuple;
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

pub struct IntersectionComputationData {
    pub t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool
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

    pub fn first_positive_hit_index(&self) -> i32 {
        let mut index: i32 = -1;
        let mut lowest_value = f64::MAX;
        for n in 0..self.values.len() {
            let intersection = &self.values[n];
            if intersection.intersection_t < lowest_value && intersection.intersection_t >= 0.0 {
                lowest_value = intersection.intersection_t;
                index = n as i32;
            }
        }
        return index;
    }
}

impl Intersection<'_> {
    pub fn prepare_computations(&self, ray: &Ray) -> IntersectionComputationData {
        let t = self.intersection_t;
        let object = self.object.clone();
        let point = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = object.normal_at(&point);
        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        IntersectionComputationData { t, object, point, eyev, normalv, inside }
    }
}

#[cfg(test)]
mod intersection_test;