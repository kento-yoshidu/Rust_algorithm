// https://atcoder.jp/contests/abc328/tasks/abc328_c

fn run(_n: usize, _q: usize, s: &str, vec: Vec<(usize, usize)>) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut cum = vec![0];

    for (i, a) in chars.windows(2).enumerate() {
        if a[0] == a[1] {
            cum.push(cum[i]+1)
        } else {
            cum.push(cum[i])
        }
    }

    vec.iter()
        .map(|(l, r)| {
            cum[*r-1] - cum[*l-1]
        })
        .collect()
}

/*
pub fn run2(_n: usize, _q: usize, s: &str, vec: Vec<(usize, usize)>) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();

    let cum: Vec<usize> = chars.windows(2)
        .scan(0, |state, arr| {
            if arr[0] == arr[1] {
                *state += 1;
            }

            Some(*state)
        })
        .collect();

    println!("{:?}", cum);

    vec.iter()
        .map(|(l, r)| {
            cum[*r] - cum[*l]
        })
        .collect()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 2, 0, 0], run(11, 4, "mississippi", vec![(3, 9), (4, 10), (4, 6), (7, 7)]));
        assert_eq!(vec![4], run(5, 1, "aaaaa", vec![(1, 5)]));
    }

    /*
    #[test]
    fn test2() {
        assert_eq!(vec![2, 2, 0, 0], run2(11, 4, "mississippi", vec![(3, 9), (4, 10), (4, 6), (7, 7)]));
        assert_eq!(vec![4], run2(5, 1, "aaaaa", vec![(1, 5)]));
    }
    */
}
