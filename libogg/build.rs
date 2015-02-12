extern crate "pkg-config" as pkg_config;
extern crate gcc;

fn main() {
    match pkg_config::find_library("ogg") {
        Ok(()) => return,
        Err(..) => {}
    };

    let root = Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap())
        .join("libogg");

    gcc::Config::new()
                .file("libogg/src/bitwise.c")
                .file("libogg/src/framing.c")
                .define("_USRDLL", None)
                .define("LIBOGG_EXPORTS", None)
                .include(root.join("include"))
                .compile("libogg.a");
}
