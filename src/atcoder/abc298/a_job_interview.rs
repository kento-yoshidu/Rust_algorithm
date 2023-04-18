pub fn run(n: i32, str: &str) -> &str {
    let ok = str.contains('o');
    let reject = str.contains('x');

    if ok && !reject {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run(4, "oo--"));
        assert_eq!("No", run(3, "---"));
        assert_eq!("Yes", run(1, "o"));
        assert_eq!("No", run(100, "ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooox"));
    }
}
