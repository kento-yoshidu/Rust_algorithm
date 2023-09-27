// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ec

fn check(s: &str) -> bool {
	s.chars().eq(s.chars().rev())
}

fn run(_n: usize, _q: usize, s: &str, vec: Vec<(usize, usize)>) -> Vec<String> {
	vec.iter().map(|v| {
		if check(&s[(v.0 - 1)..=(v.1 - 1)]) {
			String::from("Yes")
		} else {
			String::from("No")
		}
	}).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        assert_eq!(vec![String::from("Yes"), String::from("No"), String::from("Yes")], run(11, 3, "mississippi", vec![(5, 8), (6, 10), (2, 8)]));
	}
}
