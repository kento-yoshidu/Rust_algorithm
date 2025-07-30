// https://atcoder.jp/contests/abc415/tasks/abc415_b

fn run(s: &str) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '#' {
            ans.push(i+1);
        }
    }

    ans.chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, Vec<(usize, usize)>);

    #[test]
    fn abc415_b() {
        let tests = [
            TestCase(".#.##..##.#.###....#", vec![(2, 4), (5, 8), (9, 11), (13, 14), (15, 20)]),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
