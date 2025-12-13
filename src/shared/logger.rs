use std::{fmt::Arguments, fs::File, vec};
use alerta::*;
use log::{Log, Record};
use simplelog::*;
pub struct Logger {
    backend: CombinedLogger
}

impl Logger {
    pub fn new(debug: bool, logfile_path: &str) -> Self {
        let logfile = File::create(logfile_path).expect("Could'nt create log file");
        let loglevel = if debug {LevelFilter::Trace} else {LevelFilter::Warn};
        let logconf = ConfigBuilder::new()
                    .set_level_padding(LevelPadding::Right) // pad the levels for alignent
                    .set_thread_level(LevelFilter::Error) // always show thread
                    .set_location_level(if debug {LevelFilter::Error} else {LevelFilter::Debug}) // Only show the location for debug or in debug mode
                    .set_thread_padding(ThreadPadding::Right(5)) // pad the thread for alignent  
                    .set_level_color(Level::Trace, Some(Color::White))  // pretty colors :3
                    .set_level_color(Level::Debug, Some(Color::Cyan))  // pretty colors :3
                    .set_level_color(Level::Info , Some(Color::Green))   // pretty colors :3
                    .set_level_color(Level::Warn , Some(Color::Yellow)) // pretty colors :3
                    .set_level_color(Level::Error, Some(Color::Red))    // pretty colors :3
                    .set_thread_mode(ThreadLogMode::Names) // show thread names
                    .build();

        Self { 
            backend: *CombinedLogger::new(vec![
                TermLogger::new(loglevel, logconf.clone(), TerminalMode::Stdout, ColorChoice::Auto),
                WriteLogger::new(loglevel, logconf.clone(), logfile)
            ])
        }
    }

    pub fn log(&self, level: Level, message: Arguments,file: Option<&str>,line: Option<u32>) {
        self.backend.log(
            &Record::builder()
                        .level(level)
                        .file(file)
                        .line(line)
                        .args(format_args!("{message}"))
                        .build()
        );
    }

    pub fn popup(&self, level: Level, title: &str, message: Arguments,file: Option<&str>,line: Option<u32>){
        let icon = match level {
            Level::Error => Icon::Error,
            Level::Warn => Icon::Warning,
            Level::Info => Icon::Info,
            Level::Debug => Icon::Info,
            Level::Trace => Icon::Info,
        };

        let _ = alerta().title(&title.to_string()).message(format!("{message}")).icon(icon).show();

        self.log(level, message,file,line);
    }

    pub fn fatal(&self, message: Arguments,file: Option<&str>,line: Option<u32>) -> ! {
        let _ =  alerta()
                    .title("Fatal error")
                    .message(format!("{message}"))
                    .icon(Icon::Error)
                    .show();

        self.log(Level::Error, message,file,line);

        panic!(); // its fatal afterall
    }
}