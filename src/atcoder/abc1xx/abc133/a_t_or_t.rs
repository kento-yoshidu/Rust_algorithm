// https://atcoder.jp/contests/abc133/tasks/abc133_a

fn run(n: usize, a: usize, b: usize) -> usize {
	b.min(n * a)
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize, usize);

	#[test]
	fn abc133_a() {
		let tests = [
			TestCase(4, 2, 9, 8),
			TestCase(4, 2, 7, 7),
			TestCase(4, 2, 8, 8),
		];

		for TestCase(n, a, b, expected) in tests {
			assert_eq!(run(n, a, b), expected);
		}
	}
}
