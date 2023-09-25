// https://atcoder.jp/contests/abc034/tasks/abc034_a

pub fn run(x: usize, y: usize) -> String {
    if x < y {
        String::from("Better")
    } else {
        String::from("Worse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Better"), run(12, 34));
        assert_eq!(String::from("Worse"), run(98, 56));
    }
}
