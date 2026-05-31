// https://atcoder.jp/contests/abc326/tasks/abc326_b

fn check(s: String) -> bool {
    let chars: Vec<u32> =
        s.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

    if chars[0] * chars[1] == chars[2] {
        true
    } else {
        false
    }
}

fn run(n: usize) -> usize {
    (n..)
        .find(|num| {
            check(num.to_string())
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc326_b() {
        let tests = [
            TestCase(320, 326),
            TestCase(144, 144),
            TestCase(526, 600),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
