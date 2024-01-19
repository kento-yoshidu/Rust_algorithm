// https://atcoder.jp/contests/abc336/tasks/abc336_b

pub fn run(n: usize) -> usize {
    let s = format!("{:b}", n);

    let mut ans = 0;

    for c in s.chars().rev() {
        if c == '0' {
            ans += 1;
        } else {
            break;
        }
    }

    ans
}

fn calc(mut vec: Vec<char>, count: usize) -> usize {
    if vec[0] != '0' {
        count
    } else {
        vec.remove(0);

        calc(vec, count+1)
    }
}

pub fn run2(n: usize) -> usize {
    let s = format!("{:b}", n);
    let vec: Vec<char> = s.chars().rev().collect();

    calc(vec, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2024, 3),
            TestCase(18, 1),
            TestCase(5, 0),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(expected, run(n));
            assert_eq!(expected, run2(n));
        }
    }
}
