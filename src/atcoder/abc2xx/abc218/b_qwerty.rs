// https://atcoder.jp/contests/abc218/tasks/abc218_b

pub fn run(vec: Vec<usize>) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    vec.iter()
        .map(|num| {
            alphabet.chars().nth(num-1).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("abcdefghijklmnopqrstuvwxyz"), run(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]));
        assert_eq!(String::from("bacdefghijklmnopqrstuvwxyz"), run(vec![2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]));
        assert_eq!(String::from("eklpyqragjdwtcbxzsnifvhmou"), run(vec![5, 11, 12, 16, 25, 17, 18, 1, 7, 10, 4, 23, 20, 3, 2, 24, 26, 19, 14, 9, 6, 22, 8, 13, 15, 21]));
    }
}

