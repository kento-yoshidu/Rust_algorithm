pub fn run(str: &str) -> &str {
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
        assert_eq!("Yes", run("oo--"));
        assert_eq!("No", run("---"));
        assert_eq!("Yes", run("o"));
        assert_eq!("No", run("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooox"));
    }
}
