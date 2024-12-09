// https://atcoder.jp/contests/abc050/tasks/abc050_a

fn run(s: &str) -> isize {
	let chars: Vec<&str> = s.split(" ").collect();

	let left: isize = chars[0].parse().unwrap();
	let right: isize = chars[2].parse().unwrap();

	match chars[1] {
		"+" => left + right,
		"-" => left - right,
		_ => unreachable!(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(&'static str, isize);

	#[test]
	fn test() {
		let tests = [
			TestCase("1 + 2", 3),
			TestCase("5 - 7", -2),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
