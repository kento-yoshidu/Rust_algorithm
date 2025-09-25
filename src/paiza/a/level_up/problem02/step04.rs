// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_move_step4

fn m(dir: char, pos: &mut (isize, isize)) {
    match dir {
        'N' => pos.1 -= 1,
        'E' => pos.0 += 1,
        'S' => pos.1 += 1,
        'W' => pos.0 -= 1,
        _ => unreachable!(),
    }
}

fn run(x: isize, y: isize, n: usize) -> (isize, isize) {
    let dir = ['E', 'S', 'W', 'N'];
    let mut now_dir = 0;
    let mut count = 0;
    let mut max_count = 1;
    let mut is_first = true;

    let mut pos = (x, y);

    for _ in 0..n {
        m(dir[now_dir], &mut pos);

        count += 1;

        if is_first && count == max_count {
            is_first = false;
            count = 0;
            now_dir = (now_dir+1) % 4;
        } else if count == max_count {
            is_first = true;
            count = 0;
            max_count += 1;
            now_dir = (now_dir+1) % 4;
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, usize, (isize, isize));

    #[test]
    fn paiza_a_level_up_problem02_step04() {
        let tests = [
            TestCase(0, 0, 3, (0, 1)),
            TestCase(38, 47, 27, (41, 47)),
        ];

        for TestCase(x, y, n, expected) in tests {
            assert_eq!(run(x, y, n), expected);
        }
    }
}
