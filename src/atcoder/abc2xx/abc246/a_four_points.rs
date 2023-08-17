// https://atcoder.jp/contests/abc246/tasks/abc246_a

pub fn run(vec: [(i32, i32); 3]) -> (i32, i32) {
    vec.iter().fold((0, 0), |a, b| {
        (a.0 ^ b.0, a.1 ^ b.1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((3, -1), run([(-1, -1), (-1, 2), (3, 2)]));
        assert_eq!((-20, -40), run([(-60, -40), (-60, -80), (-20, -80)]));
    }
}

