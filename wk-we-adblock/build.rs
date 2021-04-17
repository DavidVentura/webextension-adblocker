extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let webkit2gtk = pkg_config::probe_library("webkit2gtk-4.0").unwrap();
    let gtk = pkg_config::probe_library("gtk+-3.0").unwrap();

    println!("cargo:rerun-if-changed=wrapper.h");

    // Convert path to includes to -I<path> format
    let gtk_pathed = gtk
        .include_paths
        .iter()
        .map(|x| format!("-I{}", x.to_string_lossy()));
    let webkit_pathed = webkit2gtk
        .include_paths
        .iter()
        .map(|x| format!("-I{}", x.to_string_lossy()));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .whitelist_function("g_signal_connect_object")
        .whitelist_function("webkit_uri_request_get_uri")
        .whitelist_function("webkit_web_page_get_id")
        .whitelist_function("webkit_web_page_get_uri")
        .blacklist_type("GObject")
        .whitelist_type("GCallback")
        .whitelist_type("WebKitWebPage")
        .whitelist_type("WebKitURIRequest")
        .whitelist_type("WebKitURIResponse")
        .whitelist_type("gpointer")
        .whitelist_type("WebKitWebExtension")
        .header("wrapper.h")
        .clang_args(webkit_pathed)
        .clang_args(gtk_pathed)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
