#[allow(dead_code)]
pub fn run(s: String) -> String {
    let temp: Vec<_> = s.split("/").collect();

    if temp[1].parse::<i32>().unwrap() > 4 {
        String::from("TBD")
    } else {
        String::from("Heisei")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Heisei"), run(String::from("2019/04/30")));
        assert_eq!(String::from("TBD"), run(String::from("2019/11/30")));
    }
}
