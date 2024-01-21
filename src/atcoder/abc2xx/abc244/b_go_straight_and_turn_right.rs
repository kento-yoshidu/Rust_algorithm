// https://atcoder.jp/contests/abc244/tasks/abc244_b

pub fn run(_n: usize, t: &str) -> (isize, isize) {
    t.chars()
        // 最初はx軸のplusの方向を向いていて、原点(0, 0)にいる
        .fold(("xp", (0, 0)), |(dir, state), c| {
            match c {
                'S' => {
                    match dir {
                        "xp" => (dir, (state.0 + 1, state.1)),
                        "ym" => (dir, (state.0, state.1 - 1)),
                        "xm" => (dir, (state.0 - 1, state.1)),
                        "yp" => (dir, (state.0, state.1 + 1)),
                        _ => ("xp", state)
                    }
                },
                'R' => {
                    match dir {
                        "xp" => ("ym", state),
                        "ym" => ("xm", state),
                        "xm" => ("yp", state),
                        "yp" => ("xp", state),
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!()
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((2, -1), run(4, "SSRS"));
        assert_eq!((0, 1), run(20, "SRSRSSRSSSRSRRRRRSRR"));
    }
}
