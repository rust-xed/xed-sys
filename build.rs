use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use target_lexicon::{OperatingSystem, Triple};

fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir(path).or_else(|e| match e.kind() {
        io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}

fn find_python() -> Command {
    // Check for pylauncher first since it is the most accurate way to get the
    // newest version of python.
    //
    // This is more relevant on windows but it can also be installed on linux.
    if let Ok(status) = Command::new("py").arg("-3").arg("-V").status() {
        if status.success() {
            let mut cmd = Command::new("py");
            cmd.arg("-3");
            return cmd;
        }
    }

    // Next check for an explicit python3 installation.
    if let Ok(status) = Command::new("python3").arg("-V").status() {
        if status.success() {
            return Command::new("python3");
        }
    }

    // Finally just try and run python. If the version is
    // too old (e.g. python generally means python2 on linux)
    // then we'll fail later on. On windows it's generally
    // likely to see python as just plain python.
    if let Ok(status) = Command::new("python").arg("-V").status() {
        if status.success() {
            return Command::new("python");
        }
    }

    panic!("Unable to find a working python installation. Tried `python3` and `python`.");
}

fn build_xed(_cc: &cc::Build) {
    println!("cargo:rerun-if-changed=xed/VERSION");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed-env=PROFILE");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to read OUT_DIR"));
    let cwd = env::current_dir().expect("Failed to get CWD");
    let target = env::var("TARGET").expect("Failed to read TARGET");
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_owned());
    let debug = match env::var("DEBUG") {
        Ok(var) => var != "false",
        _ => false,
    };

    let triple = Triple::from_str(&target).expect("TARGET was not a valid target triple");

    let install_dir = out_dir.join("install");
    let build_dir = out_dir.join("build");
    let mfile_path = cwd.join("xed/mfile.py");

    create_dir(&install_dir)
        .unwrap_or_else(|_| panic!("Failed to create directory '{}'", install_dir.display()));
    create_dir(&build_dir)
        .unwrap_or_else(|_| panic!("Failed to create directory '{}'", build_dir.display()));

    // Set locale to C to avoid user language settings interference
    env::set_var("LC_ALL", "C");

    let num_jobs: u32 = env::var("NUM_JOBS")
        .unwrap_or_else(|_| "1".to_owned())
        .parse()
        .unwrap_or(1);

    let mut cmd = find_python();
    cmd.env("PYTHONDONTWRITEBYTECODE", "x")
        .current_dir(&build_dir)
        // The -B flag prevents python from generating .pyc files
        .arg("-B")
        .arg(&mfile_path)
        .arg("install")
        .arg(format!("--jobs={}", num_jobs))
        .arg("--extra-ccflags=-fPIC")
        .arg("--no-werror")
        .arg(format!("--host-cpu={}", triple.architecture))
        .arg(format!("--install-dir={}", install_dir.display()));

    if cfg!(feature = "dylib") {
        cmd.arg("--shared");
    } else {
        cmd.arg("--static-stripped");
    }

    if debug {
        cmd.arg("--debug");
    }

    if profile == "release" {
        cmd.arg("--opt=3");
    } else {
        cmd.arg("--opt=0");
    }

    if cfg!(feature = "_internal-enc2") {
        cmd.arg("--enc2");
    }

    eprintln!("XED build command: {:?}", cmd);

    let status = cmd.status().expect("Failed to start xed build");

    if !status.success() {
        panic!("Building xed failed");
    }

    let lib_dir = install_dir.join("lib");
    let bin_dir = install_dir.join("bin");
    let linkty = match cfg!(feature = "dylib") {
        true => "dylib",
        false => "static",
    };

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib={linkty}=xed");

    if triple.operating_system == OperatingSystem::Windows {
        println!("cargo:rustc-link-search=native={}", bin_dir.display());
    }

    macro_rules! cfg_link_enc2 {
        ($variant:literal) => {
            if cfg!(feature = $variant) {
                println!("cargo:rustc-link-lib={linkty}=xed-{}", $variant);

                if cfg!(feature = "enc2-chk") {
                    println!("cargo:rustc-link-lib={linkty}=xed-chk-{}", $variant);
                }
            }
        };
    }

    cfg_link_enc2!("enc2-m32-a32");
    cfg_link_enc2!("enc2-m64-a64");

    #[cfg(not(feature = "dylib"))]
    #[cfg(all(feature = "enc2-m32-a32", feature = "enc2-m64-a64"))]
    compile_error!(concat!(
        "Cannot enable both `enc2-m32-a32` and `enc2-m64-a64` features when linking statically.",
        "You can avoid this by enabling the `dylib` feature to build these as dylibs."
    ));
}
fn build_inline_shim(mut cc: cc::Build) {
    let cwd = std::env::current_dir().unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to read OUT_DIR"));

    cc.include(out_dir.join("install/include"))
        .include(&cwd)
        // xed-static.c contains an instance of this. It's not an error and we
        // don't want to be modifying generated files so just silence the warning.
        .flag_if_supported("-Wno-duplicate-decl-specifier");

    macro_rules! cfg_define_enc2 {
        ($variant:literal) => {
            if cfg!(feature = $variant) {
                let def = $variant.replace('-', "_").to_ascii_uppercase();
                cc.define(&format!("XED_SYS_{def}"), None);
            }
        };
    }

    cfg_define_enc2!("enc2-m32-a32");
    cfg_define_enc2!("enc2-m64-a64");

    if cfg!(feature = "enc2-chk") {
        cc.define("XED_SYS_ENC2_CHK", None);
    }

    if cfg!(feature = "dylib") {
        cc.define("XED_DLL", None);
    }

    if cfg!(feature = "bindgen") {
        cc.file(out_dir.join("xed-static.c"));
    } else {
        cc.file("src/xed-static.c");
    }

    cc.compile("xed-shim");
}

#[cfg(feature = "bindgen")]
fn build_bindgen() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to read OUT_DIR"));

    let dot_rs = out_dir.join("xed.rs");

    let mut builder = bindgen::builder()
        .clang_arg("-I")
        .clang_arg(out_dir.join("install/include").display().to_string())
        .allowlist_type("xed3?_.*")
        .allowlist_function("(str2)?xed3?_.*")
        .allowlist_var("(XED|xed)_.*")
        .blocklist_item("XED_.*_DEFINED")
        .prepend_enum_name(false)
        .impl_debug(true)
        .use_core()
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("xed-static.c"))
        .wrap_static_fns_suffix("_xed_sys_inline");

    if cfg!(feater = "dylib") {
        builder = builder.clang_arg("-DXED_DLL");
    }

    if cfg!(feature = "enc2-m32-a32") {
        builder = builder.clang_arg("-DXED_SYS_ENC2_M32_A32");
    }

    if cfg!(feature = "enc2-m64-a64") {
        builder = builder.clang_arg("-DXED_SYS_ENC2_M64_A64");
    }

    if cfg!(feature = "enc2-chk") {
        builder = builder.clang_arg("-DXED_SYS_ENC2_CHK");
    }

    let bindings = builder
        .header("xed.h")
        .generate()
        .unwrap_or_else(|e| panic!("Could not generate bindings for xed.h: {}", e));

    bindings.write_to_file(&dot_rs).unwrap_or_else(|e| {
        panic!(
            "Could not write generated bindings to {}: {e}",
            dot_rs.display()
        )
    });
}

fn main() {
    let cc = cc::Build::new();
    build_xed(&cc);
    #[cfg(feature = "bindgen")]
    build_bindgen();
    build_inline_shim(cc);
}
