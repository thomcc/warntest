fn main() {
    use std::io::Write;
    println!("cargo:rerun-if-env-changed=REBUILD");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:warning=cargo is sometimes okay if you know about this");
    let read = false;
    let mut ttyout = std::fs::OpenOptions::new()
        .write(true)
        .read(read)
        .open("/dev/tty")
        .expect("failed to open /dev/tty");
    write!(
        ttyout,
        "\n\x1b[1;33mwarning\x1b[39m:\x1b[0m cargo doesn't want you to know about this\n",
    )
    .expect("failed to write");
    ttyout.flush().expect("failed to flush");
}
