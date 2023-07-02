#[allow(dead_code)]
pub fn run(x: u8) -> u8 {
    match x {
        0 => 1,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, run(1));
        assert_eq!(1, run(0));
    }
}
