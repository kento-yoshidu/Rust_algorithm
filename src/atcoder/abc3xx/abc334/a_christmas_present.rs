// https://atcoder.jp/contests/abc334/tasks/abc334_a

pub fn run(b: usize, g: usize) -> String {
    if b > g {
        String::from("Bat")
    } else {
        String::from("Glove")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Bat"), run(300, 100));
        assert_eq!(String::from("Glove"), run(334, 343));
    }
}
