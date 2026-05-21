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

    struct TestCase(usize, usize, &'static str, &'static str);

    #[test]
    fn abc290_b() {
        let tests = [
            TestCase(10, 3, "oxxoxooxox", "oxxoxoxxxx"),
            TestCase(10, 2, "oxxoxooxox", "oxxoxxxxxx"),
            TestCase(10, 4, "oxxoxooxox", "oxxoxooxxx"),
        ];
    }
}
