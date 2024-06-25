// https://atcoder.jp/contests/abc354/tasks/abc354_a

fn run(h: usize) -> u32 {
    let mut day : u32 = 0;
    let mut height = 0;

    loop {
        if height > h {
            return day;
        }

        height += 2_i32.pow(day) as usize;
        day += 1;
    }
}

fn calc(h: usize, height: usize, day: usize) -> usize {
    if height > h {
        day
    } else {
        calc(h, height + 2_i32.pow(day as u32) as usize, day+1)
    }
}

fn run2(h: usize) -> usize {
    calc(h, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, u32);

    #[test]
    fn test() {
        let tests = [
            TestCase(54, 6),
            TestCase(7, 4),
            TestCase(262144, 19),
        ];

        for TestCase(h, expected) in tests {
            assert_eq!(run(h), expected);
            assert_eq!(run2(h), expected.try_into().unwrap());
        }
    }
}
