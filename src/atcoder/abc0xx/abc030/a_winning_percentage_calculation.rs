use std::cmp::Ordering::*;

pub fn run(a: f64, b: f64, c: f64, d: f64) -> String {
    match (b / a).partial_cmp(&(d / c)) {
        Some(Greater) => String::from("TAKAHASHI"),
        Some(Less) => String::from("AOKI"),
        Some(Equal) => String::from("DRAW"),
        None => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("AOKI"), run(5.0, 2.0, 6.0, 3.0));
        assert_eq!(String::from("TAKAHASHI"), run(100.0, 80.0, 100.0, 73.0));
        assert_eq!(String::from("DRAW"), run(66.0, 30.0, 55.0, 25.0));
    }
}
