// https://atcoder.jp/contests/abc071/tasks/abc071_a

fn run(x: isize, a: isize, b: isize) -> char {
	let abs_a = (x - a).abs();
	let abs_b = (x - b).abs();

	if abs_a < abs_b {
		'A'
	} else {
		'B'
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(isize, isize, isize, char);

	#[test]
	fn test() {
		let tests = [
			TestCase(5, 2, 7, 'B'),
			TestCase(1, 999, 1000, 'A'),
		];

		for TestCase(x, a, b, expected) in tests {
			assert_eq!(run(x, a, b), expected);
		}
	}
}
