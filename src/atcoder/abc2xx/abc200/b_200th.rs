// https://atcoder.jp/contests/abc200/tasks/abc200_b

fn run(n: usize, k: usize) -> usize {
    let mut ans = n.to_string();

    for _ in 0..k {
        let num = ans.parse::<usize>().unwrap();

        if num % 200 == 0 {
            ans = (num / 200).to_string();
        } else {
            ans = ans + "200"
        }
    }

    ans.parse().unwrap()
}

fn run2(n: usize, k: usize) -> usize {
    (0..k)
        .fold(n, |state, _| {
            if state % 200 == 0 {
                state / 200
            } else {
                (state.to_string() + "200").parse().unwrap()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc200_b() {
        let tests = [
            TestCase(2021, 4, 50531),
            TestCase(40000, 2, 1),
            TestCase(8691, 20, 84875488281),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
            assert_eq!(run2(n, k), expected);
        }
    }
}
