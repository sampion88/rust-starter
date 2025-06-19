use std::fs
use std::fs::File;
use std::io::prelude::*;
use tempfile::tempdir;

fn main() -> std::io::Result<()> {
    // ok: AIK_Rust-hardcoded-tmp-dir
    let safe_dir = tempdir()?;
    // ok: AIK_Rust-hardcoded-tmp-dir
    let safe_dir2 = fs::create_dir("/public");
    // ruleid: AIK_Rust-hardcoded-tmp-dir
    let dir = fs::create_dir_all("/tmp/my_app_temp_dir")?;
    // ruleid: AIK_Rust-hardcoded-tmp-dir
    let dir2 = fs::create_dir("/tmp")?;
    // ruleid: AIK_Rust-hardcoded-tmp-dir
    let mut file_1 = File::create_new("/tmp/foo.txt")?;
    file_1.write_all("Hello, world!".as_bytes())?;
    // ok: AIK_Rust-hardcoded-tmp-dir
    let mut safe_file_1 = File::create_new("foo.txt")?;
    safe_file_1.write_all("Hello, world!".as_bytes())?;
    // ruleid: AIK_Rust-hardcoded-tmp-dir
    let mut file_2 = File::create("/TMP/foo.txt")?;
    file_2.write_all(&1234_u32.to_be_bytes())?;
    // ruleid: AIK_Rust-hardcoded-tmp-dir
    let mut file_3 = File::create_buffered("/temp/foo.txt")?;
    assert!(file_3.capacity() > 0);
    for i in 0..100 {
        writeln!(&mut file_3, "{i}")?;
    }
    file_3.flush()?;
    Ok(())
}
