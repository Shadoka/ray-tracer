use crate::tuple::{Tuple, color};

#[derive(Clone, Debug)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

// public static methods

pub fn material() -> Material {
    return Material { 
        color: color(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0
    }
}

// private static methods

fn equals_float(a: f64, b: f64) -> bool {
    return (a - b).abs() < 0.00001
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color
        && equals_float(self.ambient, other.ambient)
        && equals_float(self.diffuse, other.diffuse)
        && equals_float(self.specular, other.specular)
        && equals_float(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod material_test;