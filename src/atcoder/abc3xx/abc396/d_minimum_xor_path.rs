// https://atcoder.jp/contests/abc396/tasks/abc396_d

use std::collections::HashMap;

fn dfs(
    n: usize,
    hash_map: &HashMap<usize, Vec<(usize, usize)>>,
    seen: &mut Vec<bool>,
    cur: usize,
    xor: usize,
    ans: &mut usize
) {
    seen[cur] = true;

    let Some(next) = hash_map.get(&cur) else {
        return;
    };

    for (w, next) in next {
        if *next == n {
            *ans = (*ans).min(w ^ xor);
            continue;
        }

        if seen[*next] {
            continue;
        }

        dfs(n, hash_map, seen, *next, w ^ xor, ans)
    }

    seen[cur] = false;
}

fn run(n: usize, _m: usize, uvw: Vec<(usize, usize, usize)>) -> usize {
    let mut hash_map = HashMap::new();

    for (u, v, w) in uvw {
        hash_map.entry(u).or_insert_with(|| Vec::new()).push((w, v));
        hash_map.entry(v).or_insert_with(|| Vec::new()).push((w, u));
    }

    let mut seen = vec![false; n+1];

    let mut ans = std::usize::MAX;

    dfs(n, &hash_map, &mut seen, 1, 0, &mut ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 4, vec![(1, 2, 3), (2, 4, 5), (1, 3, 4), (3, 4, 7)], 3),
            TestCase(4, 3, vec![(1, 2, 1), (2, 3, 2), (3, 4, 4)], 7),
            TestCase(7, 10, vec![(1, 2, 726259430069220777), (1, 4, 988687862609183408), (1, 5, 298079271598409137), (1, 6, 920499328385871537), (1, 7, 763940148194103497), (2, 4, 382710956291350101), (3, 4, 770341659133285654), (3, 5, 422036395078103425), (3, 6, 472678770470637382), (5, 7, 938201660808593198)], 186751192333709144),
        ];

        for TestCase(n, m, uvw, expected) in tests {
            assert_eq!(run(n, m, uvw), expected);
        }
    }
}
