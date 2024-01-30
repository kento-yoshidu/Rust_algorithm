// https://atcoder.jp/contests/abc276/tasks/abc276_b

use std::collections::HashSet;

pub fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut ans = vec![HashSet::new(); n+1];

    for (a, b) in ab {
        ans[a].insert(b);
        ans[b].insert(a);
    }

    ans.iter()
        .skip(1)
        .map(|hash_set| {
            if hash_set.len() == 0 {
                vec![0]
            } else {
                let mut vec = vec![hash_set.len()];

                let mut set = hash_set.into_iter().collect::<Vec<_>>();

                set.sort();

                for i in set {
                    vec.push(*i);
                }

                vec
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![3, 2, 3, 6], vec![2, 1, 5], vec![2, 1, 6], vec![0], vec![2, 2, 6], vec![3, 1, 3, 5]], run(6, 6, vec![(3, 6), (1, 3), (5, 6), (2, 5), (1, 2), (1, 6)]));
        assert_eq!(vec![vec![4, 2, 3, 4, 5], vec![4, 1, 3, 4, 5], vec![4, 1, 2, 4, 5], vec![4, 1, 2, 3, 5], vec![4, 1, 2, 3, 4]], run(5, 10, vec![(1, 2), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5), (3, 4), (3, 5), (4, 5)]));
    }
}
