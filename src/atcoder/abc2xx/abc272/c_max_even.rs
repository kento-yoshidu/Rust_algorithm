// https://atcoder.jp/contests/abc272/tasks/abc272_c

pub fn run(_n: usize, a: Vec<isize>) -> isize {
    let (mut even, mut odd): (Vec<isize>, Vec<isize>) = a.iter().partition(|num| {
        *num % 2 == 0
    });

    even.sort();
    even.reverse();

    odd.sort();
    odd.reverse();

    let mut ans = -1;

    if even.len() > 1 {
        ans = ans.max(even[0] + even[1]);
    }

    if odd.len() > 1 {
        ans = ans.max(odd[0] + odd[1]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(3, vec![2, 3, 4]));
        assert_eq!(-1, run(2, vec![1, 0]));
    }
}
