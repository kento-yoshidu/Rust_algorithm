// https://atcoder.jp/contests/abc085/tasks/abc085_a

fn run(s: &str) -> String {
	let vec: Vec<&str> = s.split("/").collect();

	format!("2018/{}/{}", vec[1], vec[2])
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(&'static str, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase("2017/01/07", "2018/01/07"),
			TestCase("2017/01/31", "2018/01/31"),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
