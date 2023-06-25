#[allow(dead_code)]
pub fn run(str: &str) -> &str {
    for word in ["and", "not", "that", "the", "you"] {
        if str.contains(word) {
            return "Yes"
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run("in that case you should print yes and not no"));
        assert_eq!("No", run("in diesem fall sollten sie no und nicht yes ausgeben"));
    }
}
