// https://atcoder.jp/contests/abc014/tasks/abc014_2

pub fn run(_n: usize, x: usize, a: &Vec<usize>) -> usize {
    if x == 0 {
        return 0;
    }

    let mut b = Vec::new();

    let mut num = x;

    loop {
        if num == 1 {
            b.push(1);
            break
        }

        b.push(num%2);
        num /= 2;
    }

    b.iter()
        .enumerate()
        .map(|(i, b)| {
            if *b == 1 {
                a[i]
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn run2(_n: usize, x: usize, a: &Vec<usize>) -> usize {
    let bit = format!("{:b}", x);

    bit.chars()
        .enumerate()
        .filter(|(_, c)| *c == '1')
        .map(|(i, _)| a[i])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, vec![1, 10, 100, 1000], 101),
            TestCase(20, 1048575, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 210),
            TestCase(4, 0, vec![1000, 1000, 1000], 0),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, &a), expected);
            assert_eq!(run2(n, x, &a), expected);
        }
    }
}
