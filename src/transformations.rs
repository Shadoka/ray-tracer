use crate::{tuple::Tuple, matrix::{Matrix4, matrix4, translation}};


pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix4 {
    let forward = (to - from).normalize();
    let up_normalized = up.normalize();
    let left = forward.cross(&up_normalized);
    let true_up = left.cross(&forward);

    let orientation_transform_values = vec!(
        left.x, left.y, left.z, 0.0,
        true_up.x, true_up.y, true_up.z, 0.0,
        -forward.x, -forward.y, -forward.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    let orientation = matrix4(&orientation_transform_values);

    return orientation * translation(-from.x, -from.y, -from.z);
}

#[cfg(test)]
mod transformations_test;