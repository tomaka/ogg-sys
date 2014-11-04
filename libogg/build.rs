extern crate gcc;

fn main() {
    let root = Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap())
        .join("libogg");

    let config = gcc::Config {
        include_directories: vec![
            root.join("include"),
        ],
        definitions: vec![
            ("_USRDLL".to_string(), None),
            ("LIBOGG_EXPORTS".to_string(), None)
        ],
        .. std::default::Default::default()
    };

    println!("cargo:include={}", root.join("include").display());

    gcc::compile_library("libogg.a", &config, &[
        "libogg/src/bitwise.c",
        "libogg/src/framing.c",
    ]);
}
