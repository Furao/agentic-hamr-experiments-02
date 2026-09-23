// This file will not be overwritten if HAMR codegen is rerun

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};
use log::{Level, LevelFilter, Log, Metadata, Record};

const FALSE_VERDICT: &str = "hlr_30_llr_15_16_recovery_deadline is currently false";
pub(crate) const TIMEOUT_MESSAGE: &str = "Mode-transition timeout: Recovery not observed by D2";
const MONITOR_TARGET: &str = concat!(env!("CARGO_CRATE_NAME"), "::component::r2u2_monitor");
static REPORTED: AtomicBool = AtomicBool::new(false);
static LOGGER: MonitorLogger = MonitorLogger;

#[cfg(all(feature = "sel4", not(test)))]
static BACKEND: sel4_logging::Logger = sel4_logging::LoggerBuilder::const_default()
    .level_filter(LevelFilter::Info)
    .write(|s| sel4::debug_print!("{}", s))
    .build();

#[cfg(test)]
static RECORDS: std::sync::Mutex<Vec<(Level, String)>> = std::sync::Mutex::new(Vec::new());
#[cfg(test)]
pub(crate) fn take_records() -> Vec<(Level, String)> {
    core::mem::take(&mut *RECORDS.lock().unwrap())
}

struct ExactMessage<'a> { remaining: &'a str, matches: bool }
impl Write for ExactMessage<'_> {
    fn write_str(&mut self, fragment: &str) -> fmt::Result {
        if let Some(tail) = self.remaining.strip_prefix(fragment) {
            self.remaining = tail;
        } else {
            self.matches = false;
        }
        Ok(())
    }
}

// Called directly, never via log!, to avoid recursively invoking the adapter.
fn emit(record: &Record<'_>) {
    #[cfg(all(feature = "sel4", not(test)))]
    BACKEND.log(record);
    #[cfg(test)]
    RECORDS.lock().unwrap().push((record.level(), record.args().to_string()));
}

struct MonitorLogger;
impl Log for MonitorLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool { metadata.level() <= Level::Info }
    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) { return; }
        if record.target() == MONITOR_TARGET {
            if record.level() == Level::Info {
                let mut matcher = ExactMessage { remaining: FALSE_VERDICT, matches: true };
                let _ = fmt::write(&mut matcher, *record.args());
                if matcher.matches && matcher.remaining.is_empty()
                    && !REPORTED.swap(true, Ordering::Relaxed) {
                    emit(&Record::builder().level(Level::Error).target(module_path!())
                        .args(format_args!("{}", TIMEOUT_MESSAGE)).build());
                }
                // Generated routine verdict statuses are not operational messages.
                return;
            }
        }
        emit(record);
    }
    fn flush(&self) {
        #[cfg(all(feature = "sel4", not(test)))]
        BACKEND.flush();
    }
}

pub fn init_logging() {
    REPORTED.store(false, Ordering::Relaxed);
    #[cfg(all(feature = "sel4", not(test)))]
    log::set_logger(&LOGGER).unwrap();
    #[cfg(test)]
    {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| log::set_logger(&LOGGER).unwrap());
        take_records();
    }
    // HAMR emits verdicts at Info; retain them until the adapter translates them.
    log::set_max_level(LevelFilter::Info);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[serial_test::serial]
    fn reporter_matches_exact_source_message_and_level() {
        init_logging();
        for (target, level, message) in [
            ("other", Level::Info, FALSE_VERDICT),
            (MONITOR_TARGET, Level::Warn, FALSE_VERDICT),
            (MONITOR_TARGET, Level::Debug, FALSE_VERDICT),
            (MONITOR_TARGET, Level::Info, "hlr_30_llr_15_16_recovery_deadline is currently false extra"),
            (MONITOR_TARGET, Level::Info, "hlr_30_llr_15_16_recovery_deadline"),
        ] {
            LOGGER.log(&Record::builder().target(target).level(level)
                .args(format_args!("{}", message)).build());
        }
        assert_eq!(take_records(), vec![(Level::Info, FALSE_VERDICT.into()), (Level::Warn, FALSE_VERDICT.into())]);
        for _ in 0..3 {
            let name = "hlr_30_llr_15_16_recovery_deadline";
            LOGGER.log(&Record::builder().target(MONITOR_TARGET).level(Level::Info)
                .args(format_args!("{} is currently {}", name, false)).build());
        }
        assert_eq!(take_records(), vec![(Level::Error, TIMEOUT_MESSAGE.into())]);
        LOGGER.flush();
    }
}
