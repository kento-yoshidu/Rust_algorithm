// https://atcoder.jp/contests/abc087/tasks/abc087_a

fn run(x: usize, a: usize, b: usize) -> usize {
	let b_count = (x - a) / b;

	x - a - b * b_count
}

#[cfg(test)]
mod test {
	use super::*;

	struct TestCase(usize, usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(1234, 150, 100, 84),
			TestCase(1000, 108, 108, 24),
			TestCase(579, 123, 456, 0),
			TestCase(7477, 549, 593, 405),
		];

		for TestCase(x, a, b, expected) in tests {
			assert_eq!(run(x, a, b), expected);
		}
	}
}
