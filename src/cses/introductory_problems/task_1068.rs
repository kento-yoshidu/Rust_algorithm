// https://cses.fi/problemset/task/1068

pub fn run(n: usize) -> Vec<usize> {
    let mut ans = Vec::new();

    ans.push(n);

    let mut n = n;

    loop {
        if n == 1 {
            return ans;
        } else if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }

        ans.push(n);
    }
}

pub fn run2(n: usize) -> Vec<usize> {
    std::iter::successors(Some(n), |&x| {
        if x == 1 { None }
        else if x % 2 == 0 { Some(x / 2) }
        else { Some(x * 3 + 1) }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn cses_1068() {
        let tests = [
            TestCase(3, vec![3, 10, 5, 16, 8, 4, 2, 1]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
