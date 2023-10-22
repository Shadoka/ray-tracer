use super::*;

#[test]
fn test_world() {
    let w = world();
    assert_eq!(0, w.objects.len());
    assert_eq!(None, w.light_source);
}