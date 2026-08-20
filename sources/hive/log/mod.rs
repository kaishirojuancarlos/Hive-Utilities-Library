use crate::hive::io::StandardOutputStream;
use crate::hive::text::{NEWLINE, TAB, WHITESPACE};
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Stdout, Write, stdout};
use std::path::PathBuf;

pub trait Logger
{
	fn get_margin_size(
		&self
	) -> usize;
	fn set_margin_size(
		&mut self,
		size: usize
	);
	
	fn submit_log<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>;
	
	fn log<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>
	{
		let mut data = String::new();
		let margin_size = self.get_margin_size();
		let margin = generate_margin(margin_size);
		data.push_str(&*margin);
		data.push_str(&*text.into());
		self.submit_log(data);
	}
	
	fn log_process<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*DataTypeIndicator::Process.to_string());
		data.push_str(&*text.into());
		self.log(data);
	}
	
	fn log_alert<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*DataTypeIndicator::Alert.to_string());
		data.push_str(&*text.into());
		self.log(data);
	}
	
	fn log_error<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*DataTypeIndicator::Error.to_string());
		data.push_str(&*text.into());
		self.log(data);
	}
	
	fn log_header<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*DataTypeIndicator::Header.to_string());
		data.push_str(&*text.into());
		self.log(data);
	}
	
	fn log_sub_data<T>(
		&mut self,
		text: T
	)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*DataTypeIndicator::SubData.to_string());
		data.push_str(&*text.into());
		self.log(data);
	}
	
	fn increase_margin(
		&mut self
	)
	{
		let margin_size = self.get_margin_size();
		let size = if margin_size >= usize::MAX
		{
			usize::MAX
		}
		else
		{
			margin_size + 1
		};
		self.set_margin_size(size);
	}
	
	fn decrease_margin(
		&mut self
	)
	{
		let margin_size = self.get_margin_size();
		let size = if margin_size <= 1
		{
			0
		}
		else
		{
			margin_size - 1
		};
		self.set_margin_size(size);
	}
	
	fn reset_margin_size(
		&mut self
	)
	{
		self.set_margin_size(0);
	}
}

pub const MARGIN_SPACER: char = TAB;

pub fn generate_margin(
	margin_size: usize
) -> String
{
	let mut margin = String::new();
	if margin_size > 0
	{
		for _ in 0..margin_size
		{
			margin.push(MARGIN_SPACER);
		}
	}
	margin
}

pub enum DataTypeIndicator
{
	None,
	Process,
	Alert,
	Error,
	Header,
	SubData
}

impl Display for DataTypeIndicator
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self
		{
			DataTypeIndicator::None =>
				String::from(WHITESPACE),
			DataTypeIndicator::Process =>
				"... ".parse().unwrap(),
			DataTypeIndicator::Alert =>
				"! ".parse().unwrap(),
			DataTypeIndicator::Error =>
				"!! ".parse().unwrap(),
			DataTypeIndicator::Header =>
				":: ".parse().unwrap(),
			DataTypeIndicator::SubData =>
				"-- ".parse().unwrap()
		};
		write!(f, "{}", str)
	}
}

pub struct StandardLogger
{
	stream: StandardOutputStream<Stdout>,
	margin_size: usize
}

impl StandardLogger
{
	pub fn new() -> Self
	{
		Self { stream: StandardOutputStream::new(stdout()), margin_size: 0 }
	}
}

impl Logger for StandardLogger
{
	fn get_margin_size(&self) -> usize {
		self.margin_size
	}
	
	fn set_margin_size(&mut self, size: usize) {
		let size_processed = if size < 1
		{
			0
		}
		else if size >= usize::MAX
		{
			usize::MAX
		}
		else
		{
			size
		};
		self.margin_size = size_processed;
	}
	
	fn submit_log<T>(&mut self, text: T)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*text.into());
		data.push(NEWLINE);
		self.stream.write_all(data.as_ref())
			.unwrap();
	}
}

pub struct FileLogger
{
	file: File,
	margin_size: usize
}

impl FileLogger
{
	pub fn new<P>(
		path: P
	) -> Self
	where
		P: Into<PathBuf>
	{
		Self { file: OpenOptions::new().write(true).create(true).append(true).open(path.into()).unwrap(), margin_size: 0 }
	}
}

impl Logger for FileLogger
{
	fn get_margin_size(&self) -> usize {
		self.margin_size
	}
	
	fn set_margin_size(&mut self, size: usize) {
		let size_processed = if size < 1
		{
			0
		}
		else if size >= usize::MAX
		{
			usize::MAX
		}
		else
		{
			size
		};
		self.margin_size = size_processed;
	}
	
	fn submit_log<T>(&mut self, text: T)
	where
		T: Into<String>
	{
		let mut data = String::new();
		data.push_str(&*text.into());
		data.push(NEWLINE);
		self.file.write_all(data.as_ref())
			.unwrap();
	}
}
