// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_h

fn run(h: usize, w: usize, x_vec: Vec<Vec<usize>>, q: usize, a: Vec<Vec<usize>>) -> Vec<usize> {
    let mut tmp = vec![vec![0; w+1]; h+1];

    // 横方向の累積和
    for y in 1..=h {
        for x in 1..=w {
            tmp[y][x] = tmp[y][x-1] + x_vec[y-1][x-1]
        }
    }

    // 縦方向の累積和
    for y in 1..=h {
        for x in 1..=w {
            tmp[y][x] = tmp[y][x] + tmp[y-1][x]
        }
    }

    let mut p_a = Vec::<usize>::new();
    let mut p_b = Vec::<usize>::new();
    let mut p_c = Vec::<usize>::new();
    let mut p_d = Vec::<usize>::new();

    for i in a.iter() {
        p_a.push(i[0]);
        p_b.push(i[1]);
        p_c.push(i[2]);
        p_d.push(i[3]);
    }

    let mut ans = Vec::<usize>::new();

    for i in 0..q {
        ans.push(tmp[p_c[i]][p_d[i]] + tmp[p_a[i]-1][p_b[i]-1] - tmp[p_a[i]-1][p_d[i]] - tmp[p_b[i]-1][p_c[i]]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize, Vec<Vec<usize>>, Vec<usize>);

    #[test]
    fn tessoku_a08() {
        let tests = [
            TestCase(5, 5, vec![vec![2, 0, 0, 5, 1], vec![1, 0, 3, 0, 0], vec![0, 8, 5, 0, 2], vec![4, 1, 0, 0, 6], vec![0, 9, 2, 7, 0]], 2, vec![vec![2, 2, 4, 5], vec![1, 1, 5, 5]], vec![25, 56]),
        ];

        for TestCase(h, w, x_vec, q, a, expected) in tests {
            assert_eq!(run(h, w, x_vec, q, a), expected);
        }
    }
}
