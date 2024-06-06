// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_a

pub fn run(x: usize) -> usize {
    match x {
        400..=599 => {
            8
        },
        600..=799 => {
            7
        },
        800..=999 => {
            6
        },
        1000..=1199 => {
            5
        },
        1200..=1399 => {
            4
        },
        1400..=1599 => {
            3
        },
        1600..=1799 => {
            2
        },
        1800.. => {
            1
        }
        _ => {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(725, 7),
            TestCase(1600, 2),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
