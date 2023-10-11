// https://atcoder.jp/contests/abc219/tasks/abc219_a

pub fn run(x: usize) -> String {
    match x {
        0..=39 => (40 - x).to_string(),
        40..=69 => (70 - x).to_string(),
        70..=89 => (90 - x).to_string(),
        _ => String::from("expert"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("14"), run(56));
        assert_eq!(String::from("8"), run(32));
        assert_eq!(String::from("40"), run(0));
        assert_eq!(String::from("expert"), run(100));
    }
}
