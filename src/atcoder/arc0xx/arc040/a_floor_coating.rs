// https://atcoder.jp/contests/arc040/tasks/arc040_a

fn run(_n: usize, s: Vec<&str>) -> &'static str {
    let (t, a) = s.into_iter()
        .flat_map(|s| s.chars())
        .fold((0, 0), |acc, c| {
            match c {
                'R' => (acc.0+1, acc.1),
                'B' => (acc.0, acc.1+1),
                _ => acc
            }
        });

    if t > a {
        "TAKAHASHI"
    } else if a > t {
        "AOKI"
    } else {
        "DRAW"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec!["R.RB", "RR.B", "BRBB", "RRB."], "TAKAHASHI"),
            TestCase(2, vec!["..", ".."], "DRAW"),
            TestCase(3, vec!["BRB", "RBR", "BRB"], "AOKI"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
