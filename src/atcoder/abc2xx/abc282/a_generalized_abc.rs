// https://atcoder.jp/contests/abc282/tasks/abc282_a

pub fn run(k: usize) -> String {
    (0..k).map(|i| {
        match i % 3 {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => unreachable!(),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(3));
        assert_eq!(String::from("ABCA"), run(4));
        assert_eq!(String::from("A"), run(1));
    }
}
