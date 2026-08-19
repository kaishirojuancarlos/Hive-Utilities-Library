use crate::concatenate;
use crate::hive::text::SPACE;

pub fn launch_test()
{
	let string = concatenate!("ALAINE!", " -- ", 2290, SPACE, '@');
	println!(":: {string}");
}
