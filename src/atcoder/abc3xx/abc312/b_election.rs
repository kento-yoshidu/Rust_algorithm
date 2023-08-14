// https://atcoder.jp/contests/abc231/tasks/abc231_b

pub fn run(_n: usize, s: Vec<&str>) -> &str {
    use std::collections::HashMap;

    let mut hash = HashMap::new();

    for str in s.iter() {
        let counter = hash.entry(str).or_insert(0);
        *counter += 1;
    }

    hash.iter().max_by(|a, b|
        a.1.cmp(&b.1)
    ).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("takahashi", run(5, vec!["snuke", "snuke", "takahashi", "takahashi", "takahashi"]));
        assert_eq!("takahashi", run(5, vec!["takahashi", "takahashi", "aoki", "takahashi", "snuke"]));
        assert_eq!("a", run(1, vec!["a"]));
    }
}
