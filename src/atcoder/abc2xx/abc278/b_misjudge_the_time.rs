// https://atcoder.jp/contests/abc278/tasks/abc278_b

fn check(h: usize, m: usize) -> bool {
    let a = h / 10;
    let b = h % 10;
    let c = m / 10;
    let d = m % 10;

    if a*10+c <= 23 && b*10+d <= 59 {
        true
    } else {
        false
    }
}

pub fn run(h: usize, m: usize) -> (usize, usize) {
    let mut i = h;
    let mut j = m;

    loop {
        if check(i, j) == true {
            return (i, j);
        }

        j += 1;

        if j == 60 {
            i += 1;
            j = 0;
        }

        if i == 24 {
            i = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (usize, usize));

    #[test]
    fn test() {
        let tests= [
            TestCase(5, 23, (5, 23)),
            TestCase(19, 57, (20, 0)),
            TestCase(20, 40, (21, 0)),
            TestCase(5, 41, (5, 41)),
            TestCase(23, 49, (0, 0)),
            TestCase(16, 20, (20, 0)),
            TestCase(21, 42, (22, 0)),
            TestCase(23, 40, (0, 0)),
            TestCase(3, 48, (3, 48)),
        ];

        for TestCase(h, m, expected) in tests {
            assert_eq!(run(h, m), expected);
        }
    }
}
