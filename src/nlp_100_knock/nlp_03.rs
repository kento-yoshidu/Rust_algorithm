/* 03 */
// 各単語の文字数をリスト化する
#[allow(dead_code)]
pub fn run() -> Vec<usize> {
    let str1 = String::from("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.");

    str1.replace(",", "")
        .replace(".", "")
        .split(" ")
        .map(|s| s.len())
        .collect()
}

#[allow(dead_code)]
pub fn run2() -> Vec<usize> {
    let str1 = String::from("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.");

    str1.split(&[',', '.', ' '] as &[char])
        .filter(|s| s.len() != 0)
        .map(|s| s.len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], run());
    }

    #[test]
    fn test2() {
        assert_eq!(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], run2());
    }
}
