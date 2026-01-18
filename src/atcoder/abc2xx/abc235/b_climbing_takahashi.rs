// https://atcoder.jp/contests/abc235/tasks/abc235_b

fn run(n: usize, h: &Vec<usize>) -> usize {
    let mut ans = h[0];

    for i in 1..n {
        if ans < h[i] {
            ans = h[i]
        } else {
            return ans;
        }
    }

    h[n-1]
}

fn run2(n: usize, h: &Vec<usize>) -> usize {
    h.windows(2)
        .find(|a| {
            a[0] >= a[1]
        })
        .map(|a| a[0])
        .unwrap_or(h[n-1])
}

fn calc(i: usize, vec: &Vec<usize>) -> usize {
    if  i == vec.len()-1 || vec[i] >= vec[i+1] {
        vec[i]
    } else {
        calc(i+1, vec)
    }
}

fn run3(_n: usize, h: &Vec<usize>) -> usize {
    calc(0, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc235_b() {
        let tests = [
            TestCase(5, vec![1, 5, 10, 4, 2], 10),
            TestCase(3, vec![100, 1000, 100000], 100000),
            TestCase(4, vec![27, 1828, 1828, 9242], 1828),
        ];

        for TestCase(i, h, expected) in tests {
            assert_eq!(run(i, &h), expected);
            assert_eq!(run2(i, &h), expected);
            assert_eq!(run3(i, &h), expected);
        }
    }
}
