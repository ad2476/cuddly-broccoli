extern crate walkdir;

use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let exec_path = find_target_dir(&out_dir)
        .expect("failed to find target dir")
        .join(env::var("PROFILE").unwrap());

    copy_files(&manifest_dir.join("assets"), &exec_path.join("assets"));
}

fn find_target_dir(candidate: &Path) -> Option<&Path> {
    if candidate.ends_with("target") {
        return Some(candidate);
    }
    match candidate.parent() {
        Some(parent) => find_target_dir(parent),
        None => None,
    }
}

fn copy_files(from: &Path, to: &Path) {
    let from_path: PathBuf = from.into();
    let to_path: PathBuf = to.into();
    for entry in WalkDir::new(from_path.clone()) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from_path) {
            let target_path = to_path.join(rel_path);
            if entry.file_type().is_dir() {
                DirBuilder::new()
                    .recursive(true)
                    .create(target_path)
                    .expect("failed to create target dir");
            } else {
                fs::copy(entry.path(), &target_path).expect("failed to copy");
            }
        }
    }
}
