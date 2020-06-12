fn main() {
    let whitelist_regex = "^[jJ]anet|ANET.*";

    let bindings = bindgen::Builder::default()
        .header("./csrc/janet.h")
        .whitelist_type(whitelist_regex)
        .whitelist_function(whitelist_regex)
        .whitelist_var(whitelist_regex)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    #[cfg(feature = "link-amalg")]
    cc::Build::new()
        .file("csrc/janet.c")
        .include("csrc")
        .compile("janet");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
