use cc;

fn main() {
    // Switching the order of the following two lines will affect the output.
    // As of this writing, first in wins. Because of ODR, the linker is free
    // to change this behavior at any time, no diagnostic required.

    cc::Build::new()
        .file("src/bad_2.c")
        .compile("bad_2");

    cc::Build::new()
        .file("src/bad_1.c")
        .compile("bad_1");
}
