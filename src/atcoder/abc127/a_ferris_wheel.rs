pub fn run(age: usize, price: usize) -> usize {
    if 12 < age {
        return price
    } else if 6 <= age {
        return price / 2
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(100, run(30, 100));
        assert_eq!(50, run(12, 100));
        assert_eq!(0, run(0, 100));
    }
}
