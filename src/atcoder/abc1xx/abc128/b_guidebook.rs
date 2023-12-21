// https://atcoder.jp/contests/abc128/tasks/abc128_b

pub fn run(_n: usize, st: Vec<(&str, usize)>) -> Vec<usize> {
    let mut vec: Vec<(usize, &(&str, usize))> = st.iter()
        .enumerate()
        .map(|(i, t)| (i, t))
        .collect();

    vec.sort_by(|a, b| a.1.0.cmp(b.1.0).then(b.1.1.cmp(&a.1.1)));

    vec.iter()
        .map(|(i, _)| i + 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 4, 6, 1, 5, 2], run(6, vec![("khabarovsk", 20), ("moscow", 10), ("kazan", 50), ("kazan", 35), ("moscow", 60), ("khabarovsk", 40)]));
        assert_eq!(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], run(10, vec![("yakutsk", 10), ("yakutsk", 20), ("yakutsk", 30), ("yakutsk", 40), ("yakutsk",  50), ("yakutsk", 60), ("yakutsk", 70), ("yakutsk", 80), ("yakutsk", 90), ("yakutsk", 100)]));
    }
}
