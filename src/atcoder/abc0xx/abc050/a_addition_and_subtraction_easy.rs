// https://atcoder.jp/contests/abc050/tasks/abc050_a

fn run(s: String) -> isize {
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

	#[test]
	fn test() {
		assert_eq!(3, run(String::from("1 + 2")));
		assert_eq!(-2, run(String::from("5 - 7")));
	}
}
