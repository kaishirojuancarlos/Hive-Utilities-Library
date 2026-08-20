use std::error::Error;
use sudo2::RunningAs;

pub fn elevate_with_root_access() -> Result<RunningAs, Box<dyn Error>>
{
	let var = std::env::var("XDG_CURRENT_DESKTOP");
	if var.is_err()
	{
		return sudo2::escalate_if_needed();
	}
	sudo2::pkexec()
}
