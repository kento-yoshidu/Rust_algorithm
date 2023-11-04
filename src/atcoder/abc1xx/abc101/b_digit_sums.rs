// https://atcoder.jp/contests/abc101/tasks/abc101_b

fn calc(num: usize, sum: usize) -> usize {
    if num == 0 {
        sum
    } else {
        calc(num/10, sum + num % 10)
    }
}

pub fn run(n: usize) -> String {
    let num = calc(n, 0);

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
        assert_eq!(String::from("No"), run(101));
        assert_eq!(String::from("Yes"), run(999999999));
    }
}
