use std::{fs, io, path::PathBuf};

#[allow(dead_code)]
#[path = "../../snapshot_tests.rs"]
mod snapshot_tests;

use snapshot_tests::SNAPSHOTS_DIR_NAME;

fn main() {
    let saved_snapshots_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(SNAPSHOTS_DIR_NAME)
        .join("snapshots");
    if let Err(err) = fs::remove_dir_all(saved_snapshots_path) {
        if err.kind() != io::ErrorKind::NotFound {
            panic!("Failed to remove old snapshots: {err}");
        }
    }

    println!("Updating all snapshots:");
    for test_runner in snapshot_tests::snapshot_test_runners() {
        for snapshot in test_runner.generate_snapshots().unwrap() {
            fs::write(snapshot.save_path(), snapshot.data).unwrap();
        }
    }
}
