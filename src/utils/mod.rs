pub const EPSILON: f32 = 0.00005;

pub fn f32_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
}

pub fn remove_suffix(mut s: String, suffix: &str) -> String {
        if s.ends_with(suffix) {
                s.truncate(s.len() - suffix.len());
        }
        s
}