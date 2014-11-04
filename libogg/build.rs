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

    let libfiles_root = root.join("src");

    gcc::compile_library("libogg.a", &config, &[
        libfiles_root.join("bitwise.c").as_str().unwrap(),
        libfiles_root.join("framing.c").as_str().unwrap(),
    ]);
}
