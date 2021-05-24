extern crate assert_cmd;

use std::io::{BufRead, BufReader};
use assert_cmd::Command;
use assert_cmd::output::OutputResult;
use std::{fs, process, thread, time};

#[cfg(test)]
mod integration {
    use super::*;
    fn exec_analyzer_command(cid: &str, args: &[&str], with_annotations: Option<bool>) -> OutputResult {
        let input = if let Some(annots) = with_annotations {
            format!("{{\"pid\":{},\"id\":\"{}\",\"annotations\":{{\"oci-ftrace-syscall-analyzer/trace\":\"{}\"}}}}", process::id(), cid, annots)
        } else {
            format!("{{\"pid\":{},\"id\":\"{}\"}}", process::id(), cid)
        };
        Command::cargo_bin("oci-ftrace-syscall-analyzer")
            .unwrap()
            .args(args)
            .write_stdin(input)
            .ok()
    }

    fn remove_trace_dir(trace_path: &str) {
        let mut retry = 10;
        while let Err(_) = fs::remove_dir(&trace_path) {
            retry -= 1;
            if retry == 0 {
                break;
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    macro_rules! get_cid {
        ($test_id:expr) => {{
            &format!("analyzer_test_{}_{}", $test_id, process::id())
        }};
    }

    macro_rules! get_tracefs_path {
        ($cid:expr) => {{
            &format!("/sys/kernel/debug/tracing/instances/{}", $cid)
        }};
    }

    #[test]
    fn trace_start() {
        let cid = get_cid!("record_1");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record"], None).is_ok());
        assert!(fs::metadata(&trace_path).is_ok());
        assert_eq!(
            "1\n",
            String::from_utf8(
                fs::read(&format!("{}/events/syscalls/enable", &trace_path)).unwrap()
            )
            .unwrap()
        );
        remove_trace_dir(&trace_path);
    }

    #[test]
    fn set_buffer_size() {
        let cid = get_cid!("record_2");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record", "--buffer-size-kb", "999"], None).is_ok());
        assert!(fs::metadata(&trace_path).is_ok());
        assert_eq!(
            "999\n",
            String::from_utf8(fs::read(&format!("{}/buffer_size_kb", &trace_path)).unwrap())
                .unwrap()
        );
        remove_trace_dir(&trace_path);
    }

    #[test]
    fn syscall_filter() {
        let cid = get_cid!("record_3");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record", "--syscalls", "openat"], None).is_ok());
        assert!(fs::metadata(&trace_path).is_ok());
        assert_eq!(
            "X\n",
            String::from_utf8(
                fs::read(&format!("{}/events/syscalls/enable", &trace_path)).unwrap()
            )
            .unwrap()
        );
        assert_eq!(
            "1\n",
            String::from_utf8(
                fs::read(&format!(
                    "{}/events/syscalls/sys_enter_openat/enable",
                    &trace_path
                ))
                .unwrap()
            )
            .unwrap()
        );
        assert_eq!(
            "1\n",
            String::from_utf8(
                fs::read(&format!(
                    "{}/events/syscalls/sys_exit_openat/enable",
                    &trace_path
                ))
                .unwrap()
            )
            .unwrap()
        );
        assert_eq!(
            "0\n",
            String::from_utf8(
                fs::read(&format!(
                    "{}/events/syscalls/sys_enter_open/enable",
                    &trace_path
                ))
                .unwrap()
            )
            .unwrap()
        );
        assert_eq!(
            "0\n",
            String::from_utf8(
                fs::read(&format!(
                    "{}/events/syscalls/sys_exit_open/enable",
                    &trace_path
                ))
                .unwrap()
            )
            .unwrap()
        );
        remove_trace_dir(&trace_path);
    }

    #[test]
    fn trace_start_with_annotations() {
        let cid = get_cid!("record_4");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record", "--use-annots"], Some(true)).is_ok());
        assert!(fs::metadata(&trace_path).is_ok());
        assert_eq!(
            "1\n",
            String::from_utf8(
                fs::read(&format!("{}/events/syscalls/enable", &trace_path)).unwrap()
            )
            .unwrap()
        );
        remove_trace_dir(&trace_path);
    }

    #[test]
    fn trace_does_not_start_with_use_annots() {
        let cid = get_cid!("record_5");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record", "--use-annots"], None).is_ok());
        assert!(fs::metadata(&trace_path).is_err());
    }

    #[test]
    fn trace_does_not_start_with_annotations_false() {
        let cid = get_cid!("record_6");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record", "--use-annots"], Some(false)).is_ok());
        assert!(fs::metadata(&trace_path).is_err());
    }

    #[test]
    fn error_filter() {
        let cid = get_cid!("record_7");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record", "--error-only"], None).is_ok());
        assert!(fs::metadata(&trace_path).is_ok());
        assert_eq!(
            "ret < 0\n",
            String::from_utf8(
                fs::read(&format!(
                    "{}/events/syscalls/sys_exit_open/filter",
                    &trace_path
                ))
                .unwrap()
            )
            .unwrap()
        );
        remove_trace_dir(&trace_path);
    }

    #[test]
    fn report() {
        let cid = get_cid!("report_1");
        assert!(exec_analyzer_command(cid, &["record"], None).is_ok());
        let log = &"./ftrace_syscalls_dump_1.log";
        assert!(exec_analyzer_command(cid, &["report", "--output", log], None).is_ok());
        assert!(fs::metadata(log).is_ok());
        let mut buf = String::new();
        BufReader::new(fs::File::open(&log).unwrap())
            .read_line(&mut buf)
            .unwrap();
        assert!(buf.contains("tracer"));
        fs::remove_file(log).unwrap();
    }

    #[test]
    fn report_livedump() {
        let cid = get_cid!("report_2");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record"], None).is_ok());
        let log = &"./ftrace_syscalls_dump_2.log";
        assert!(exec_analyzer_command(cid, &["report", "--output", log, "--livedump", cid], None).is_ok());
        assert!(fs::metadata(log).is_ok());
        let mut buf = String::new();
        BufReader::new(fs::File::open(&log).unwrap())
            .read_line(&mut buf)
            .unwrap();
        assert!(buf.contains("tracer"));
        remove_trace_dir(&trace_path);
        fs::remove_file(log).unwrap();
    }

    #[test]
    fn report_profile() {
        let cid = get_cid!("report_3");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record"], None).is_ok());
        let log = &"./ftrace_syscalls_dump_3.log";
        let profile = &"./seccomp_profile_3.json";
        assert!(exec_analyzer_command(
            cid,
            &["report", "--output", log, "--seccomp-profile", profile],
            None,
        ).is_ok());
        assert!(fs::metadata(log).is_ok());
        assert!(fs::metadata(profile).is_ok());
        let mut buf = String::new();
        BufReader::new(fs::File::open(&log).unwrap())
            .read_line(&mut buf)
            .unwrap();
        assert!(buf.contains("tracer"));
        let content = fs::read_to_string(&profile).unwrap();
        assert!(content.contains("defaultAction"));
        assert!(content.contains("syscalls"));
        remove_trace_dir(&trace_path);
        fs::remove_file(log).unwrap();
        fs::remove_file(profile).unwrap();
    }

    #[test]
    fn report_conflict_between_livedump_and_profile() {
        let cid = get_cid!("report_4");
        let trace_path = get_tracefs_path!(cid);
        assert!(exec_analyzer_command(cid, &["record"], None).is_ok());
        let log = &"./ftrace_syscalls_dump_4.log";
        let profile = &"./seccomp_profile_4.json";
        assert!(exec_analyzer_command(cid, &["report", "--seccomp-profile", profile, "--livedump", cid], None).is_err());
        assert!(fs::metadata(log).is_err());
        assert!(fs::metadata(profile).is_err());
        remove_trace_dir(&trace_path);
    }
}
