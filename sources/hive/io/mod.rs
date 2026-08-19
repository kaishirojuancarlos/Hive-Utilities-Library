pub mod serialisation;

use std::io;
use std::io::{Read, Write};

pub const DEFAULT_BUFFER_SIZE: usize = 8192;

pub trait InputStream: Read
{
	fn skip(
		&mut self,
		amount: u64
	) -> io::Result<u64>
	{
		let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
		let mut remaining = amount;
		let mut skipped = 0;
		while remaining > 0
		{
			let size = remaining.min(buffer.len() as u64) as usize;
			let count = self.read(&mut buffer[..size])?;
			if count == 0
			{
				break;
			}
			skipped += count as u64;
			remaining -= count as u64;
		}
		Ok(skipped)
	}
	
	fn copy_to<W>(
		&mut self,
		output: &mut W
	) -> io::Result<u64>
	where
		Self: Sized,
		W: Write + Sized
	{
		copy(self, output)
	}
}

impl<R> InputStream for R
where
	R: Read + Sized {}

pub trait OutputStream: Write
{
	fn write_bytes(
		&mut self,
		byte: u8
	) -> io::Result<()>
	{
	self.write_all(&[byte])
	}
	
	fn copy_from<R>(
		&mut self,
		input: &mut R
	) -> io::Result<u64>
	where
		Self: Sized,
		R: Read + Sized
	{
		copy(input, self)
	}
}

impl<W> OutputStream for W
where
	W: Write + Sized {}

pub struct StandardOutputStream<T>
where
	T: Write + Sized
{
	inner_stream: T
}

impl<T> StandardOutputStream<T>
where
	T: Write + Sized
{
	pub fn new<S>(
		stream: S
	) -> Self
	where
		S: Into<T>
	{
		Self { inner_stream: stream.into() }
	}
}

impl<T> Write for StandardOutputStream<T>
	where
		T: Sized + Write,
{
	fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
		self.inner_stream.write(buffer)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.inner_stream.flush()
	}
}

pub struct StandardInputStream<T>
where
	T: Read + Sized
{
	inner_stream: T
}

impl<T> StandardInputStream<T>
where
	T: Read + Sized
{
	pub fn new<S>(
		stream: S
	) -> Self
	where
		S: Into<T>
	{
		Self { inner_stream: stream.into() }
	}
}

impl<T> Read for StandardInputStream<T>
	where
		T: Read + Sized,
{
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		self.inner_stream.read(buffer)
	}
}

pub fn copy<R, W>(
	input: &mut R,
	output: &mut W
) -> io::Result<u64>
where
	R: Read + Sized,
	W: Write + Sized
{
	io::copy(input, output)
}

pub struct BufferedInputStream<T>
where
	T: Read + Sized
{
	inner_stream: T,
	buffer: Vec<u8>,
	position: usize,
	length: usize
}

impl<T> BufferedInputStream<T>
where
	T: Read + Sized
{
	pub fn with_capacity(
		stream: T,
		capacity: usize
	) -> Self
	{
		Self { inner_stream: stream, buffer: vec![0; capacity], position: 0, length: 0 }
	}
	
	pub fn new(
		stream: T
	) -> Self
	{
		Self::with_capacity(stream, DEFAULT_BUFFER_SIZE)
	}
	
	pub fn into_inner_stream(
		self
	) -> T
	{
		self.inner_stream
	}
	
	pub fn refill(
		&mut self
	) -> io::Result<bool>
	{
		self.position = 0;
		self.length = self.inner_stream.read(&mut self.buffer)?;
		Ok(self.length != 0)
	}
}

impl<T> Read for BufferedInputStream<T>
where
	T: Read + Sized
{
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		if buffer.is_empty()
		{
			return Ok(0);
		}
		let mut written = 0;
		while written < buffer.len()
		{
			if self.position == self.length
			{
				if !self.refill()?
				{
					break;
				}
			}
			let available = self.length - self.position;
			let requested = buffer.len() - written;
			let count = available.min(requested);
			buffer[written..written + count]
				.copy_from_slice(&self.buffer[self.position..self.position + count]);
			self.position += count;
			written += count;
		}
		Ok(written)
	}
}

pub struct BufferedOutputStream<T>
where
	T: Write + Sized
{
	inner_stream: T,
	buffer: Vec<u8>
}

impl<T> BufferedOutputStream<T>
where
	T: Write + Sized
{
	pub fn with_capacity(
		stream: T,
		capacity: usize
	) -> Self
	{
		Self { inner_stream: stream, buffer: Vec::with_capacity(capacity) }
	}
	
	pub fn new(
		stream: T
	) -> Self
	{
		Self::with_capacity(stream, DEFAULT_BUFFER_SIZE)
	}
	
	pub fn into_inner_stream(
		mut self
	) -> io::Result<T>
	{
		self.flush()?;
		Ok(self.inner_stream)
	}
}

impl<T> Write for BufferedOutputStream<T>
where
	T: Write + Sized
{
	fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
		self.buffer.extend_from_slice(buffer);
		if self.buffer.len() >= self.buffer.capacity()
		{
			self.flush()?;
		}
		Ok(buffer.len())
	}
	
	fn flush(&mut self) -> io::Result<()> {
		if !self.buffer.is_empty()
		{
			self.inner_stream.write_all(&self.buffer)?;
			self.buffer.clear();
		}
		self.inner_stream.flush()
	}
}

pub fn process_stream<R, W>(
	input: &mut R,
	output: &mut W
) -> io::Result<()>
where
	R: Read + Sized,
	W: Write + Sized
{
	let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
	loop
	{
		let count = input.read(&mut buffer)?;
		if count == 0
		{
			break;
		}
		output.write_all(&buffer[..count])?;
	}
	Ok(())
}

pub trait HasBufferedOutputStream<T>
where
	T: Write + Sized
{
	fn into_buffered_output_stream(self) -> BufferedOutputStream<T>;
}

pub trait HasBufferedInputStream<T>
where
	T: Read + Sized
{
	fn into_buffered_input_stream(self) -> BufferedInputStream<T>;
}

impl<W> HasBufferedOutputStream<W> for W
where
	W: OutputStream
{
	fn into_buffered_output_stream(self) -> BufferedOutputStream<W> {
		BufferedOutputStream::new(self)
	}
}

impl<R> HasBufferedInputStream<R> for R
where
	R: InputStream
{
	fn into_buffered_input_stream(self) -> BufferedInputStream<R> {
		BufferedInputStream::new(self)
	}
}
