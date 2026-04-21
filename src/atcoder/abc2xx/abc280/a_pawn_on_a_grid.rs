// https://atcoder.jp/contests/abc280/tasks/abc280_a

fn run(str: &str) -> usize {
    let mut count = 0;

    for c in str.chars() {
        if c == '#' {
            count += 1;
        }
    }

    count
}

fn run2(str: &str) -> usize {
    str.chars().filter(|c| {
        *c == '#'
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc280_a() {
        let tests = [
            TestCase("#..........##..", 3),
            TestCase("..........", 0),
            TestCase("#.#.#....#..##.####...#..#####", 16),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
