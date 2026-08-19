use std::sync::{Arc, LockResult, Mutex, MutexGuard};

pub struct Immutable<T>
{
	store: Arc<T>
}

impl<T> Immutable<T>
{
	pub fn new<V>(
		value: V
	) -> Self
	where
		V: Into<T>
	{
		Self { store: Arc::new(value.into()) }
	}
	
	pub fn get(
		&self
	) -> &T
	{
		self.store.as_ref()
	}
}

pub struct Mutable<T>
{
	store: Arc<Mutex<T>>
}

impl<T> Mutable<T>
{
	pub fn new<V>(
		value: V
	) -> Self
	where
		V: Into<T>
	{
		Self { store: Arc::new(Mutex::new(value.into())) }
	}
	
	pub fn get(
		&self
	) -> LockResult<MutexGuard<T>>
	{
		self.store.lock()
	}
	
	pub fn set<V>(
		&mut self,
		value: V
	)
	where
		V: Into<T>
	{
		*self.store.lock().unwrap() = value.into();
	}
}
