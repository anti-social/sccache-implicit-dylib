// Builds dummy.cpp into a static library and links it the same way the `cc`
// crate does by default: `static=dummy` plus a bare `stdc++` without a kind.
use std::{env, path::PathBuf, process::Command};

fn tool(name: &str, default: &str) -> String {
    let target = env::var("TARGET").unwrap().replace('-', "_");
    env::var(format!("{name}_{target}"))
        .or_else(|_| env::var(name))
        .unwrap_or_else(|_| default.to_owned())
}

fn run(cmd: &mut Command) {
    let status = cmd.status().unwrap();
    assert!(status.success(), "{cmd:?} failed: {status}");
}

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let obj = out.join("dummy.o");
    println!("cargo:rerun-if-changed=dummy.cpp");

    run(Command::new(tool("CXX", "c++"))
        .args(["-O2", "-fPIC", "-c", "dummy.cpp", "-o"])
        .arg(&obj));
    run(Command::new(tool("AR", "ar"))
        .arg("crs")
        .arg(out.join("libdummy.a"))
        .arg(&obj));

    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=dummy");
    // What `cc` emits for C++ on Linux. sccache rewrites it to `dylib=stdc++`.
    println!("cargo:rustc-link-lib=stdc++");
}
