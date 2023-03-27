pub fn about_equals(a: f32, b: f32, epsilon: f32) -> bool {
    if (a - b).abs() < epsilon {
        true
    } else {
        false
    }
}
