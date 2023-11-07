// https://atcoder.jp/contests/abc117/tasks/abc117_b

pub fn run(_n: usize, l: Vec<usize>) -> String {
    let total: usize = l.iter().sum();
    let max = l.iter().max().unwrap();

    if total - max > *max {
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
        assert_eq!(String::from("Yes"), run(4, vec![3, 8, 5, 1]));
        assert_eq!(String::from("No"), run(4, vec![3, 8, 4, 1]));
        assert_eq!(String::from("No"), run(10, vec![1, 8, 10, 5, 8, 12, 34, 100, 11, 3]));
    }
}
