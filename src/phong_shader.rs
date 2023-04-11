use crate::{material::Material, lights::PointLight, tuple::{Tuple, color}};


pub fn lighting(material: &Material, light: &PointLight, point: &Tuple, eye_vector: &Tuple, normal_vector: &Tuple) -> Tuple {
    // combine the surface color with the lights color/intensity
    let effective_color = material.color * light.intensity;

    // find the direction of the light source
    let light_vector = (light.position - point).normalize();

    // compute the ambient contribution
    let ambient = effective_color * material.ambient;

    let mut diffuse = color(0.0, 0.0, 0.0);
    let mut specular = color(0.0, 0.0, 0.0);

    // light_dot represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface
    let light_dot_normal = light_vector.dot(normal_vector);
    if light_dot_normal >= 0.0 {
        // compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye
        let reflect_vector = -light_vector.reflect(normal_vector);
        let reflect_dot_eye = reflect_vector.dot(eye_vector);

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    return ambient + diffuse + specular;
}

#[cfg(test)]
mod phong_shader_test;