#[allow(dead_code)]
pub fn run(n: usize, vec: Vec<&str>) -> String {
    let white = vec.iter().filter(|&&color| {
        color == "white"
    }).count();

    if white > (n / 2) {
        String::from("white")
    } else {
        String::from("black")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("black"), run(1, vec!["black"]));
        assert_eq!(String::from("white"), run(1, vec!["white"]));
        assert_eq!(String::from("white"), run(5, vec!["black", "white", "white", "white", "white", "white", "black"]));
        assert_eq!(String::from("black"), run(9, vec!["black", "black", "black", "black", "black", "black", "black", "black", "black"]));
    }
}
