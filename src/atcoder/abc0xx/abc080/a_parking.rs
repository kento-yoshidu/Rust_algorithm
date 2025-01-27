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

	struct TestCase(usize, usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(7, 17, 120, 119),
			TestCase(5, 20, 100, 100),
			TestCase(6, 18, 100, 100),
		];

		for TestCase(n, a, b, expected) in tests {
			assert_eq!(run(n, a, b), expected);
		}
	}
}
