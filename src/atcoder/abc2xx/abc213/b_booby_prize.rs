// https://atcoder.jp/contests/abc213/tasks/abc213_b

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();
    vec.sort();

    let num = vec[vec.len()-2];

    a.iter()
        .position(|i| {
            *i == num
        })
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(6, vec![1, 123, 12345, 12, 1234, 123456]));
        assert_eq!(5, run(5, vec![3, 1, 4, 15, 9]));
    }
}
