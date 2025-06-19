use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;
use std::fs::File;

fn test_dir_builder () {
    let mut builder = DirBuilder::new();
    // ruleid: AIK_Rust-insecure-file-permissions
    builder.mode(0o777);
    // ok: AIK_Rust-insecure-file-permissions
    builder.mode(0o755);
}


fn foo_unsafe1() {
    let f = File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    // ruleid: AIK_Rust-insecure-file-permissions
    permissions.set_readonly(false);
}

fn foo() {
    let f = File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    // ok: AIK_Rust-insecure-file-permissions
    permissions.set_mode(0o644);
}

fn foo_unsafe2() {
    let f = File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    // ruleid: AIK_Rust-insecure-file-permissions
    permissions.set_mode(0o777);
}
