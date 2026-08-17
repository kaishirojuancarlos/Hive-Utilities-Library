use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/**
An immutable type wrapper.
*/
#[repr(transparent)]
pub struct Immutable<T>
{
	value: T
}

impl<T> Immutable<T>
{
	pub const fn new(value: T) -> Self
	{
		Self { value }
	}
	
	pub fn get(&self) -> &T
	{
		&self.value
	}
	
	pub fn into_inner(self) -> T
	{
		self.value
	}
}

impl<T> Deref for Immutable<T>
{
	type Target = T;
	
	fn deref(&self) -> &Self::Target
	{
		&self.value
	}
}

pub struct Mutable<T>
{
	value: RwLock<T>
}

impl<T> Mutable<T>
{
	pub const fn new(value: T) -> Self
	{
		Self { value: RwLock::new(value) }
	}
	
	pub fn get(&self) -> RwLockReadGuard<'_, T>
	{
		self.value.read().expect("mutable value lock was poisoned")
	}
	
	pub fn get_mut(&self) -> RwLockWriteGuard<'_, T>
	{
		self.value.write().expect("mutable value lock was poisoned")
	}
	
	pub fn set(&self, value: T)
	{
		*self.get_mut() = value;
	}
	
	pub fn into_inner(self) -> T{
		self.value.into_inner().expect("mutable value lock was poisoned")
	}
}
