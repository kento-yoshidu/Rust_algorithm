// https://atcoder.jp/contests/abc028/tasks/abc028_b

pub fn run(n: String) -> Vec<usize> {
    let mut ans = vec![0; 6];

    for c in n.chars() {
        ans[(c as u8 - 65) as usize] += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 1, 0, 0, 1, 1], run(String::from("BEAF")));
        assert_eq!(vec![1, 0, 1, 2, 2, 0], run(String::from("DECADE")));
        assert_eq!(vec![1, 2, 3, 4, 5, 6], run(String::from("ABBCCCDDDDEEEEEFFFFFF")));
    }
}
