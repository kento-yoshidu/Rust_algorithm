// https://atcoder.jp/contests/abc197/tasks/abc197_b
// Refactoring 何か間違ってる気がする

fn run(h: usize, w: usize, y: usize, x: usize, vec: Vec<&str>) -> isize {
    let mut y = y;
    let mut x = x;

    let mut ans = 1;

    let map = vec
        .iter()
        .map(|m| m.chars().collect())
        .collect::<Vec<Vec<char>>>();

    x -= 1;
    y -= 1;

    let mut right = x;
    loop {
        // 端に到達していたらbreak
        if right == w-1 {
            break
        // 現在地の一つ先を見て、壁ならbreak
        } else if map[y][right+1] == '#' {
            break
        } else {
            ans += 1;
            right += 1;
        }
    }

    let mut left = x;
    loop {
        if left == 0 {
            break
        } else if map[y][left-1] == '#' {
            break;
        } else {
            ans += 1;
            left -= 1;
        }
    }

    let mut bottom = y;
    loop {
        if bottom == h-1 {
            break;
        } else if map[bottom+1][x] == '#' {
            break;
        } else {
            ans += 1;
            bottom += 1;
        }
    }

    let mut top = y;
    loop {
        if top == 0 {
            break
        } else if map[top-1][x] == '#' {
            break;
        } else {
            ans += 1;
            top -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<&'static str>, isize);

    #[test]
    fn abc197_b() {
        let tests = [
            TestCase(4, 4, 2, 2, vec!["##..", "...#", "#.#.",".#.#"], 4),
            TestCase(3, 5, 1, 4, vec!["#....", "#####", "....#"], 4),
            TestCase(5, 5, 4, 2, vec![".#..#", "#.###", "##...", "#..#.", "#.###"], 3),
        ];

        for TestCase(h, w, y, x, vec, expected) in tests {
            assert_eq!(run(h, w, y, x, vec), expected);
        }
    }
}
