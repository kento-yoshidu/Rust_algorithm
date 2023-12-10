// https://atcoder.jp/contests/abc209/tasks/abc209_b

pub fn run(_n: usize, x: usize, a: Vec<usize>) -> String {
    let total: usize = a.iter()
        .enumerate()
        .map(|(i, &num)| {
            if i % 2 == 0 {
                num - 1
            } else {
                num
            }
        })
        .sum();

    if total <= x {
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
        assert_eq!(String::from("Yes"), run(2, 3, vec![1, 3]));
        assert_eq!(String::from("No"), run(4, 10, vec![3, 3, 4, 4]));
        assert_eq!(String::from("Yes"), run(8, 30, vec![3, 1, 4, 1, 5, 9, 2, 6]));
    }
}
