/* 01 */
// 1, 3, 5, 7文字目を取り出し並べる

pub fn run() -> String {
    let str = String::from("パタトクカシーー");

    str.chars().step_by(2).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("パトカー"), run());
    }
}
