pub fn quick(vec: &mut Vec<usize>, left: usize, right: usize) {
    if left >= right {
        return
    }

    let pivot = vec[left];

    let mut i = left;
    let mut j = right;

    loop {
        loop {
            if vec[i] >= pivot {
                break
            }

            i += 1;
        }

        loop {
            if pivot >= vec[j] {
                break
            }

            j -= 1;
        }

        if i >= j {
            break
        }

        let tmp_l = vec[i];
        let tmp_r = vec[j];

        vec[i] = tmp_r;
        vec[j] = tmp_l;
    }

    quick(vec, left, i-1);
    quick(vec, j+1, right);
}

pub fn main() {
    let mut vec = vec![5, 0, 9, 7, 1, 6, 3, 8, 4, 2];

    quick(&mut vec, 0, 9);

    println!("{:?}", vec);
}
