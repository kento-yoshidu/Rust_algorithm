// https://atcoder.jp/contests/tenka1-2017-beginner/tasks/tenka1_2017_a

pub fn run(s: String) -> usize {
    s.chars()
        .filter(|i| {
            *i == '1'
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(String::from("111100")));
    }
}
