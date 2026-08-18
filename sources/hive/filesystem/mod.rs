use crate::hive::core::Immutable;
use std::fmt::{Display, Pointer};
use std::fs::{DirBuilder, File, Metadata, OpenOptions};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub type Result<T> = std::io::Result<T>;

pub struct Entry
{
	path: Immutable<PathBuf>
}

impl Entry
{
	pub fn new<T>(
		path: T
	) -> Self
	where
		T: Into<PathBuf>
	{
		Self { path: Immutable::new(path.into()) }
	}
	
	pub fn as_path_buf(
		&self
	) -> &PathBuf
	{
		self.path.get()
	}
	
	pub fn exists(
		&self
	) -> bool
	{
		self.as_path_buf().exists()
	}
	
	pub fn is_a_file(
		&self
	) -> bool
	{
		self.as_path_buf().is_file()
	}
	
	pub fn is_a_directory(
		&self
	) -> bool
	{
		self.as_path_buf().is_dir()
	}
	
	pub fn metadata(
		&self
	) -> Result<Metadata>
	{
		self.as_path_buf().metadata()
	}
	
	pub fn size(
		&self
	) -> Result<u64>
	{
		Ok(self.metadata()?.size())
	}
	
	pub fn create_file(
		&self
	) -> Result<File>
	{
		OpenOptions::new()
			.write(true)
			.create(true)
			.open(self.as_path_buf())
	}
	
	pub fn create_directory(
		&self
	) -> Result<()>
	{
		DirBuilder::new()
			.recursive(false)
			.create(self.as_path_buf())
	}
	
	pub fn create_directory_recursively(
		&self
	) -> Result<()>
	{
		DirBuilder::new()
			.recursive(true)
			.create(self.as_path_buf())
	}
	
	pub fn child<T>(
		&self,
		path: T
	) -> Self
	where
		T: Into<PathBuf>
	{
		Self::new(self.as_path_buf().join(path.into()))
	}
}

impl Display for Entry
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_path_buf().to_string_lossy())
	}
}
