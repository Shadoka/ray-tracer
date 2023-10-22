use crate::{shape::Shape, lights::PointLight};

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

#[cfg(test)]
mod world_test;