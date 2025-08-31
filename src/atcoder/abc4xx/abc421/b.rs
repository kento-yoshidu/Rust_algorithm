// https://atcoder.jp/contests/abc421/tasks/abc421_b

fn rev(x: usize) -> usize {
    let num: String = x.to_string().chars().rev().collect();

    num.parse::<usize>().unwrap()
}


fn run(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;

    for _ in 0..9 {
        let nx = y;

        y = rev(x + y);
        x = nx;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc421_b() {
        let tests = [
            TestCase(1, 1, 415),
            TestCase(3, 7, 895),
            TestCase(90701, 90204, 9560800101),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
