// https://atcoder.jp/contests/abc326/tasks/abc326_b

fn check(s: String) -> bool {
    let chars: Vec<u32> =
        s.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

    if chars[0] * chars[1] == chars[2] {
        true
    } else {
        false
    }
}

pub fn run(n: usize) -> usize {
    (n..).find(|num| {
        check(num.to_string())
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(326, run(320));
        assert_eq!(144, run(144));
        assert_eq!(600, run(526));
    }
}
