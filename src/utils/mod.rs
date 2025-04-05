pub const EPSILON: f64 = 0.00001;

pub fn f64_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn remove_suffix(mut s: String, suffix: &str) -> String {
    if s.ends_with(suffix) {
        s.truncate(s.len() - suffix.len());
    }
    s
}
