
extern crate bindgen;
extern crate fs_extra;
extern crate num_cpus;

use std::process::{Command,Output};
use std::io;
use std::env;
use std::fs;
use std::path::{self, Path};
use std::error::Error;

use fs_extra::dir;
use fs_extra::error::Result as FsResult;

/// Prints stuff about error
fn handle_err<A: AsRef<str>>(o: io::Result<Output>, cmd: A) -> Output {
    let o = match o {
        Err(e) => {
            println!("{}", cmd.as_ref());
            println!("\tIO Error on exec:\n{:?}",e);
            ::std::process::exit(1);
        }
        Ok(o) => o 
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

const BINDGEN_JOBS: &'static [(&'static str, &'static str)] = &[
    ("xed/include/public/xed/xed-interface.h", "../xed_interface.rs"),
    ("xed/include/public/xed/xed-version.h", "../xed_version.rs"),
];

fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    match fs::create_dir(path) {
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e)
        },
        x => x,
    }
}

fn overwrite_dir<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dest: P2) -> FsResult<u64> {
    dir::copy(src, dest, &dir::CopyOptions{
        overwrite: true,
        skip_exist: true,
        copy_inside: true,
        ..dir::CopyOptions::new()
    })
}

/// Autogenerates bindings
fn build_bindings() {
    for job in BINDGEN_JOBS {
        let dot_h = job.0;
        let dot_rs = job.1;
        let bindings = match bindgen::Builder::default()
            .clang_arg("--include-directory=xed/obj")
            .clang_arg("--include-directory=xed/include/public/xed")
            .header(dot_h)
            .generate() {
            Ok(x) => x,
            Err(e) => panic!("Could not generate bindings for {}. Error {:?}", dot_h, e)
        };
        match bindings.write_to_file(dot_rs) {
            Ok(_) => {}
            Err(e) => panic!("Could not write generated bindings to {}. Error {:?}", dot_rs, e)
        };
    }
}

/// Build script entry point
fn main() -> Result<(), Box<Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
   
    // linker directory
    let current_dir = env::current_dir().expect("Could not fetch current directory");
    let lib_dir = {
        let mut x = current_dir.clone();
        x.push("xed");
        x.push("build");
        x.push("obj");
        x
    };
    let xed_dir = {
        let mut x = current_dir.clone();
        x.push("xed");
        x
    };
    let mbuild_dir = {
        let mut x = current_dir.clone();
        x.push("mbuild");
        x
    };

    let mut new_dir = path::PathBuf::from(&out_dir);
    new_dir.push("xed-build");
    dir::remove(new_dir.clone()).err();
    create_dir(new_dir.clone())?;

    new_dir.push("mbuild");
    eprintln!("{:?}", new_dir);
    overwrite_dir(mbuild_dir, new_dir.clone())?;

    new_dir.pop();
    new_dir.push("xed");
    overwrite_dir(xed_dir, new_dir.clone())?;
    
    new_dir.pop();
    env::set_current_dir(new_dir.clone())?;

    eprintln!("NewDir: {:?}", new_dir);

    // Build the project
    let output = Command::new("python")
        .arg("mfile.py")
        .arg(format!("--jobs={}", num_cpus::get()))
        .arg("--silent")
        .arg("--static-stripped")
        //.arg("--extra-ccflags=-fPIC")
        .arg("--opt=3")
        .arg("--no-werror")
        //.arg("--elf-dwarf")
        //.arg("--cc=/usr/bin/clang")
        //.arg("--cxx=/usr/bin/clang++")
        //.arg("--ar=/usr/bin/ar")
        //.arg("--yasm")
        //.arg("--linker=/usr/bin/ld")
        //.arg("--compiler=clang")
        .current_dir("xed")
        .output();
    handle_err(output, "Failed to run `mfile.py`");

    // Configure linker
    println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
    println!("cargo:rustc-link-lib=static=xed");

    // auto generate bindings
    build_bindings();

    Ok(())
}

