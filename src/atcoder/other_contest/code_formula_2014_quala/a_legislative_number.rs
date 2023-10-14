// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_a

pub fn run(n: usize) -> String {
    for i in 1..100 {
        if i*i*i == n {
            return String::from("Yes")
        }
    }

    String::from("No")
}

pub fn run2(n: usize) -> String {
    if (1..=100).any(|i| {
        i * i * i == n
    }) {
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
        assert_eq!(String::from("Yes"), run(8));
        assert_eq!(String::from("No"), run(24));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(8));
        assert_eq!(String::from("No"), run2(24));
    }
}
