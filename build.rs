/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::process::Command;
use std::path::{PathBuf, Path};
use std::env;

fn main() {
    let src_dir = env::current_dir().unwrap();
    if cfg!(windows) {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        env::set_var("OUT_DIR", msys_compatible(&out_dir));
    }
    let jobs = format!("-j{}", env::var("NUM_JOBS").unwrap());
    let make_result = Command::new("make")
        .args(&["-f", &msys_compatible(&src_dir.join("makefile.cargo")), &jobs])
        .status()
        .unwrap();
    assert!(make_result.success());
}

// Function taken from
// https://github.com/alexcrichton/curl-rust/blob/master/curl-sys/build.rs
fn msys_compatible(path: &Path) -> String {
    let path = path.to_str().unwrap();
    if !cfg!(windows) {
        return path.to_string()
    }
    path.replace("C:\\", "/c/").replace("\\", "/")
}
