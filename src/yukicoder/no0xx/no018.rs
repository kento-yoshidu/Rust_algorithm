// https://yukicoder.me/problems/no/18

fn run(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let base = (c as u8 - b'A') as i32;
            let shift = (i as i32 + 1) % 26;
            let shifted = (base - shift + 26) % 26;

            (shifted as u8 + b'A') as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_018() {
        let tests = [
            TestCase("BCD", "AAA"),
            TestCase("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "ZZZZZZZZZZZZZZZZZZZZZZZZZZ"),
            TestCase("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ", "YXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZ"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
