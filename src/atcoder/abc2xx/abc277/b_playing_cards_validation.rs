// https://atcoder.jp/contests/abc277/tasks/abc277_b

use itertools::Itertools;

fn run(_n: usize, s: Vec<&str>) -> &'static str {
    if !s.iter().all_unique() {
        return "No";
    }

    let vec: Vec<Vec<char>> =
        s.iter()
            .map(|str| str.chars().collect::<Vec<char>>())
            .collect();

    if vec.into_iter()
        .all(|v| {
            ['H', 'D', 'C', 'S'].contains(&v[0])
            &&
            ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'].contains(&v[1])
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc277_b() {
        let tests = [
            TestCase(4, vec!["H3", "DA", "D3", "SK"], "Yes"),
            TestCase(5, vec!["H3", "DA", "CK", "H3", "S7"], "No"),
            TestCase(4, vec!["3H", "AD", "3D", "KS"], "No"),
            TestCase(5, vec!["00", "AA", "XX", "YY", "ZZ"], "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
