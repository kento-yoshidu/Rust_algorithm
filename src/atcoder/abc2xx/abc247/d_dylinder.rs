/*
use std::collections::VecDeque;

#[allow(unused)]
fn run(q: usize, vec: Vec<Vec<usize>>) {
    let mut deque = VecDeque::new();

    let mut ans = Vec::<usize>::new();

    for v in vec {
        if v[0] == 1 {
            deque.push_back((v[1], v[2]));
        } else {
            let mut count = 0;

            loop {
                let mut rest = v[1];

                let deque_num = &mut deque.front().unwrap().1;
                let element = deque.get(0).unwrap();

                println!("{:?}", element);

                if deque_num > &mut rest {
                    deque.remove(0);
                    deque.push_front((element.0, *deque_num - rest));
                }

                println!("rest = {}, num={}", rest, deque_num);

                break
            }

        }
    }

    println!("{:?}", deque);


}

fn main() {
    run(4, vec![vec![1, 2, 3], vec![2, 2], vec![1, 3, 4], vec![2, 3]]);
}
*/
