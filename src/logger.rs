// --------------
// RUNTIME LOGGER
// --------------

// struct RuntimeLogger {
//     // write to uefi console output
//     // not sure if this works on runtime services
//     writer: Option<NonNull<Output<'static>>>,
// }

// // st.stdout

// impl log::Log for RuntimeLogger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         metadata.level() <= Level::Info;
//         self.writer.is_some()
//     }

//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//             // println!("{} - {}", record.level(), record.args());
//         }
//         if let Some(mut ptr) = self.writer {
//             let writer = unsafe { ptr.as_mut() };
//             let result = DecoratedLog::write(
//                 writer,
//                 record.level(),
//                 record.args(),
//                 record.file().unwrap_or("<unknown file>"),
//                 record.line().unwrap_or(0),
//             );
//         }
//     }

//     fn flush(&self) {}
// }

// use log::{LevelFilter, SetLoggerError};

// static LOGGER: RuntimeLogger = RuntimeLogger;

// pub fn init_runtime_logger() -> Result<(), SetLoggerError> {
//     log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
// }
