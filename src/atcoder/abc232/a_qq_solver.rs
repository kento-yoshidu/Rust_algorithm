pub fn run(str: &str) -> i32 {
    let ch1 = str.chars().nth(0).unwrap();
    let ch3 = str.chars().nth(2).unwrap();

    let n: i32 = ch1 as i32 - 48;
    let m: i32 = ch3 as i32 - 48;

    n * m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(21, run("3x7"));
        assert_eq!(81, run("9x9"));
        assert_eq!(1, run("1x1"));
    }
}

// https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
