// https://atcoder.jp/contests/abc080/tasks/abc080_b

fn calc(n: String) -> u32 {
    n.chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

pub fn run(n: usize) -> String {
    let num = calc(n.to_string()) as usize;

    if n % num == 0 {
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
        assert_eq!(String::from("Yes"), run(12));
        assert_eq!(String::from("No"), run(57));
        assert_eq!(String::from("No"), run(148));
        assert_eq!(String::from("Yes"), run(27));
    }
}
