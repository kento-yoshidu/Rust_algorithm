// https://atcoder.jp/contests/abc308/tasks/abc308_b

pub fn run(n: usize, m: usize, c: Vec<&str>, d: Vec<&str>, p: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        if let Some(j) = (0..m).find(|&j| c[i] == d[j]) {
            ans += p[j + 1];
        } else {
            ans += p[0];
        }
    }

    ans
}

pub fn run2(n: usize, m: usize, c: Vec<&str>, d: Vec<&str>, p: Vec<usize>) -> usize {
    (0..n)
        .fold(0, |state, i| {
            if let Some(j) = (0..m).find(|&j| c[i] == d[j]) {
                state + p[j + 1]
            } else {
                state + p[0]
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5200, run(3, 2, vec!["red", "green", "blue"], vec!["blue", "red"], vec![800, 1600, 2800]));
        assert_eq!(21, run(3, 2, vec!["code", "queen", "atcoder"], vec!["king", "queen"], vec![10, 1, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(5200, run2(3, 2, vec!["red", "green", "blue"], vec!["blue", "red"], vec![800, 1600, 2800]));
        assert_eq!(21, run2(3, 2, vec!["code", "queen", "atcoder"], vec!["king", "queen"], vec![10, 1, 1]));
    }
}
