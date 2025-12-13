// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ae

fn run(ax: isize, ay: isize, bx: isize, by: isize, cx: isize, cy: isize) -> f64 {
    let bax = ax as f64 - bx as f64;
    let bay = ay as f64 - by as f64;
    let bcx = cx as f64 - bx as f64;
    let bcy = cy as f64 - by as f64;
    let cax = ax as f64 - cx as f64;
    let cay = ay as f64 - cy as f64;
    let cbx = bx as f64 - cx as f64;
    let cby = by as f64 - cy as f64;

    let p = if bax * bcx + bay * bcy < 0. {
        1
    } else if cax * cbx + cay * cby < 0. {
        3
    } else {
        2
    };

    match p {
        1 => {
            (bax * bax + bay * bay).sqrt()
        },
        3 => {
            (cax * cax + cay * cay).sqrt()
        },
        _ => {
            let s = (bax * bcy - bay * bcx).abs() as f64;
            let bc_length = (bcx * bcx + bcy * bcy).sqrt();

            s / bc_length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize, isize, f64);

    #[test]
    fn ma_033() {
        let tests = [
            TestCase(0, 5, 1, 1, 3, 0, 4.123105625617661),
            TestCase(-40, -30, -50, -10, -20, -20, 15.811388300841896),
            TestCase(1000000000, 1000000000, -1000000000, -1000000000, 0, -1000000000, 2236067977.499789714813),
        ];

        for TestCase(ax, ay, bx, by, cx, cy, expected) in tests {
            assert_eq!(run(ax, ay, bx, by, cx, cy), expected);
        }
    }
}
