#![allow(non_snake_case, non_camel_case_types, unused_imports, dead_code, static_mut_refs)]
// Only the application/API shells are stubs. The monitor module and spec.bin are
// copied byte-for-byte by build.rs from HAMR output; the actual R2U2 runtime runs.
extern crate self as data;
pub mod open_platform_Data_Model {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub enum OperatingMode { Normal = 0, Recovery = 1 }
}
pub mod bridge {
    pub mod seL4_MAVLinkFirewall_MAVLinkFirewall_GUMBOX {
        pub fn error_threshold() -> u16 { 5 }
    }
    pub mod seL4_MAVLinkFirewall_MAVLinkFirewall_api {
        use crate::open_platform_Data_Model::OperatingMode;
        pub trait seL4_MAVLinkFirewall_MAVLinkFirewall_Full_Api {}
        pub struct Stub;
        impl seL4_MAVLinkFirewall_MAVLinkFirewall_Full_Api for Stub {}
        pub struct seL4_MAVLinkFirewall_MAVLinkFirewall_Application_Api<A> {
            pub mode: OperatingMode,
            pub error: bool,
            pub marker: core::marker::PhantomData<A>,
        }
        impl<A> seL4_MAVLinkFirewall_MAVLinkFirewall_Application_Api<A> {
            pub fn peek_current_mode(&self) -> OperatingMode { self.mode }
            pub fn peek_error_status(&self) -> bool { self.error }
        }
    }
}
pub mod component {
    pub mod seL4_MAVLinkFirewall_MAVLinkFirewall_app {
        pub struct seL4_MAVLinkFirewall_MAVLinkFirewall { pub rejected_count: u16 }
    }
    pub mod r2u2_monitor { include!(concat!(env!("OUT_DIR"), "/r2u2_monitor.rs")); }
}
mod reporter;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};
    use bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_api::*;
    use component::seL4_MAVLinkFirewall_MAVLinkFirewall_app::*;
    use open_platform_Data_Model::OperatingMode::{Normal, Recovery};

    struct Capture {
        dispatch: AtomicUsize,
        statuses: Mutex<Vec<(usize, String)>>,
        errors: Mutex<Vec<(usize, log::Level, &'static str)>>,
        reporter: reporter::TimeoutReporter,
    }
    static CAPTURE: Capture = Capture {
        dispatch: AtomicUsize::new(0), statuses: Mutex::new(Vec::new()),
        errors: Mutex::new(Vec::new()), reporter: reporter::TimeoutReporter::new(),
    };
    impl log::Log for Capture {
        fn enabled(&self, _: &log::Metadata) -> bool { true }
        fn log(&self, r: &log::Record) {
            let d = self.dispatch.load(Ordering::Relaxed);
            if self.reporter.observe(r) {
                // Direct sink emission, synchronously inside the generated hook.
                self.errors.lock().unwrap().push((d, log::Level::Error, reporter::TIMEOUT_MESSAGE));
            }
            self.statuses.lock().unwrap().push((d, r.args().to_string()));
        }
        fn flush(&self) {}
    }

    #[test]
    fn generated_monitor_and_reporting_traces() {
        log::set_logger(&CAPTURE).unwrap();
        log::set_max_level(log::LevelFilter::Info);
        // One serialized test: the generated monitor instance is static mut.
        let scenarios = [
            ("timely_D1", Some(2), 1, None),
            ("timely_D2", Some(3), 1, None),
            ("late_D3", Some(4), 1, Some(3)),
            ("never_recovery", None, 1, Some(3)),
            ("delayed_first_assertion", None, 4, Some(6)),
            ("no_assertion", None, 99, None),
            ("already_recovery", Some(0), 1, None),
            ("reboot_timeout_again", None, 1, Some(3)),
        ];
        let mut failures = Vec::new();
        for (name, recovery_at, trigger, expected_error) in scenarios {
            let mut app = seL4_MAVLinkFirewall_MAVLinkFirewall { rejected_count: 0 };
            CAPTURE.reporter.reset();
            CAPTURE.statuses.lock().unwrap().clear();
            CAPTURE.errors.lock().unwrap().clear();
            app.r2u2_monitor_initialize();
            assert!(CAPTURE.statuses.lock().unwrap().is_empty(), "initialization stepped monitor");
            let mut api = seL4_MAVLinkFirewall_MAVLinkFirewall_Application_Api::<Stub> {
                mode: Normal, error: false, marker: core::marker::PhantomData,
            };
            for d in 0..10 {
                CAPTURE.dispatch.store(d, Ordering::Relaxed);
                api.mode = if recovery_at.is_some_and(|at| d >= at) { Recovery } else { Normal };
                app.r2u2_monitor_pre_timeTriggered(&api);
                // This is the application's final output for the sampled dispatch.
                // Four initial rejections, then one at the trigger: reachable count trace.
                app.rejected_count = if d >= trigger { 5 } else { 4 };
                api.error = d >= trigger;
                app.r2u2_monitor_post_timeTriggered(&mut api);
                let actual: Vec<_> = CAPTURE.errors.lock().unwrap().iter().map(|e| e.0).collect();
                let expected: Vec<_> = expected_error.filter(|at| d >= *at).into_iter().collect();
                if actual != expected { failures.push(format!("{name}, dispatch {d}: errors {actual:?}, expected {expected:?}")); }
            }
            println!("{name}: {:?}; verdicts {:?}", CAPTURE.errors.lock().unwrap(), CAPTURE.statuses.lock().unwrap());
        }
        // Prove false input lookalikes and ordinary status logs cannot emit errors.
        CAPTURE.reporter.reset(); CAPTURE.errors.lock().unwrap().clear();
        log::info!("hlr_30_llr_15_16_recovery_deadline is currently false");
        log::info!(target: "probe::component::r2u2_monitor", "hlr_30_llr_15_16_recovery_deadline is currently true");
        assert!(CAPTURE.errors.lock().unwrap().is_empty());
        for _ in 0..3 {
            log::info!(target: "probe::component::r2u2_monitor", "hlr_30_llr_15_16_recovery_deadline is currently false");
        }
        assert_eq!(CAPTURE.errors.lock().unwrap().len(), 1);
        assert!(failures.is_empty(), "generated reporting lost required violations: {failures:#?}");
    }
}

#[cfg(test)]
mod diagnostics {
    #[test]
    fn raw_runtime_verdicts() {
        for trigger in [1, 2, 4] {
            let mut monitor = r2u2_core::Monitor::default();
            r2u2_core::update_binary_file(include_bytes!(concat!(env!("OUT_DIR"), "/spec.bin")), &mut monitor);
            let mut failures_seen = Vec::new();
            for d in 0..10 {
                r2u2_core::load_int_signal(&mut monitor, 0, if d == 0 { 0 } else if d <= trigger { 4 } else { 5 });
                r2u2_core::load_int_signal(&mut monitor, 1, 5);
                r2u2_core::load_bool_signal(&mut monitor, 2, d >= trigger);
                r2u2_core::load_int_signal(&mut monitor, 3, if d >= trigger + 3 { 1 } else { 0 });
                r2u2_core::monitor_step(&mut monitor);
                for out in r2u2_core::get_output_buffer(&monitor) {
                    if !out.verdict.truth { failures_seen.push((d, out.verdict.time)); }
                    println!("trigger={trigger} dispatch={d} runtime_time={} spec={} verdict_time={} truth={}", monitor.time_stamp, out.spec_num, out.verdict.time, out.verdict.truth);
                }
            }
            assert_eq!(failures_seen, vec![(trigger + 2, trigger)], "raw deadline verdict");
        }
    }
}
