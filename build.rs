fn main() {
    use std::io::Write;
    println!("cargo:rerun-if-env-changed=REBUILD");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    let read = false;
    let mut ttyout = std::fs::OpenOptions::new()
        .write(true)
        .read(read)
        .open("/dev/tty")
        .expect("failed to open /dev/tty");
    write!(
        ttyout,
        "\n\x1b[1;33mcargo doesnt want you to know this, but it's a warning\x1b[m\n"
    )
    .expect("failed to write");
    ttyout.flush().expect("failed to flush");
}
