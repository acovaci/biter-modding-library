use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};

const LOG_PATH: &str = "/Users/adrian.covaci/Documents/Projects/biter-lib/logs/lib.log";
const LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

pub fn init() {
    init_config(
        Config::builder()
            .appender(
                Appender::builder().build(
                    "stdout",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
                            .build(),
                    ),
                ),
            )
            .appender(
                Appender::builder().build(
                    "logfile",
                    Box::new(
                        FileAppender::builder()
                            .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
                            .append(false)
                            .build(LOG_PATH)
                            .unwrap(),
                    ),
                ),
            )
            .logger(
                Logger::builder()
                    .appender("logfile")
                    .additive(true)
                    .build("lib", LevelFilter::Debug),
            )
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("logfile")
                    .build(LevelFilter::Debug),
            )
            .unwrap(),
    )
    .unwrap();
}
