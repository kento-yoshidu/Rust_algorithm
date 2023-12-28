// https://atcoder.jp/contests/abc086/tasks/arc089_a

pub fn run(_n: u32, t: Vec<Vec<i32>>) -> &'static str {
    let mut pos = (0, 0);
    let mut time = 0;

    for a in t {
        // 今回移動できる時間
        let time_dif = a[0] - time;

        time = a[0];

        // どれだけのマス移動することになるか
        let dist = (pos.0 - a[1]).abs() + (pos.1 - a[2]).abs();

        // 時間が足りず移動できない場合
        if dist > time_dif {
            return "No"
        }

        println!("dist={}", dist);

        if (dist % 2 == 0 && time_dif % 2 == 0) || (dist % 2 != 0 && time_dif % 2 != 0) {
            // 座標を更新
            pos = (a[1], a[2]);
        } else {
            println!("out");
            println!("dist={}, time={}", dist, time);
            return "No"
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run(2, vec![vec![3, 1, 2], vec![6, 1, 1]]));
        assert_eq!("No", run(1, vec![vec![2, 100, 100]]));
        assert_eq!("No", run(2, vec![vec![5, 1, 1], vec![100, 1, 1]]));
        assert_eq!("Yes", run(1, vec![vec![1, 0, 1]]));
        assert_eq!("No", run(2, vec![vec![6, 3, 3], vec![8, 1, 1]]));
        assert_eq!("Yes", run(3, vec![vec![1, 1, 0], vec![2, 2, 0], vec![3, 3, 0]]));
    }
}
