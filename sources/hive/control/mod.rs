use std::error::Error;

pub fn escalate_with_root_access() -> Result<(), Box<dyn Error>>
{
	sudo::escalate_if_needed()?;
	Ok(())
}
