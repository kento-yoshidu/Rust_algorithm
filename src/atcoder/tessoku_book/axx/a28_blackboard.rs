// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_o

fn run(_n: usize, ta: Vec<(char, isize)>) -> Vec<isize> {
    ta.into_iter()
        .scan(0, |state, (c, num)| {
            match c {
                '+' => {
                    *state = (*state + num) % 10000;
                    Some(*state)
                },
                '-' => {
                    if *state - num < 0 {
                        *state += 10000;
                    }

                    *state = (*state - num) % 10000;
                    Some(*state)
                },
                '*' => {
                    *state = *state * 100 % 10000;
                    Some(*state)
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(char, isize)>, Vec<isize>);

    #[test]
    fn tessoku_a28() {
        let tests = [
            TestCase(4, vec![('+', 57), ('+', 43), ('*', 100), ('-', 1)], vec![57, 100, 0, 9999]),
        ];

        for TestCase(n, ta, expected) in tests {
            assert_eq!(run(n, ta), expected);
        }
    }
}
