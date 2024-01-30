// https://atcoder.jp/contests/abc336/tasks/abc336_c

fn calc(num: usize, mut result: Vec<usize>) -> Vec<usize> {
    if num == 0 {
        result
    } else {
        result.push(num % 5);
        calc(num / 5, result)
    }
}

pub fn run(n: usize) -> usize {
    let mut vec = calc(n-1, Vec::new());

    vec.reverse();

    vec.iter()
        .fold(0, |state, num| {
            state * 10 + num * 2
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 24),
            TestCase(133, 2024),
            TestCase(31415926535, 2006628868244228),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(expected, run(n));
        }
    }
}
