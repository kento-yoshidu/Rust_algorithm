// https://atcoder.jp/contests/abc418/tasks/abc418_b

fn run(s: &str) -> f64 {
    let mut ans: f64= 0.0;

    let s: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
        if s[i] != 't' {
            continue;
        }

        for j in i+1..s.len() {
            if s[j] != 't' {
                continue;
            }

            let str = &s[i..=j];

            if str.len() < 3 {
                continue;
            }

            let not_t = &s[i..=j].iter().filter(|c| **c != 't').count();

            ans = ans.max((str.len() as f64 - *not_t as f64 - 2.0) / (str.len() as f64 - 2.0));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, f64);

    #[test]
    fn abc418_b() {
        let tests = [
            TestCase("attitude", 0.5),
            TestCase("ottottott", 0.6666666666666666),
            TestCase("coffeecup", 0.0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
