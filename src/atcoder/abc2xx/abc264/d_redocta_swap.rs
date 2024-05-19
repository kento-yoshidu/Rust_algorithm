// https://atcoder.jp/contests/abc264/tasks/abc264_d

pub fn run(s: &str) -> usize {
    let mut ans = 0;

    let mut vec: Vec<usize> = s.chars()
        .map(|c| {
            match c {
                'a' => 0,
                't' => 1,
                'c' => 2,
                'o' => 3,
                'd' => 4,
                'e' => 5,
                'r' => 6,
                _ => unreachable!(),
            }
        })
        .collect();

    for i in 0..7 {
        for j in 0..7-i-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("catredo", 8),
            TestCase("atcoder", 0),
            TestCase("redocta", 21),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
