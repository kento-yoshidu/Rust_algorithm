// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_move_step2

fn run(y: isize, x: isize, _n: usize, d: Vec<char>) -> Vec<(isize, isize)> {
    let mut ans = Vec::new();

    let mut pos = (y, x);

    for c in d {
        match c {
            'N' => pos = (pos.0 - 1, pos.1),
            'S' => pos = (pos.0 + 1, pos.1),
            'E' => pos = (pos.0, pos.1 + 1),
            'W' => pos = (pos.0, pos.1 - 1),
            _ => unreachable!(),
        }

        ans.push(pos);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, usize, Vec<char>, Vec<(isize, isize)>);

    #[test]
    fn paiza_a_level_up_problem02_step02() {
        let tests = [
            TestCase(0, 0, 1, vec!['N'], vec![(-1, 0)]),
            TestCase(5, 10, 4, vec!['N', 'W', 'E', 'S'], vec![(4, 10), (4, 9), (4, 10), (5, 10)]),
        ];

        for TestCase(y, x, n, d, expected) in tests {
            assert_eq!(run(y, x, n, d), expected);
        }
    }
}
