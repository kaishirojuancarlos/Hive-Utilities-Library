use crate::hive::log::{FileLogger, Logger, StandardLogger};

pub fn launch_test()
{
	let mut logger = FileLogger::new("TEST.txt");
	logger.log_process("ALARIC!");
	logger.increase_margin();
	logger.log_error("0x0");
}
