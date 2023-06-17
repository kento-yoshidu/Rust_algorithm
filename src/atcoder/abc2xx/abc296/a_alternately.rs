#[allow(dead_code)]
pub fn run(n: usize, str: &str) -> &str {
    for i in 0..n - 1 {
        if str.chars().nth(i).unwrap() == str.chars().nth(i + 1).unwrap() {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run(6, "MFMFMF"));
        assert_eq!("No", run(9, "FMFMMFMFM"));
        assert_eq!("Yes", run(1, "F"));
    }
}
