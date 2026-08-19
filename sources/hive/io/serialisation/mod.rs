use bincode::config::{Configuration, standard};
use bincode::error::{DecodeError, EncodeError};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io;
use std::io::{Read, Write};

pub trait Serialiser
{
	type Output;
	type Error;
	
	fn serialise<T>(
		&mut self,
		value: &T
	) -> Result<Self::Output, Self::Error>
	where
		T: Serialize;
}

pub trait Deserialiser
{
	type Input;
	type Error;
	fn deserialise<T>(
		&mut self,
		input: Self::Input
	) -> Result<T, Self::Error>
	where
		T: DeserializeOwned;
}

pub struct JsonSerialiser;

impl JsonSerialiser
{
	pub fn new() -> Self
	{
		Self {}
	}
}

impl Serialiser for JsonSerialiser
{
	type Output = String;
	type Error = serde_json::Error;
	
	fn serialise<T>(&mut self, value: &T) -> Result<Self::Output, Self::Error>
		where
			T: Serialize
	{
		serde_json::to_string(value)
	}
}

impl Deserialiser for JsonSerialiser
{
	type Input = String;
	type Error = serde_json::Error;
	
	fn deserialise<T>(&mut self, input: Self::Input) -> Result<T, Self::Error>
		where
			T: DeserializeOwned
	{
		serde_json::from_str(&input)
	}
}

pub struct CiboriumSerialiser;

impl CiboriumSerialiser
{
	pub fn new() -> Self
	{
		Self {}
	}
}

impl Serialiser for CiboriumSerialiser
{
	type Output = Vec<u8>;
	type Error = ciborium::ser::Error<std::io::Error>;
	
	fn serialise<T>(&mut self, value: &T) -> Result<Self::Output, Self::Error>
		where
			T: Serialize
	{
		let mut buffer = Vec::new();
		ciborium::into_writer(value, &mut buffer)?;
		Ok(buffer)
	}
}

impl Deserialiser for CiboriumSerialiser
{
	type Input = Vec<u8>;
	type Error = ciborium::de::Error<std::io::Error>;
	
	fn deserialise<T>(&mut self, input: Self::Input) -> Result<T, Self::Error>
		where
			T: DeserializeOwned
	{
		ciborium::from_reader(input.as_slice())
	}
}

pub struct BincodeSerialiser
{
	configuration: Configuration
}

impl BincodeSerialiser
{
	pub fn with_confiuration(
		configuration: Configuration
	) -> Self
	{
		Self { configuration }
	}
	
	pub fn new() -> Self
	{
		Self::with_confiuration(standard())
	}
	
	pub fn configuration(
		&self
	) -> Configuration
	{
		self.configuration
	}
}

impl Serialiser for BincodeSerialiser
{
	type Output = Vec<u8>;
	type Error = EncodeError;
	
	fn serialise<T>(&mut self, value: &T) -> Result<Self::Output, Self::Error>
		where
			T: Serialize
	{
		bincode::serde::encode_to_vec(value, self.configuration)
	}
}

impl Deserialiser for BincodeSerialiser
{
	type Input = Vec<u8>;
	type Error = DecodeError;
	
	fn deserialise<T>(&mut self, input: Self::Input) -> Result<T, Self::Error>
		where
			T: DeserializeOwned
	{
		let (value, consumed) = bincode::serde::decode_from_slice(&input, self.configuration)?;
		if consumed != input.len()
		{
			return Err(DecodeError::OtherString("trailing data after object".into()));
		}
		Ok(value)
	}
}

pub const OBJECT_SERIALISER: BincodeSerialiser = BincodeSerialiser { configuration: standard() };

pub struct ObjectOutputStream<W>
where
	W: Write + Sized
{
	inner_stream: W,
	serialiser: BincodeSerialiser
}

impl<W> ObjectOutputStream<W>
where
	W: Write + Sized
{
	pub fn new(
		stream: W
	) -> Self
	{
		Self { inner_stream: stream, serialiser: OBJECT_SERIALISER }
	}
	
	pub fn write_object<T>(
		&mut self,
		value: &T
	) -> io::Result<()>
	where
		T: Serialize
	{
		let data = self.serialiser
			.serialise(value)
			.map_err(io::Error::other)?;
		let length = u64::try_from(data.len())
			.map_err(|_| io::Error::other("object is too large"))?;
		self.inner_stream.write_all(&length.to_be_bytes())?;
		self.inner_stream.write_all(&data)?;
		Ok(())
	}
	
	pub fn into_inner_stream(
		self
	) -> W
	{
		self.inner_stream
	}
	
	pub fn serialiser(
		&self
	) -> &BincodeSerialiser
	{
		&self.serialiser
	}
	
	pub fn serialiser_mutable(
		&mut self
	) -> &mut BincodeSerialiser
	{
		&mut self.serialiser
	}
}

impl<W> Write for ObjectOutputStream<W>
where
	W: Write + Sized
{
	fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
		self.inner_stream.write(buffer)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.inner_stream.flush()
	}
}

pub struct ObjectInputStream<R>
where
	R: Read + Sized
{
	inner_stream: R,
	deserialiser: BincodeSerialiser
}

impl<R> ObjectInputStream<R>
where
	R: Read + Sized
{
	pub fn new(
		stream: R
	) -> Self
	{
		Self { inner_stream: stream, deserialiser: OBJECT_SERIALISER }
	}
	
	
	pub fn read_object<T>(
		&mut self
	) -> io::Result<T>
	where
		T: DeserializeOwned + Sized
	{
		let mut length_buffer = [0u8; 8];
		self.inner_stream.read_exact(&mut length_buffer)?;
		let length = u64::from_be_bytes(length_buffer);
		let length = usize::try_from(length)
			.map_err(|_| io::Error::other("object is too large"))?;
		let mut data = vec![0u8; length];
		self.inner_stream.read_exact(&mut data)?;
		self.deserialiser
			.deserialise(data)
			.map_err(io::Error::other)
	}
	
	pub fn into_inner_stream(
		self
	) -> R
	{
		self.inner_stream
	}
	
	pub fn deserialiser(
		&self
	) -> &BincodeSerialiser
	{
		&self.deserialiser
	}
	
	pub fn deserialiser_mutable(
		&mut self
	) -> &mut BincodeSerialiser
	{
		&mut self.deserialiser
	}
}

impl<R> Read for ObjectInputStream<R>
where
	R: Read + Sized
{
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		self.inner_stream.read(buffer)
	}
}
