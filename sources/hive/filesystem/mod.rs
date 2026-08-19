use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct FileInputStream
{
	file: File
}

impl FileInputStream
{
	pub fn new<P>(
		path: P
	) -> io::Result<Self>
	where
		P: Into<PathBuf>
	{
		Ok(Self { file: OpenOptions::new().read(true).open(path.into())? })
	}
	
	pub fn into_file(
		self
	) -> File
	{
		self.file
	}
}

impl Read for FileInputStream
{
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		self.file.read(buffer)
	}
}

pub struct FileOutputStream
{
	file: File
}

impl FileOutputStream
{
	pub fn new<P>(
		path: P
	) -> io::Result<Self>
	where
		P: Into<PathBuf>
	{
		Ok(Self { file: OpenOptions::new().write(true).create(true).open(path.into())? })
	}
	
	pub fn into_file(
		self
	) -> File
	{
		self.file
	}
}

impl Write for FileOutputStream
{
	fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
		self.file.write(buffer)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.file.flush()
	}
}
