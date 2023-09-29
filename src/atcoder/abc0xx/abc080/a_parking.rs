// https://atcoder.jp/contests/abc080/tasks/abc080_a

fn run(n: usize, a: usize, b: usize) -> usize {
	if n * a <= b {
		n * a
	} else {
		b
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(119, run(7, 17, 120));
		assert_eq!(100, run(5, 20, 100));
		assert_eq!(100, run(6, 18, 100));
	}
}
