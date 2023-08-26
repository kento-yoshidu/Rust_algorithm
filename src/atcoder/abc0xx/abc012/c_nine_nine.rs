// https://atcoder.jp/contests/abc012/tasks/abc012_3

pub fn run(n: usize) -> Vec<String> {
    let mut vec: Vec<(usize, usize)> = Vec::new();

    let sa = 2025 - n;

    for i in 1..9 {
        for j in 1..9 {
            if i*j == sa {
                vec.push((i, j))
            }
        }
    }

    vec.iter().map(|v| {
        format!("{} * {}", v.0, v.1)
    }).collect::<Vec::<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["2 * 6", "3 * 4", "4 * 3", "6 * 2"], run(2013));
        assert_eq!(vec!["1 * 1"], run(2024));
    }
}

