use crate::tuple::{point, color};

use super::*;

#[test]
pub fn test_point_light() {
    let position = point(0.0, 0.0, 0.0);
    let intensity = color(1.0, 1.0, 1.0);
    let light = point_light(&position, &intensity);

    assert_eq!(light.position, position);
    assert_eq!(light.intensity, intensity);
}