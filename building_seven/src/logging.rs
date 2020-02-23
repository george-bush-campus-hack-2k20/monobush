use fern;
use log::{info, trace, warn, debug};
pub fn setup() -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
    // Perform allocation-free log formatting
	.format(|out, message, record| {
	    out.finish(format_args!(
		"{}[{}][{}] {}",
		chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
		record.target(),
		record.level(),
		message
	    ))
	})
    // Add blanket level filter -
	.level(log::LevelFilter::Debug)
    // - and per-module overrides
	.level_for("hyper", log::LevelFilter::Error)
    // Output to stdout, files, and other Dispatch configurations
	.chain(std::io::stdout())
    // Apply globally
	.apply()
}

