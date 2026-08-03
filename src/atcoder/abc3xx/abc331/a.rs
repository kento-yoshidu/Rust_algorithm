// https://atcoder.jp/contests/abc331/tasks/abc331_a

fn run(m_: usize, d_: usize, y: usize, m: usize, d: usize) -> String {
    if m_ == m && d_ == d {
        format!("{} 1 1", y + 1)
    } else if d_ == d {
        format!("{} {} {}", y, m + 1, 1)
    } else {
        format!("{} {} {}", y, m , d+1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, &'static str);

    #[test]
    fn abc331_a() {
        let tests = [
            TestCase(12, 30, 2023, 12, 30, "2024 1 1"),
            TestCase(36, 72, 6789, 23, 45, "6789 23 46"),
            TestCase(12, 30, 2012, 6, 20, "2012 6 21"),
            TestCase(12, 30, 2012, 6, 30, "2012 7 1"),
        ];

        for TestCase(m_, d_, y, m, d, expected) in tests {
            assert_eq!(run(m_, d_, y, m, d), expected);
        }
    }
}
