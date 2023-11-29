// https://atcoder.jp/contests/abc254/tasks/abc254_b

pub fn run(n: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![Vec::<usize>::new(); n];

    for i in 0..n {
        for j in 0..i+1 {
            if j == 0 || j == i {
                ans[i].push(1);
            } else {
                let num = ans[i-1][j-1];
                let num2 = ans[i-1][j];

                ans[i].push(num + num2);
            }
        }
    }

    ans
}

/*
pub fn run(n: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![Vec::<usize>::new(); n];

    let ans: Vec<_> = (0..n)
        .scan(vec![vec![]; n], |state, i| {
            println!("{}, {:?}", n, state);

            for j in 0..i+1 {
                if j == 0 || j == i {
                    state[i].push(1);
                } else {
                    state[i].push(999);
                }
            }
            Some(state.clone())
        })
        .collect();

    println!("{:?}", ans);

    vec![vec![0]]
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1], vec![1, 1], vec![1, 2, 1]], run(3));
        assert_eq!(vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1], vec![1, 5, 10, 10, 5, 1], vec![1, 6, 15, 20, 15, 6, 1], vec![1, 7, 21, 35, 35, 21, 7, 1], vec![1, 8, 28, 56, 70, 56, 28, 8, 1], vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]], run(10));
    }
}
