use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(vec: Vec<&str>) -> String {
    let set: HashSet<&&str> = vec.iter().collect();

    if set.len() == 4 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(vec!["3B", "HR", "2B", "H"]));
        assert_eq!(String::from("No"), run(vec!["2B", "3B", "HR", "3B"]));
    }
}
