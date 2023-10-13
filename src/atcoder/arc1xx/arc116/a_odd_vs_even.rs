// https://atcoder.jp/contests/arc116/tasks/arc116_a

pub fn run(_t: usize, vec: Vec<usize>) -> Vec<&'static str> {
    vec.iter().map(|v| {
        if v % 4 == 1 || v % 4 == 3 {
            "Odd"
        } else if v % 4 == 0 {
            "Even"
        } else {
            "Same"
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["Same", "Odd", "Even"], run(3, vec![2, 998244353, 1000000000000000000]));
    }
}
