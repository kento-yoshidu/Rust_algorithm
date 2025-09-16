// https://atcoder.jp/contests/abc140/tasks/abc140_a

fn run(n: usize) -> usize {
	n * n * n
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize);

	#[test]
	fn abc140_a() {
		let tests = [
			TestCase(2, 8),
			TestCase(1, 1),
			TestCase(3, 27),
		];

		for TestCase(n, expected) in tests {
			assert_eq!(run(n), expected);
		}
	}
}
