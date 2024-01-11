// https://atcoder.jp/contests/abc188/tasks/abc188_b

pub fn run(_n: usize, a: Vec<isize>, b: Vec<isize>) -> String {
    let total: isize = a.iter()
        .zip(b.iter())
        .map(|t| t.0 * t.1 )
        .sum();

    if total == 0 {
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
        assert_eq!(String::from("Yes"), run(2, vec![-3, 6], vec![4, 2]));
        assert_eq!(String::from("No"), run(2, vec![4, 5], vec![-1, -3]));
        assert_eq!(String::from("Yes"), run(3, vec![1, 3, 5], vec![3, -6, 3]));
    }
}
