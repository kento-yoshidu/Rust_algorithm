// https://atcoder.jp/contests/abc301/tasks/abc301_b

pub fn run(n: usize, a: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0];

    a.iter()
        .skip(1)
        .fold(vec![a[0]], |mut stack, num| {
            let last = stack.iter().last().unwrap();

            if (last - num).abs() == 1 {
                stack.push(*num);
                stack
            } else {
                if last < num {
                    stack.push(*num);

                    for i in (*last+1..*num).rev() {
                        stack.push(i);
                    }
                    stack
                } else {
                    stack
                }
            }
        })
}
