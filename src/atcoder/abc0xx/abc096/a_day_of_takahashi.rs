// https://atcoder.jp/contests/abc096/tasks/abc096_a

fn run(a: usize, b: usize) -> usize {
	if a <= b {
		a
	} else {
		if a == 1 {
			1
		} else {
			a - 1
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(5, 5, 5),
			TestCase(2, 1, 1),
			TestCase(11, 30, 11),
			TestCase(1, 1, 1),
		];

		for TestCase(a, b, expected) in tests {
			assert_eq!(run(a, b), expected);
		}
	}
}
