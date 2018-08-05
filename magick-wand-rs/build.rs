extern crate bindgen;
extern crate pkg_config;

use std::path::PathBuf;
use std::fmt::Write;
use std::env;

fn main() {
    let magick_wand = pkg_config::probe_library("MagickWand")
        .expect("MagickWand couldn't be found");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Just a bit fancy so the defines from pkg-config are accessible from Rust,
    // generally this can just be a static header in the repo source or
    // something.
    let header = {
        let mut bindings_header = "#include <wand/MagickWand.h>\n".to_owned();

        for (define_name, value) in magick_wand.defines.iter() {
            if let Some(value) = value {
                writeln!(
                    bindings_header,
                    "#define {} ({})",
                    define_name,
                    value,
                ).unwrap();
            }
        }

        writeln!(
            bindings_header,
            "#define MAGICKWAND_VERSION {:?}\n",
            magick_wand.version,
        ).unwrap();

        bindings_header
    };

    let bindings = {
        let mut builder = bindgen::Builder::default()
            .header_contents("bindings.h", &header)
            // Just to reduce noise in the generated bindings.
            .whitelist_function(".*Magick.*")
            .whitelist_var("MAGICK.*");

        for path in &magick_wand.include_paths {
            builder = builder.clang_arg("-isystem").clang_arg(path.to_string_lossy());
        }

        builder.generate().expect("Couldn't generate bindings")
    };

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
