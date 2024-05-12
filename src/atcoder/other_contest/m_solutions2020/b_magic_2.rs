// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_b

fn run(abc: [usize; 3], k: usize) -> &'static str {
    let mut vec = abc.clone();

    for _ in 0..k {
        if vec[0] >= vec[1] {
            vec[1] *= 2;
        } else if vec[1] >= vec[2] {
            vec[2] *= 2;
        }
    }

    if vec[0] < vec[1] && vec[1] < vec[2] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 3], usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase([7, 2, 5], 3, "Yes"),
            TestCase([7, 4, 2], 3, "No"),
        ];

        for TestCase(abc, k, expected) in tests {
            assert_eq!(run(abc, k), expected);
        }
    }
}
