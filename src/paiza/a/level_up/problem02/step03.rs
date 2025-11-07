// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_move_step3

fn run(y: isize, x: isize, d: char, dd: char) -> (isize, isize) {
    let mut x = x;
    let mut y = y;

    let dis = if dd == 'L' {
        -1
    } else {
        1
    };

    match d {
        'N' => x += dis,
        'S' => x -= dis,
        'E' => y += dis,
        'W' => y -= dis,
        _ => unreachable!(),
    }

    (y ,x)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, char, char, (isize, isize));

    #[test]
    fn paiza_a_level_up_problem02_step03() {
        let tests = [
            TestCase(4, 2, 'N', 'R', (4, 3)),
            TestCase(6, 9, 'E', 'R', (7, 9)),
        ];

        for TestCase(y, x, d, dd, expected) in tests {
            assert_eq!(run(y, x, d, dd), expected);
        }
    }
}
