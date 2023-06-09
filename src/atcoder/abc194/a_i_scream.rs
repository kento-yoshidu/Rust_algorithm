pub fn run(a: i32, b: i32) -> String {
    if a + b >= 15 && b >= 8 {
        String::from("アイスクリーム")
    } else if a + b >= 10 && b >= 3 {
        String::from("アイスミルク")
    } else if a + b >= 3 {
        String::from("ラクトアイス")
    } else {
        String::from("氷菓")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("アイスクリーム"), run(10, 8));
        assert_eq!(String::from("ラクトアイス"), run(1, 2));
        assert_eq!(String::from("氷菓"), run(0, 0));
    }
}
