use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(n: usize, vec: Vec<i32>) -> String {
    let hash = HashSet::<i32>::from_iter(vec);

    if hash.len() == n {
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
        assert_eq!(String::from("Yes"), run(5, vec![3, 1, 2, 4, 5]));
        assert_eq!(String::from("No"), run(6, vec![3, 1, 4, 1, 5, 2]));
        assert_eq!(String::from("Yes"), run(3, vec![1, 2, 3]));
        assert_eq!(String::from("Yes"), run(1, vec![1]));
    }
}
