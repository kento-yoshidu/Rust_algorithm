// https://atcoder.jp/contests/abc173/tasks/abc173_b

fn run(_n: usize, s: &Vec<&str>) -> Vec<String> {
    let ans: Vec<usize> =
        ["AC", "WA", "TLE", "RE"].into_iter()
            .map(|str| {
                s.iter()
                    .filter(|str2| str == **str2)
                    .count()
            })
            .collect();

    ["AC", "WA", "TLE", "RE"].into_iter()
        .enumerate()
        .map(|(i, str)| {
            format!("{} x {}", str, ans[i])
        })
        .collect()
}

fn run2(_n: usize, s: &Vec<&str>) -> Vec<String> {
    use std::collections::HashMap;

    let mut hash_map = HashMap::new();

    // 一つも出てこないかもしれないので要素を追加しておく
    hash_map.insert("AC", 0);
    hash_map.insert("WA", 0);
    hash_map.insert("TLE", 0);
    hash_map.insert("RE", 0);

    s.into_iter()
        .for_each(|str| {
            *hash_map.entry(str).or_insert(1) += 1;
        });

    ["AC", "WA", "TLE", "RE"].into_iter()
        .map(|str| {
            format!("{} x {}", str, hash_map[str])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc173_b() {
        let tests = [
            TestCase(6, vec!["AC", "TLE", "AC", "AC", "WA", "TLE"], vec!["AC x 3", "WA x 1", "TLE x 2", "RE x 0"]),
            TestCase(10, vec!["AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC"], vec!["AC x 10", "WA x 0", "TLE x 0", "RE x 0"]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, &s), expected);
            assert_eq!(run2(n, &s), expected);
        }
    }
}
