#[allow(dead_code)]
pub fn run(d: f64, t: f64, s: f64) -> String {
    if d <= s * t {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(1000_f64, 15_f64, 80_f64));
        assert_eq!(String::from("Yes"), run(2000_f64, 20_f64, 100_f64));
        assert_eq!(String::from("No"), run(10000_f64, 1_f64, 1_f64));
    }
}
