// https://atcoder.jp/contests/abc206/tasks/abc206_a

pub fn run(n: f64) -> String {
    let price = (n * 1.08).floor() as usize;

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
