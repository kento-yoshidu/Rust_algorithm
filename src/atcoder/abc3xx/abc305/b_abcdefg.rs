use std::collections::HashMap;

#[allow(dead_code)]
fn run(p: &str, q: &str) -> i32 {
    let distance = HashMap::from([
        ("A", 0),
        ("B", 3),
        ("C", 4),
        ("D", 8),
        ("E", 9),
        ("F", 14),
        ("G", 23),
    ]);

    let difference: i32 = distance[p] - distance[q];

    difference.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run("A", "C"));
        assert_eq!(20, run("G", "B"));
        assert_eq!(10, run("C", "F"));
    }
}

// https://stackoverflow.com/questions/73501732/why-does-method-call-on-ambiguous-numeric-cause-error
