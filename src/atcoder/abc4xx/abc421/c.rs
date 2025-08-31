// https://atcoder.jp/contests/abc421/tasks/abc421_c

pub fn run(_n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut a_positions = Vec::new();

    for (i, &c) in chars.iter().enumerate() {
        if c == 'A' {
            a_positions.push(i);
        }
    }

    let mut cost_a = 0;

    for (j, &pos) in a_positions.iter().enumerate() {
        let target = j * 2;
        cost_a += (pos as isize - target as isize).abs() as usize;
    }

    let mut b_positions = Vec::new();

    for (i, &c) in chars.iter().enumerate() {
        if c == 'B' {
            b_positions.push(i);
        }
    }

    let mut cost_b = 0;

    for (j, &pos) in b_positions.iter().enumerate() {
        let target = j * 2;
        cost_b += (pos as isize - target as isize).abs() as usize;
    }

    cost_a.min(cost_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn abc421_c() {
        let tests = [
            TestCase(3, "AABBBA", 2),
            TestCase(3, "AAABBB", 3),
            TestCase(17, "AAABABABBBABABBABABABABBAAABABABBA", 15),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
