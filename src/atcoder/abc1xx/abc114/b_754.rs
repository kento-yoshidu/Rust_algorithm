// https://atcoder.jp/contests/abc114/tasks/abc114_b

fn run(s: &str) -> usize {
    let mut ans = 1000;

    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..(chars.len() - 2) {
        let num = (chars[i] as i32 - 48) * 100 + (chars[i+1] as i32 - 48) * 10 + (chars[i+2] as i32 -48);

        ans = ans.min((753 - num).abs());
    }

    ans as usize
}

fn check(s: String) -> usize {
    (s.parse::<isize>().unwrap() - 753).abs() as usize
}

fn run2(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.windows(3)
        .map(|a| {
            check(format!("{}{}{}", a[0], a[1], a[2]))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc114_b() {
        let tests = [
            TestCase("1234567876", 34),
            TestCase("35753", 0),
            TestCase("1111111111", 642),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
