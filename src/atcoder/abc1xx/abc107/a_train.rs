// https://atcoder.jp/contests/abc107/tasks/abc107_a

fn run(n: usize, i: usize) -> usize {
	n - i + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(4, 2, 3),
			TestCase(1, 1, 1),
			TestCase(15, 11, 5),
		];

		for TestCase(n, i, expected) in tests {
			assert_eq!(run(n, i), expected);
		}
	}
}
