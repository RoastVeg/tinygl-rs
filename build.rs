fn main() {
    let dir = std::env::var("OUT_DIR").unwrap();
    let _tinygl = cmake::Config::new("tinygl")
        .define("TINYGL_BUILD_SHARED", "OFF")
        .define("CMAKE_DISABLE_FIND_PACKAGE_OpenMP", "TRUE")
        .build();
    println!("cargo:rustc-link-search=native={}{}", dir, "/lib");
    println!("cargo:rustc-link-lib=static=tinygl-static");
    let bindings = bindgen::Builder::default()
        .header("tinygl/include/zbuffer.h")
    //.allowlist_var("(GL.*|ZB.*|TGL_[^F].*|COLOR.*|TINYGL.*)")
	.allowlist_var("(GL.*|ZB.*)")
        .allowlist_type("(GL.*|ZB.*)")
        .allowlist_function("(gl.*|ZB.*)")
        //.bitfield_enum("GL_.*_BITS?")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .use_core()
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
