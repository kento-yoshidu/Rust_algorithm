// https://atcoder.jp/contests/abc438/tasks/abc438_b

fn run(n: usize, m: usize, s: &str, t: &str) -> usize {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut ans = std::usize::MAX;

    for i in 0..=n-m {
        let mut count = 0;

        for j in 0..m {
            let x = s[i+j] as isize;
            let y = t[j] as isize;

            count += (x - y + 10) % 10;
        }

        ans = ans.min(count as usize);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str, usize);

    #[test]
    fn abc438_a() {
        let tests = [
            TestCase(4, 2, "2025", "91", 2),
            TestCase(3, 2, "438", "38", 0),
            TestCase(5, 5, "00000", "11111", 45),
            TestCase(8, 3, "20251227", "438", 13),
        ];

        for TestCase(n, m, s, t, expected) in tests {
            assert_eq!(run(n, m, s, t), expected);
        }
    }
}
