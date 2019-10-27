use std::{
    env,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir(path).or_else(|e| match e.kind() {
        io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to read OUT_DIR"));
    let cwd = env::current_dir().expect("Failed to get CWD");
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_owned());

    // Ensure python exists and can run
    match Command::new("python").arg("-V").output() {
        Ok(output) => {
            if !output.status.success() {
                panic!("'python -V' returned with an error");
            }
        }
        Err(e) => {
            panic!("Python is required to run xed: {}", e);
        }
    }

    let install_dir = out_dir.join("install");
    let build_dir = out_dir.join("build");
    let mfile_path = cwd.join("../xed/mfile.py");

    create_dir(&install_dir).expect(&format!(
        "Failed to create directory '{}'",
        install_dir.display()
    ));
    create_dir(&build_dir).expect(&format!(
        "Failed to create directory '{}'",
        build_dir.display()
    ));

    // Set locale to C to avoid user language settings interference
    env::set_var("LC_ALL", "C");

    env::set_current_dir(&build_dir).unwrap();

    let num_jobs: u32 = env::var("NUM_JOBS")
        .unwrap_or("1".to_owned())
        .parse()
        .unwrap_or(1);

    let mut cmd = Command::new("python");
    cmd.arg(&mfile_path)
        .arg(format!("--jobs={}", num_jobs))
        .arg("--silent")
        .arg("--static-stripped")
        .arg("--extra-ccflags=-fPIC")
        .arg("--no-werror")
        .arg(format!("--install-dir={}", install_dir.display()))
        .arg("--knc");
        //.arg("--knl");

    if profile == "release" {
        cmd.arg("--opt=3");
    } else {
        cmd.arg("--opt=0");
    }

    cmd.arg("install").current_dir(&build_dir);

    let status = cmd.status().expect("Failed to start xed build");

    if !status.success() {
        panic!("Building xed failed");
    }
}
