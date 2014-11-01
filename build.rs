extern crate gcc;

fn main() {
    gcc::compile_library("libogg.a", &[
        "libogg/bitwise.c",
        "libogg/framing.c"
    ]);
}
