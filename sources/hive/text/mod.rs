pub const WHITESPACE: char = '\u{0}';
pub const SPACE: char = ' ';
pub const TAB: char = '\t';
pub const NEWLINE: char = '\n';

#[macro_export]
macro_rules! concatenate {
	($($value:expr),* $(,)?) => {{
		use std::fmt::Write;
		let mut output = String::new();
		$(
			write!(&mut output, "{}", $value).unwrap();
		)*
		output
	}}
}
