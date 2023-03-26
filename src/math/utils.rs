pub fn about_equals(a: f32, b: f32, epsilon: f32) -> bool {
    if (a - b).abs() < epsilon {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn about_equals() {
        assert_eq!(super::about_equals(4.0, 4.000001, 0.00001), true)
    }
}
