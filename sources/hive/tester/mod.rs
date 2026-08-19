use crate::hive::filesystem::{FileInputStream, FileOutputStream};
use std::io::{Read, Write};

pub fn launch_test()
{
	let file = "TEST.txt";
	let mut output = FileOutputStream::new(file)
		.unwrap();
	output.write_all(b"ALARIC!")
		.unwrap();
	output.flush()
		.unwrap();
	let mut input = FileInputStream::new(file)
		.unwrap();
	let mut data = String::new();
	input.read_to_string(&mut data)
		.unwrap();
	println!(":: {data}");
}
