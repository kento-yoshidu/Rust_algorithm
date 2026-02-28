// https://atcoder.jp/contests/abc280/tasks/abc280_b

pub fn run(n: usize, s: &Vec<isize>) -> Vec<isize> {
    let mut total = vec![s[0]];

    for i in 0..n-1 {
        total.push(s[i+1] - s[i]);
    }

    total
}

fn run2(_n: usize, s: &Vec<isize>) -> Vec<isize> {
    let mut ans = Vec::<isize>::new();

    for (_i, num) in s.into_iter().enumerate() {
        if ans.len() == 0 {
            ans.push(*num);
            continue;
        }

        let total: isize = ans.iter().sum();

        ans.push(num - total);
    }

    ans
}

fn run3(_n: usize, s: &Vec<isize>) -> Vec<isize> {
    let mut ans = vec![s[0]];

    for v in s.windows(2) {
        ans.push(v[1] - v[0]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>);

    #[test]
    fn abc280_b() {
        let tests = [
            TestCase(3, vec![3, 4, 8], vec![3, 1, 4]),
            TestCase(10, vec![314159265, 358979323, 846264338, -327950288, 419716939, -937510582, 97494459, 230781640, 628620899, -862803482], vec![314159265, 44820058, 487285015, -1174214626, 747667227, -1357227521, 1035005041, 133287181, 397839259, -1491424381]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, &s), expected);
            assert_eq!(run2(n, &s), expected);
            assert_eq!(run3(n, &s), expected);
        }
    }
}
