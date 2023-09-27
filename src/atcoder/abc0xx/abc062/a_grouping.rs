// https://atcoder.jp/contests/abc062/tasks/abc062_a

fn run(x: usize, y: usize) -> String {
	let g = [0, 2, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0];

	if g[x-1] == g[y-1] {
		String::from("Yes")
	} else {
		String::from("No")
	}

}

fn check(num: usize) -> usize {
	match num {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 0,
		4 | 6 | 9 | 11 => 1,
		2 => 2,
		_ => unreachable!()
	}
}

pub fn run2(x: usize, y: usize) -> String {
	if check(x) == check(y) {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run(1, 3));
		assert_eq!(String::from("No"), run(2, 4));
	}
}
