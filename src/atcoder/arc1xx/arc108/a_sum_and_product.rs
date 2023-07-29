// https://atcoder.jp/contests/arc108/tasks/arc108_a

#[allow(dead_code)]
pub fn run(s: usize, p: usize) -> String {
    for i in 1..=p {
        if i*i > p {
            break;
        }

        if p % i == 0 && i + p/i == s {
            return String::from("Yes");
        }
    }

    return String::from("No");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(3, 2));
        assert_eq!(String::from("No"), run(1000000000000, 1));
    }
}
