/* 00 */
// 文字列 stressedを逆に並べる
#[allow(dead_code)]
pub fn run() -> String {
    let str = String::from("stressed");

    str.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("desserts"), run());
    }
}
