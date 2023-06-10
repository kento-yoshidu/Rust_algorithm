#[allow(dead_code)]
pub fn run(n: f64) -> String {
    let price = (n * 1.08).floor() as i32;

    if price == 206 {
        String::from("so-so")
    } else if price > 206 {
        String::from(":(")
    } else {
        String::from("Yay!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yay!"), run(180.0));
        assert_eq!(String::from(":("), run(200.0));
        assert_eq!(String::from("so-so"), run(191.0));
    }
}
