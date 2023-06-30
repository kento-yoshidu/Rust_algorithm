// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ce

#[allow(unused)]
pub fn run(n: usize, a: Vec<usize>, q: usize, vec: Vec<Vec<usize>>) -> Vec<String> {
    let mut ans = Vec::<String>::new();
    let mut atari = Vec::<usize>::new();
    let mut hazure = Vec::<usize>::new();

    atari.push(0);
    hazure.push(0);

    for i in 1..n {
        atari.push(atari[i - 1]);
        hazure.push(hazure[i - 1]);

        if a[i] == 1 {
            atari[i] += 1;
        } else {
            hazure[i] += 1;
        }
    }

    for v in vec.iter() {
        let a = atari[v[1]-1] as i32 - atari[v[0]-2] as i32;
        let h = hazure[v[1]-1] as i32 - hazure[v[0]-2] as i32;

        if a > h {
            ans.push(String::from("win"));
        } else if a == h {
            ans.push(String::from("draw"));
        } else {
            ans.push(String::from("lose"));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("win"), String::from("draw"), String::from("lose")], run(7, vec![0, 1, 1, 0, 1, 0, 0], 3, vec![vec![2, 5], vec![2, 7], vec![5, 7]]));
    }
}
