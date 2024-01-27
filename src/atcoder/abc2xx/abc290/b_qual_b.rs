// https://atcoder.jp/contests/abc290/tasks/abc290_b

fn run(_n: usize, k: usize, s: &str) -> String {
    s.chars()
        .scan(k, |state, c| {
            if *state == 0 {
                Some('x')
            } else {
                if c == 'o' {
                    *state -= 1;
                    Some('o')
                } else {
                    Some('x')
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("oxxoxoxxxx"), run(10, 3, "oxxoxooxox"));
        assert_eq!(String::from("oxxoxxxxxx"), run(10, 2, "oxxoxooxox"));
        assert_eq!(String::from("oxxoxooxxx"), run(10, 4, "oxxoxooxox"));
    }
}
