// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_b

/*
    a < b < cの状態になるまでbとcを倍増させ、
    条件を満たしてから回数がオーバーしてないかをチェックする
*/
fn run(a: usize, mut b: usize, mut c: usize, k: usize) -> String {
    let mut count = 0;

    while a >= b {
        count += 1;
        b = b * 2;
    }

    while b >= c {
        count += 1;
        c = c * 2;
    }

    if count <= k {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(7, 2, 5, 3));
        assert_eq!(String::from("No"), run(7, 2, 4, 2));
    }
}
