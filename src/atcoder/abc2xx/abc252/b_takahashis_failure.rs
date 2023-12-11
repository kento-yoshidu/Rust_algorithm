// https://atcoder.jp/contests/abc252/tasks/abc252_b

fn run(_n: usize, _k: usize, a: Vec<usize>, b: Vec<usize>) -> String {
    let max = a.iter().max().unwrap();

    if b.iter()
        .any(|i| {
            a[i-1] == *max
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"),run(5, 3, vec![6, 8, 10, 7, 10], vec![2, 3, 4]));
        assert_eq!(String::from("No"),run(5, 2, vec![100, 100, 100, 1, 1], vec![5, 4]));
        assert_eq!(String::from("No"),run(2, 1, vec![100, 1], vec![2]));
    }
}
