// https://atcoder.jp/contests/joigsp2023/tasks/joigsp2023_a

// Refactoring
pub fn run(_n: usize, _q: usize, vec: Vec<(isize, isize)>, t_vec: Vec<isize>) -> Vec<isize> {
    let mut ans = Vec::new();

    for t in t_vec {
        let mut max_h = 0;

        for tup in &vec {
            let height;
            let mut rest = t;

            rest -= tup.0;

            if rest <= 0 {
                height = 0
            } else if rest < tup.1 {
                height = rest
            } else {
                height = tup.1
            }

            max_h = max_h.max(height);
        }

        ans.push(max_h);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 0, 4], run(4, 3, vec![(3, 2), (4, 4), (1, 2), (2, 3)], vec![2, 1, 10]));
        assert_eq!(vec![3], run(3, 1, vec![(2, 2), (2, 3), (2, 2)], vec![6]));
        assert_eq!(vec![2, 1, 4, 3], run(3, 4, vec![(2, 2), (4, 3), (5, 4)], vec![6, 3, 9, 7]));
        assert_eq!(vec![237857247, 370775395, 0, 34936934, 0, 194935142, 457364459, 594147177], run(6, 8, vec![(254859174, 143139414), (93613293, 194935142), (357382831, 801995983), (975916146, 20247892), (739377425, 753031505), (735561543, 682006760)], vec![595240078, 728158226, 31923474, 128550227, 52197244, 332004808, 814747290, 951530008]));
    }
}
