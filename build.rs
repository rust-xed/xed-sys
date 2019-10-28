use std::{
    env,
    error::Error,
    fs, io,
    path::{self, Path, PathBuf},
    process::Command,
    str::FromStr,
};

use target_lexicon::Triple;

const BINDGEN_JOBS: &'_ [(&'_ str, &'_ str)] = &[
    ("install/include/xed/xed-interface.h", "xed_interface.rs"),
    ("install/include/xed/xed-version.h", "xed_version.rs"),
];

fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir(path).or_else(|e| match e.kind() {
        io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}

/// Autogenerates bindings
fn build_bindings() -> Result<(), Box<dyn Error>> {
    let out_dir = path::PathBuf::from(env::var("OUT_DIR")?);

    let mut include_dir = out_dir.clone();
    include_dir.push("install");
    include_dir.push("include");

    for job in BINDGEN_JOBS {
        let dot_h = out_dir.join(job.0);
        let dot_rs = out_dir.join(job.1);

        let bindings = match bindgen::Builder::default()
            .clang_arg(format!("--include-directory={}", include_dir.display()))
            .clang_arg("-DXED_ENCODER")
            .clang_arg("-DXED_DECODER")
            .whitelist_type("xed3?_.*")
            .whitelist_function("(str2)?xed3?_.*")
            .whitelist_var("(XED|xed)_.*")
            .header(format!("{}", dot_h.display()))
            .impl_debug(true)
            .derive_copy(true)
            .derive_debug(true)
            .prepend_enum_name(false)
            .generate_inline_functions(true)
            .generate()
        {
            Ok(x) => x,
            Err(e) => panic!(
                "Could not generate bindings for {}. Error {:?}",
                dot_h.display(),
                e
            ),
        };
        match bindings.write_to_file(&dot_rs) {
            Ok(_) => {}
            Err(e) => panic!(
                "Could not write generated bindings to {}. Error {:?}",
                dot_rs.display(),
                e
            ),
        };
    }

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=xed/VERSION");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed-env=OUT_DIR");
    println!("cargo:rerun-if-changed-env=TARGET");
    println!("cargo:rerun-if-changed-env=PROFILE");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to read OUT_DIR"));
    let cwd = env::current_dir().expect("Failed to get CWD");
    let target = env::var("TARGET").expect("Failed to read TARGET");
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
    let mfile_path = cwd.join("xed/mfile.py");

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

    let num_jobs: u32 = env::var("NUM_JOBS").unwrap().parse().unwrap_or(1);

    let mut cmd = Command::new("python");
    cmd.arg(&mfile_path)
        .arg(format!("--jobs={}", num_jobs))
        .arg("--silent")
        .arg("--static-stripped")
        .arg("--extra-ccflags=-fPIC")
        .arg("--no-werror")
        .arg(format!(
            "--host-cpu={}",
            Triple::from_str(&target)
                .expect("TARGET was not a valid target triple")
                .architecture
        ))
        .arg(format!("--install-dir={}", install_dir.display()));

    if profile == "release" {
        cmd.arg("--opt=3");
    } else {
        cmd.arg("--opt=0");
    }

    cmd.arg("install").current_dir(&build_dir);

    eprintln!("XED build command: {:?}", cmd);

    let status = cmd.status().expect("Failed to start xed build");

    if !status.success() {
        panic!("Building xed failed");
    }

    let lib_dir = install_dir.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=xed");

    // Generate bindings
    build_bindings().expect("Failed to generate bindings");
}
