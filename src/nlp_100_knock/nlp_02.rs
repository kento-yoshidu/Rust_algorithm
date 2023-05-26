/* 02 */
// 「パトカー」と「タクシー」を交互に並べる

pub fn run() -> String {
    let str1 = String::from("パトカー");
    let str2 = String::from("タクシー");

    let mut result = String::new();

    for i in 0..str1.chars().count() {
        result.push(str1.chars().nth(i).unwrap());
        result.push(str2.chars().nth(i).unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("パタトクカシーー"), run());
    }
}
