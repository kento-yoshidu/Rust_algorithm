// https://atcoder.jp/contests/abc241/tasks/abc241_b

use std::collections::HashMap;

pub fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> String {
    // 各長さのパスタが何本あるかのHashMap
    let mut hash_map_a = HashMap::new();

    for num in a {
        *hash_map_a.entry(num).or_insert(0) += 1;
    }

    for num in b {
        // numの長さの麺が残り0か、そもそも無ければNoを返す
        if *hash_map_a.get(&num).unwrap_or(&0) == 0 {
            return String::from("No")
        } else {
            *hash_map_a.entry(num).or_insert(0) -= 1;
        }
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(3, 2, vec![1, 1, 3], vec![3, 1]));
        assert_eq!(String::from("No"), run(1, 1, vec![1000000000], vec![1]));
        assert_eq!(String::from("No"), run(5, 2, vec![1, 2, 3, 4, 5], vec![5, 5]));
    }
}
