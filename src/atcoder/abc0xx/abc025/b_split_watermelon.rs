// https://atcoder.jp/contests/abc025/tasks/abc025_b

pub fn run(_n: usize, a: isize, b: isize, v: Vec<(&str, isize)>) -> (String, isize) {
    let point: isize = v.iter()
        .map(|t| {
            let distance = t.1.max(a).min(b);

            if t.0 == "East" {
                distance
            } else {
                -distance
            }
        })
        .sum();

    if point < 0 {
        (String::from("West"), -point)
    } else if 0 < point {
        (String::from("East"), point)
    } else {
        (String::from("0"), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((String::from("West"), 8), run(3, 5, 10, vec![("East", 7), ("West", 3), ("West", 11)]));
        assert_eq!((String::from("0"), 0), run(3, 3, 8, vec![("West", 6), ("East", 3), ("East", 1)]));
        assert_eq!((String::from("East"), 25), run(3, 25, 25, vec![("East", 1), ("East", 1), ("West", 1), ("East", 100), ("West", 1)]));
    }
}
