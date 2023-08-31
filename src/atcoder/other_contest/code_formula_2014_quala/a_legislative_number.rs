// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_a

pub fn run(n: usize) -> String {
    for i in 1..100 {
        if i*i*i == n {
            return String::from("Yes")
        }
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(8));
        assert_eq!(String::from("No"), run(24));
    }
}
