use crate::hive::control::elevate_with_root_access;

pub fn launch_test()
{
	unsafe
	{
		elevate_with_root_access()
			.unwrap();
	}
}
