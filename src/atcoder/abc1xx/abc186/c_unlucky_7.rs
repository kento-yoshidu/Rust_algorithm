// https://atcoder.jp/contests/abc186/tasks/abc186_c

pub fn run(n: usize) -> usize {
    (1..=n)
        .filter(|num| {
            !num.to_string().contains("7") && !format!("{:0o}", num).to_string().contains("7")
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(17, run(20));
        assert_eq!(30555, run(100000));
    }
}
