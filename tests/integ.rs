use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::{fs, process, thread, time};

#[cfg(test)]
mod integration {
    use super::*;
    fn exec_analyzer_command(
        test_id: &str,
        args: &[&str],
        with_annotations: Option<bool>,
    ) -> String {
        let mut child = Command::new("./target/debug/oci-ftrace-syscall-analyzer")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let cid = &format!("analyzer_test_{}_{}", test_id, process::id());
        let input = if let Some(annots) = with_annotations {
            format!("{{\"pid\":{},\"id\":\"{}\",\"annotations\":{{\"oci-ftrace-syscall-analyzer/trace\":\"{}\"}}}}", process::id(), cid, annots)
        } else {
            format!("{{\"pid\":{},\"id\":\"{}\"}}", process::id(), cid)
        };
        print!("{}", &input);
        let stdin = child.stdin.as_mut().expect("stdin error");
        stdin.write(input.as_bytes()).unwrap();
        let ecode = child.wait().expect("failed to wait on child");
        assert!(ecode.success());
        format!("/sys/kernel/debug/tracing/instances/{}", cid)
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

    #[test]
    fn trace_start() {
        let trace_path = exec_analyzer_command("record_1", &["record"], None);
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
        let trace_path =
            exec_analyzer_command("record_2", &["record", "--buffer-size-kb", "999"], None);
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
        let trace_path =
            exec_analyzer_command("record_3", &["record", "--syscalls", "openat"], None);
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
        let trace_path = exec_analyzer_command("record_4", &["record", "--use-annots"], Some(true));
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
        let trace_path = exec_analyzer_command("record_5", &["record", "--use-annots"], None);
        assert!(fs::metadata(&trace_path).is_err());
    }

    #[test]
    fn trace_does_not_start_with_annotations_false() {
        let trace_path =
            exec_analyzer_command("record_6", &["record", "--use-annots"], Some(false));
        assert!(fs::metadata(&trace_path).is_err());
    }

    #[test]
    fn report() {
        exec_analyzer_command("report_1", &["record"], None);
        let log = &"./ftrace_syscalls_dump_1.log";
        exec_analyzer_command("report_1", &["report", "--output", log], None);
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
        let trace_path = exec_analyzer_command("report_2", &["record"], None);
        let cid = &format!("analyzer_test_report_2_{}", process::id());
        let log = &"./ftrace_syscalls_dump_2.log";
        exec_analyzer_command(
            "report_2",
            &["report", "--output", log, "--livedump", cid],
            None,
        );
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
        let trace_path = exec_analyzer_command("report_3", &["record"], None);
        let log = &"./ftrace_syscalls_dump_3.log";
        let profile = &"./seccomp_profile_3.json";
        exec_analyzer_command(
            "report_3",
            &["report", "--output", log, "--seccomp-profile", profile],
            None,
        );
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
        let trace_path = exec_analyzer_command("report_4", &["record"], None);
        let cid = &format!("analyzer_test_report_4_{}", process::id());
        let log = &"./ftrace_syscalls_dump_4.log";
        let profile = &"./seccomp_profile_4.json";
        let mut child = Command::new("./target/debug/oci-ftrace-syscall-analyzer")
            .args(&["report", "--seccomp-profile", profile, "--livedump", cid])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let ecode = child.wait().expect("failed to wait on child");
        assert_eq!(ecode.code().unwrap(), 255);
        assert!(fs::metadata(log).is_err());
        assert!(fs::metadata(profile).is_err());
        remove_trace_dir(&trace_path);
    }
}
