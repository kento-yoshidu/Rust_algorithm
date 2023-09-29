// https://atcoder.jp/contests/abc083/tasks/abc083_a

fn run(a: usize, b: usize, c: usize, d: usize) -> String {
	if a + b > c + d {
		String::from("Left")
	} else if a + b == c + d {
		String::from("Balanced")
	} else {
		String::from("Right")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Left"), run(3, 8, 7, 1));
		assert_eq!(String::from("Balanced"), run(3, 4, 5, 2));
		assert_eq!(String::from("Right"), run(1, 7, 6, 4));
	}
}
