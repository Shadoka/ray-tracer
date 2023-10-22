use super::*;

#[test]
fn test_world() {
    let w = world();
    assert_eq!(0, w.objects.len());
    assert_eq!(None, w.light_source);
}

#[test]
fn test_default_world() {
    let w = default_world();

    let expected_light = point_light(
        &point(-10.0, 10.0, -10.0),
        &color(1.0, 1.0, 1.0)
    );

    let mut expected_s1 = Sphere::shape();
    let mut m1 = material();
    m1.color = color(0.8, 1.0, 0.6);
    m1.diffuse = 0.7;
    m1.specular = 0.2;
    expected_s1.set_material(&m1);

    let mut expected_s2 = Sphere::shape();
    let scaling_s2 = scaling(0.5, 0.5, 0.5);
    expected_s2.set_transform(&scaling_s2);

    let actual_objects = w.get_objects();

    assert_eq!(2, actual_objects.len());
    assert_eq!(&expected_s1, actual_objects.get(0).unwrap());
    assert_eq!(&expected_s2, actual_objects.get(1).unwrap());
    assert_eq!(expected_light, w.get_light().unwrap())
}