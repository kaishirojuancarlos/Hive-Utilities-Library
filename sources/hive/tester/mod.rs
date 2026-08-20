use crate::hive::log::{Logger, StandardLogger};

pub fn launch_test()
{
	let mut logger = StandardLogger::new();
	logger.log_process("ALARIC!");
	logger.increase_margin();
	logger.log_error("0x0");
}
