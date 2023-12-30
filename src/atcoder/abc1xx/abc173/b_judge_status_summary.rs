// https://atcoder.jp/contests/abc173/tasks/abc173_b

pub fn run(_n: usize, s: Vec<&str>) -> Vec<String> {
    let ans: Vec<usize> =
        ["AC", "WA", "TLE", "RE"].iter()
            .map(|str| {
                s.iter()
                    .filter(|str2| *str == **str2)
                    .count()
            })
            .collect();

    ["AC", "WA", "TLE", "RE"].iter()
        .enumerate()
        .map(|(i, str)| {
            format!("{} x {}", str, ans[i])
        })
        .collect()
}

pub fn run2(_n: usize, s: Vec<&str>) -> Vec<String> {
    use std::collections::HashMap;

    let mut hash_map = HashMap::new();

    // 一つも出てこないかもしれないので要素を追加しておく
    hash_map.insert("AC", 0);
    hash_map.insert("WA", 0);
    hash_map.insert("TLE", 0);
    hash_map.insert("RE", 0);

    s.iter()
        .for_each(|str| {
            *hash_map.entry(str).or_insert(1) += 1;
        });

    ["AC", "WA", "TLE", "RE"].iter()
        .map(|str| {
            format!("{} x {}", str, hash_map[str])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("AC x 3"), String::from("WA x 1"), String::from("TLE x 2"), String::from("RE x 0")], run(6, vec!["AC", "TLE", "AC", "AC", "WA", "TLE"]));
        assert_eq!(vec![String::from("AC x 10"), String::from("WA x 0"), String::from("TLE x 0"), String::from("RE x 0")], run(10, vec!["AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC"]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![String::from("AC x 3"), String::from("WA x 1"), String::from("TLE x 2"), String::from("RE x 0")], run2(6, vec!["AC", "TLE", "AC", "AC", "WA", "TLE"]));
        assert_eq!(vec![String::from("AC x 10"), String::from("WA x 0"), String::from("TLE x 0"), String::from("RE x 0")], run2(10, vec!["AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC", "AC"]));
    }
}
