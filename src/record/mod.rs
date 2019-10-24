use super::utils;
use clap::ArgMatches;
use std::fs;
use std::process;

fn start_syscall_logging(trace_setting_path: &str, pid: &str, bufsize: &str, syscall_list: Option<clap::Values>) {
    change_syscall_enable_all(trace_setting_path, false);
    enable_forktrace(trace_setting_path);
    if bufsize != "" {
        change_bufsize(trace_setting_path, bufsize);
    }
    match syscall_list {
        Some(syscalls) => filter_by_syscalls(trace_setting_path, syscalls),
        None           => change_syscall_enable_all(trace_setting_path, true),
    };
    filter_by_pid(trace_setting_path, pid);
    clear_ringbuf(trace_setting_path);
}

fn change_syscall_enable_all(trace_setting_path: &str, enable: bool) {
    let syscall_enable = format!("{}/events/syscalls/enable", &trace_setting_path);
    match enable {
        false => fs::write(&syscall_enable, "0")
            .unwrap_or_else(|_| panic!("Failed to write to {}", &syscall_enable)),
        true  => fs::write(&syscall_enable, "1")
            .unwrap_or_else(|_| panic!("Failed to write to {}", &syscall_enable)),
    };
}

fn enable_forktrace(trace_setting_path: &str) {
    let event_fork = format!("{}/options/event-fork", &trace_setting_path);
    fs::write(&event_fork, "1").unwrap_or_else(|_| panic!("Failed to write to {}", &event_fork));
}

fn change_bufsize(trace_setting_path: &str, bufsize: &str) {
    let syscall_bufsize = format!("{}/buffer_size_kb", &trace_setting_path);
    fs::write(&syscall_bufsize, bufsize)
        .unwrap_or_else(|_| panic!("Failed to write to {}", &syscall_bufsize));
}

fn clear_ringbuf(trace_setting_path: &str) {
    let sys_trace = format!("{}/trace", &trace_setting_path);
    fs::write(&sys_trace, "").unwrap_or_else(|_| panic!("Failed to write to {}", &sys_trace));
}

fn filter_by_pid(trace_setting_path: &str, pid: &str) {
    let pid_filter = format!("{}/set_event_pid", &trace_setting_path);
    fs::write(&pid_filter, pid.to_string())
        .unwrap_or_else(|_| panic!("Failed to write to {}", &pid_filter));
}

fn filter_by_syscalls(trace_setting_path: &str, syscalls: clap::Values) {
    for syscall in syscalls {
        let syscalls_enter = format!(
            "{}/events/syscalls/sys_enter_{}/enable",
            &trace_setting_path, &syscall
        );
        fs::write(&syscalls_enter, "1").expect(&format!("Failed to write to {}", &syscalls_enter));
        let syscalls_exit = format!(
            "{}/events/syscalls/sys_exit_{}/enable",
            &trace_setting_path, &syscall
        );
        fs::write(&syscalls_exit, "1").expect(&format!("Failed to write to {}", &syscalls_exit));
    };
}

pub fn record(record_args: &ArgMatches) {
    let state_vals = utils::get_states_from_stdin();

    let pid = state_vals["pid"].to_string();
    if pid == "" {
        panic!("cannot find pid");
    }

    let cid = state_vals["id"].to_string();
    if cid == "" {
        panic!("cannot find container id");
    }

    let enable_trace = state_vals["annotations"]["oci-ftrace-syscall-analyzer/trace"] == "true"
        || !record_args.is_present("use-annots");

    if !enable_trace {
        process::exit(0);
    }

    let tracefs_path =
        utils::search_tracefs_path().unwrap_or_else(|_| panic!("Failed to search tracefs"));
    let trace_path = format!(
        "{}/instances/{}",
        &tracefs_path,
        &cid.trim_matches('\\').trim_matches('"')
    );
    if let Err(error) = fs::create_dir(trace_path.to_string()) {
        panic!("{}: {}", trace_path, error)
    };

    let bufsize = record_args
        .value_of("buffer-size-kb")
        .unwrap_or("")
        .to_string();
    start_syscall_logging(
        &trace_path,
        &pid.trim_matches('\\').trim_matches('"'),
        &bufsize,
        record_args.values_of("syscalls"),
    );
}
