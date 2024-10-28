// https://atcoder.jp/contests/abc284/tasks/abc284_c

fn dfs(vec: &Vec<Vec<usize>>, visited: &mut Vec<bool>, i: usize) {
    visited[i] = true;

    for i in vec[i].iter() {
        if !visited[*i] {
            dfs(vec, visited, *i);
        }
    }
}

fn run(n: usize, _m: usize, v: Option<Vec<(usize, usize)>>) -> usize {
    let mut vec = vec![Vec::new(); n+1];

    if v == None {
        return n;
    }

    for (v1, v2) in v.unwrap() {
        vec[v1].push(v2);
        vec[v2].push(v1)
    }

    let mut visited = vec![false; n+1];

    let mut ans = 0;

    for i in 1..=n {
        if visited[i] == true {
            continue;
        }

        dfs(&vec, &mut visited, i);

        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize)>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, Some(vec![(1, 2), (1, 3), (4, 5)]), 2),
            TestCase(5, 0, None, 5),
            TestCase(4, 6, Some(vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]), 1),
        ];

        for TestCase(n, m, v, expected) in tests {
            assert_eq!(run(n, m, v), expected);
        }
    }
}
