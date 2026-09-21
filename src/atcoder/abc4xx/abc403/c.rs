// https://atcoder.jp/contests/abc403/tasks/abc403_c

use std::collections::HashMap;

fn run(n: usize, _m: usize, _q: usize, query: Vec<(usize, usize, Option<usize>)>) -> Vec<&'static str> {
    let mut all = vec![false; n + 1];

    let mut map = HashMap::new();

    query.into_iter()
        .filter_map(|(i, x, y)| {
            match i {
                1 => {
                    map.entry(x).or_insert_with(Vec::new).push(y.unwrap());
                    None
                },
                2 => {
                    all[x] = true;
                    None
                },
                3 => {
                    if all[x] {
                        Some("Yes")
                    } else {
                        if map.get(&x).map_or(false, |v| v.contains(&y.unwrap())) {
                            Some("Yes")
                        } else {
                            Some("No")
                        }
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize, Option<usize>)>, Vec<&'static str>);

    #[test]
    fn abc403_c() {
        let tests = [
            TestCase(2, 3, 5, vec![(1, 1, Some(2)), (3, 1, Some(1)), (3, 1, Some(2)), (2, 2, None), (3, 2, Some(3))], vec!["No", "Yes", "Yes"]),
        ];

        for TestCase(n, m, q, query, expected) in tests {
            assert_eq!(run(n, m, q, query), expected);
        }
    }
}
