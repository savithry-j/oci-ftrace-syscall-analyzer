use super::utils;
use clap::ArgMatches;
use std::fs;
use std::process;

fn start_syscall_logging(trace_setting_path: &str, pid: &str, bufsize: &str) {
    enable_syscall_all(trace_setting_path);
    enable_forktrace(trace_setting_path);
    if bufsize != "" {
        change_bufsize(trace_setting_path, bufsize);
    }
    clear_ringbuf(trace_setting_path);
    filter_by_pid(trace_setting_path, pid);
}

fn enable_syscall_all(trace_setting_path: &str) {
    let syscall_enable = format!("{}/events/syscalls/enable", &trace_setting_path);
    fs::write(&syscall_enable, "0").expect(&format!("Failed to write to {}", &syscall_enable));
    fs::write(&syscall_enable, "1").expect(&format!("Failed to write to {}", &syscall_enable));
}

fn enable_forktrace(trace_setting_path: &str) {
    let event_fork = format!("{}/options/event-fork", &trace_setting_path);
    fs::write(&event_fork, "1").expect(&format!("Failed to write to {}", &event_fork));
}

fn change_bufsize(trace_setting_path: &str, bufsize: &str) {
    let syscall_bufsize = format!("{}/buffer_size_kb", &trace_setting_path);
    fs::write(&syscall_bufsize, bufsize)
        .expect(&format!("Failed to write to {}", &syscall_bufsize));
}

fn clear_ringbuf(trace_setting_path: &str) {
    let sys_trace = format!("{}/trace", &trace_setting_path);
    fs::write(&sys_trace, "").expect(&format!("Failed to write to {}", &sys_trace));
}

fn filter_by_pid(trace_setting_path: &str, pid: &str) {
    let pid_filter = format!("{}/set_event_pid", &trace_setting_path);
    fs::write(&pid_filter, pid.to_string()).expect(&format!("Failed to write to {}", &pid_filter));
}

pub fn record(record_args: &ArgMatches) {
    let state_vals = utils::get_states_from_stdin();

    let pid = format!("{}", state_vals["pid"].to_string());
    if pid == "" {
        panic!("cannot find pid");
    }

    let cid = format!("{}", state_vals["id"].to_string());
    if cid == "" {
        panic!("cannot find container id");
    }

    let mut enable_trace = false;
    if state_vals["annotations"]["oci-ftrace-syscall-analyzer/trace"] == "true"
        || !record_args.is_present("use-annots")
    {
        enable_trace = true;
    }
    if !enable_trace {
        process::exit(0);
    }

    let tracefs_path = utils::search_tracefs_path();
    let trace_path = format!(
        "{}/instances/{}",
        &tracefs_path,
        &cid.trim_matches('\\').trim_matches('"')
    );
    match fs::create_dir(trace_path.to_string()) {
        Err(error) => panic!("{}: {}", trace_path, error),
        Ok(_) => {}
    }

    let bufsize = record_args
        .value_of("buffer-size-kb")
        .unwrap_or("")
        .to_string();
    start_syscall_logging(
        &trace_path,
        &pid.trim_matches('\\').trim_matches('"'),
        &bufsize,
    );
}
