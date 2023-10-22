use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Tuple,
    pub position: Tuple
}

pub fn point_light(position: &Tuple, intensity: &Tuple) -> PointLight {
    return PointLight { intensity: intensity.clone(), position: position.clone() };
}

#[cfg(test)]
mod lights_test;