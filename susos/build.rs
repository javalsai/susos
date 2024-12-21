const LINKER: &str = ".cargo/linker-os-amd64.ld";

fn main() {
    println!("cargo:rustc-link-arg=-T{LINKER}");
    println!("cargo:rerun-if-changed={LINKER}");
}
