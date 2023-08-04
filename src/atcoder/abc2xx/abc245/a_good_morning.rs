// https://atcoder.jp/contests/abc245/tasks/abc245_a

pub fn run(a: usize, b: usize, c: usize, d: usize) -> String {
    if a*60 + b <= c*60 + d {
        String::from("Takahashi")
    } else {
        String::from("Aoki")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Aoki"), run(7, 0, 6, 30));
        assert_eq!(String::from("Takahashi"), run(7, 30, 7, 30));
        assert_eq!(String::from("Takahashi"), run(0, 0, 23, 59));
    }
}

