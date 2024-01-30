// https://atcoder.jp/contests/abc332/tasks/abc332_b

fn run(k: usize, g: usize, m: usize) -> (usize, usize) {
    (0..k)
        .fold((0, 0), |(glass, mug), _| {
            if glass == g {
                (0, mug)
            } else if mug == 0 {
                (glass, m)
            } else {
                if g < glass + mug {
                    (g, mug - (g - glass))
                } else {
                    (glass + mug, 0)
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((200, 500), run(5, 300, 500));
        assert_eq!((0, 0), run(5, 100, 200));
    }
}
