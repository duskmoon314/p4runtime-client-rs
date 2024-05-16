use std::process::Command;

fn main() {
    // build p4 code using p4c
    println!("cargo:rerun-if-changed=p4src/");
    Command::new("p4c")
        .args([
            "--target",
            "bmv2",
            "--arch",
            "v1model",
            "--std",
            "p4-16",
            "-o",
            "build/",
            "p4src/main.p4",
            "--p4runtime-files",
            "build/main.p4info.bin",
        ])
        .status()
        .unwrap();
}
