use nix::unistd::{Pid, getpid, setpgid};
use std::process::Command;

static mut CHILD_PID: i32 = -1;

fn deploy(cur_dir: &str) {
    let mut child = Command::new("mvn")
        .args(["wildfly:deploy"])
        .current_dir(cur_dir)
        .spawn()
        .expect("Failed to deploy");

    child.wait().unwrap();
}

fn run_clean(cur_dir: &str) {
    let mut child = Command::new("mvn")
        .args(["clean", "package"])
        .current_dir(cur_dir)
        .spawn()
        .expect("Failed to run Maven");

    child.wait().unwrap();

    deploy(cur_dir);
}

pub fn start_server(cur_dir: &str) {
    run_clean(cur_dir);
}
