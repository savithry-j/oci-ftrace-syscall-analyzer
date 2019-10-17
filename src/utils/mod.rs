use proc_mounts::MountIter;
use serde_json::Value;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn get_states_from_stdin() -> Value {
    let states = {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let state_vals: Value = serde_json::from_str(&states).unwrap();
    state_vals
}

pub fn search_tracefs_path() -> String {
    let mut path;
    path = format!("{:#?}", "");
    for mount_res in MountIter::new().unwrap() {
        let mount = mount_res.unwrap();
        if mount.source == PathBuf::from("debugfs") {
            path = format!("{}/tracing", mount.dest.display());
            break;
        }
        if mount.source == PathBuf::from("tracefs") {
            path = format!("{}", mount.dest.display());
            break;
        }
    }

    if path == "" {
        panic!("cannot find tracefs");
    }
    path.to_string()
}
