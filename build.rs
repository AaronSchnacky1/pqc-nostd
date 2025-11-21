fn main() {
    println!("cargo:rerun-if-changed=src/integrity.rs");
    println!("cargo:rerun-if-changed=build.rs");

    // TODO: Generate src/integrity_data.rs with placeholder HMAC
}
