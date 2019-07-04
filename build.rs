#![allow(unreachable_code, dead_code)]

extern crate bindgen;
extern crate fs_extra;
extern crate num_cpus;
extern crate target_lexicon;

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{self, Path};
use std::process::{Command, Output};
use std::str::FromStr;

use fs_extra::dir;
use fs_extra::error::Result as FsResult;

use target_lexicon::Triple;

/// Prints stuff about error
fn handle_err<A: AsRef<str>>(o: io::Result<Output>, cmd: A) -> Output {
    let o = match o {
        Err(e) => {
            println!("{}", cmd.as_ref());
            println!("\tIO Error on exec:\n{:?}", e);
            ::std::process::exit(1);
        }
        Ok(o) => o,
    };
    if !o.status.success() {
        let stderr = String::from_utf8_lossy(o.stderr.as_slice());
        let stdout = String::from_utf8_lossy(o.stdout.as_slice());
        println!("{}", cmd.as_ref());
        match o.status.code() {
            Option::Some(x) => println!("\tExit Code: {:?}", x),
            _ => {}
        };
        println!("\tStdErr:\n {}", stderr);
        println!("\tStdOut:\n {}", stdout);
        ::std::process::exit(1);
    }
    o
}

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

fn copy_dir_if_needed<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dest: P2) -> FsResult<u64> {
    dir::copy(
        src,
        dest,
        &dir::CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..dir::CopyOptions::new()
        },
    )
}

/// Autogenerates bindings
fn build_bindings() -> Result<(), Box<dyn Error>> {
    let out_dir = path::PathBuf::from(env::var("OUT_DIR")?);

    let mut include_dir = out_dir.clone();
    include_dir.push("install");
    include_dir.push("include");

    for job in BINDGEN_JOBS {
        let mut dot_h = out_dir.clone();
        dot_h.push(job.0);

        let mut dot_rs = out_dir.clone();
        dot_rs.push(job.1);

        let bindings = match bindgen::Builder::default()
            .clang_arg(format!("--include-directory={}", include_dir.display()))
            .clang_arg("-DXED_ENCODER")
            .clang_arg("-fkeep-inline-functions")
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

/// Build script entry point
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=xed/VERSION");

    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy xed directory
    copy_dir_if_needed("xed", &{
        let mut x = out_dir.clone();
        x.push("xed");
        x
    })?;

    // Copy mbuild directory
    copy_dir_if_needed("mbuild", &{
        let mut x = out_dir.clone();
        x.push("mbuild");
        x
    })?;

    // Create install directory
    let install_dir = {
        let mut x = out_dir.clone();
        x.push("install");
        x
    };
    create_dir(&install_dir)?;

    let python_check = Command::new("python").arg("-V").output();
    handle_err(python_check, "Python is required to build xed.");

    // Set locale to C to avoid user language setting interference
    env::set_var("LC_ALL", "C");

    // Set current directory to OUR_DIR
    env::set_current_dir(&out_dir)?;

    // Build xed
    let output = Command::new("python")
        .arg("mfile.py")
        .arg(format!("--jobs={}", num_cpus::get()))
        .arg("--silent")
        .arg("--static-stripped")
        .arg("--extra-ccflags=-fPIC")
        .arg("--opt=3")
        .arg("--no-werror")
        .arg(format!(
            "--host-cpu={}",
            Triple::from_str(&env::var("TARGET").unwrap())
                .unwrap()
                .architecture
        ))
        .arg(format!("--install-dir={}", install_dir.display()))
        .arg("install")
        .current_dir("xed")
        .output();
    handle_err(output, "Failed to run `mfile.py`");

    // Configure linker
    let lib_dir = {
        let mut x = install_dir.clone();
        x.push("lib");
        x
    };
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=xed");

    // Generate bindings
    build_bindings()?;

    Ok(())
}
