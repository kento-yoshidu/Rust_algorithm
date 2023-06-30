#[allow(dead_code)]
pub fn run(h: usize, w: usize, vec: Vec<&str>) -> (usize, usize) {
    let mut top = h;
    let mut bottom = 0;
    let mut left = w;
    let mut right = 0;

    for (i, row) in vec.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                top = top.min(i);
                bottom = bottom.max(i);
                left = left.min(j);
                right = right.max(j);
            }
        }
    }

    let mut ans = (0, 0);

    for i in top..=bottom {
        let row: Vec<_> = vec[i].chars().collect();

        for j in left..=right {
            if row[j] != '#' {
                ans = (i+1, j+1);
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((2, 4), run(5, 6, vec!["......", "..#.#.", "..###.", "..###.", "......"]));
        assert_eq!((1, 2), run(3, 2, vec!["#.", "##", "##"]));
        assert_eq!((2, 5), run(6, 6, vec!["..####", "..##.#", "..####", "..####", "..####", "......"]));
    }
}
