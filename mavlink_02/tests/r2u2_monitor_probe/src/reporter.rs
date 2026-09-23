//! W1 candidate adapter for HAMR's generated verdict log interface.
//! No allocation or deadline arithmetic; W2 must integrate into editable logging.rs.
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};

const FALSE_VERDICT: &str = "hlr_30_llr_15_16_recovery_deadline is currently false";
pub const TIMEOUT_MESSAGE: &str = "Mode-transition timeout: Recovery not observed by D2";

pub struct TimeoutReporter {
    reported: AtomicBool,
}
impl TimeoutReporter {
    pub const fn new() -> Self { Self { reported: AtomicBool::new(false) } }
    pub fn reset(&self) { self.reported.store(false, Ordering::Relaxed); }
    /// Called inside the logger, before ordinary level filtering.
    /// The caller emits an Error record directly to its sink, avoiding recursion.
    pub fn observe(&self, record: &log::Record<'_>) -> bool {
        if record.level() != log::Level::Info
            || !record.target().ends_with("::component::r2u2_monitor") {
            return false;
        }
        let mut matcher = ExactMessage { remaining: FALSE_VERDICT, matches: true };
        let _ = fmt::write(&mut matcher, *record.args());
        matcher.matches && matcher.remaining.is_empty()
            && !self.reported.swap(true, Ordering::Relaxed)
    }
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
