// https://atcoder.jp/contests/abc155/tasks/abc155_a

pub fn run(a: usize, b: usize, c: usize) -> String {
    let mut vec = vec![a, b, c];

    vec.sort();
    vec.dedup();

    if vec.len() == 2 {
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
        assert_eq!(String::from("Yes"), run(5, 7, 5));
        assert_eq!(String::from("No"), run(4, 4, 4));
        assert_eq!(String::from("No"), run(4, 9, 6));
        assert_eq!(String::from("Yes"), run(3, 3, 4));
    }
}
