use std::os::unix::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;

use crate::db::{db_init, get_path};

pub static CHILD_PID: Mutex<Vec<i32>> = Mutex::new(Vec::new());

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

fn run_standalone(path: &str, offset: u64) {
    let bin_path = format!("{}/bin", path);

    println!("PATH: {:?}", bin_path);

    let child = unsafe {
        Command::new("./standalone.sh")
            .arg(format!("-Djboss.socket.binding.port-offset={}", offset))
            .current_dir(&bin_path)
            .pre_exec(|| {
                let result = libc::setpgid(0, 0);

                if result != 0 {
                    return Err(std::io::Error::last_os_error());
                }

                Ok(())
            })
            .spawn()
            .expect("Failed to run standalone.sh")
    };

    CHILD_PID.lock().unwrap().push(child.id() as i32);
}

pub async fn start_server(cur_dir: &str, key: &str, offset: u64, first_run: i32) {
    if (cfg!(target_os = "macos") || cfg!(target_os = "linux")) && first_run == 1 {
        let pool = db_init().await.unwrap();

        let wfly_path = match get_path(&pool, key).await {
            Ok(p) => p,

            Err(e) => {
                println!("Error in gettig the Wildfly Path in start_server\n\n");

                println!("ERROR: {:?}", e);

                return;
            }
        };

        run_standalone(&wfly_path, offset);
    }

    if !cur_dir.is_empty() {
        run_clean(cur_dir);
    }
}
