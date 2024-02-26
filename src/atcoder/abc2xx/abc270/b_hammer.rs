// https://atcoder.jp/contests/abc270/tasks/abc270_b

pub fn run(x: isize, y: isize, z: isize) -> isize {
    let mut goal = x;
    let mut wall = y;
    let mut hammer = z;

    if y < 0 {
        goal = -goal;
        wall = -wall;
        hammer = -hammer;
    }

    if goal < wall {
        goal.abs()
    } else if wall < goal && wall < hammer {
        -1
    } else {
        hammer.abs() + (goal - hammer).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, -10, 1, 10),
            TestCase(20, 10, -10, 40),
            TestCase(100, 1, 1000, -1),
            TestCase(76, 334, 360, 76),
            TestCase(76, 334, 360, 76),
            TestCase(11, 712, 694, 11),
            TestCase(607, 121, 277, -1),
        ];

        for TestCase(x, y, z, expected) in tests {
            assert_eq!(run(x, y, z), expected);
        }
    }
}
