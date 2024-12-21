const LINKER: &str = ".cargo/linker-mbr-i386.ld";

fn main() {
    println!("cargo:rustc-link-arg=-T{LINKER}");
    println!("cargo:rerun-if-changed={LINKER}");
}
