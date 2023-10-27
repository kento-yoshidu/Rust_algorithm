// https://atcoder.jp/contests/abc021/tasks/abc021_b

pub fn run(_n: usize, a: usize, b: usize, _k: usize, p: Vec<usize>) -> String {
    let mut vec = vec![a, b];

    vec.append(&mut p.clone());
    vec.sort();
    vec.dedup();

    if vec.len() == p.len() + 2 {
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
        assert_eq!(String::from("Yes"), run(5, 1, 5, 3, vec![3, 4, 2]));
        assert_eq!(String::from("No"), run(7, 1, 3, 4, vec![2, 4, 2, 7]));
        assert_eq!(String::from("No"), run(4, 1, 4, 3, vec![2, 1, 3]));
        assert_eq!(String::from("No"), run(4, 1, 4, 3, vec![2, 4, 3]));
        assert_eq!(String::from("Yes"), run(20, 1, 4, 12, vec![2, 3, 5, 7, 8, 9, 10, 11, 12, 15, 13, 14]));
    }
}
