// https://atcoder.jp/contests/abc171/tasks/abc171_b

pub fn run(_n: usize, k: usize, p: Vec<usize>) -> usize {
    let mut vec = p.clone();

    vec.sort();

    (0..k)
        .map(|i| {
            vec[i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(210, run(5, 3, vec![50, 100, 80, 120, 80]));
        assert_eq!(1000, run(1, 1, vec![1000]));
    }
}
