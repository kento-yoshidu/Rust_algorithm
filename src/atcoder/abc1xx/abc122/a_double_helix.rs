#[allow(dead_code)]
fn run(c: &str) -> &str {
    match c {
        "A" => "T",
        "T" => "A",
        "G" => "C",
        _ => "G"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("T", run("A"));
        assert_eq!("C", run("G"));
    }
}
