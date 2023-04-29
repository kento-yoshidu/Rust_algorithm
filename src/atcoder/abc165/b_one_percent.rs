pub fn run(input: f64) -> i32 {
    let mut year = 0;
    let mut saving = 100.0;

    while saving < input {
        saving += (saving * 0.01).floor();
        year += 1;
    }

    year
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(103.0));
        assert_eq!(3760, run(1000000000000000000.0));
        assert_eq!(1706, run(1333333333.0));
    }
}
