// https://atcoder.jp/contests/abc012/tasks/abc012_3

pub fn run(n: usize) -> Vec<String> {
    let mut vec: Vec<(usize, usize)> = Vec::new();

    let sa = 2025 - n;

    println!("{}", sa);

    for i in 1..=9 {
        for j in 1..=9 {
            if i*j == sa {
                vec.push((i, j))
            }
        }
    }

    vec.iter()
        .map(|v| {
            format!("{} * {}", v.0, v.1) }
        )
        .collect()
}

/*
pub fn run2(n: usize) -> Vec<String> {
    let sa = 2025 - n;

    (1..=9).combinations_with_replacement(2)
        .filter(|t| {
            t[0] * t[1] == sa
        })
        .map(|t| {
            format!("{} * {}", t[0], t[1])
        })
        .collect()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["2 * 6", "3 * 4", "4 * 3", "6 * 2"], run(2013));
        assert_eq!(vec!["1 * 1"], run(2024));
    }

    /*
    #[test]
    fn test2() {
        assert_eq!(vec!["2 * 6", "3 * 4", "4 * 3", "6 * 2"], run2(2013));
        assert_eq!(vec!["1 * 1"], run2(2024));
    }
    */
}

