This project aims to show how non-Rust dependencies in a Rust project dramatically reduce the safety guarantees Rust otherwise upholds.

## What does this project do?

In this project are two C files built and linked into the Rust-based application. They both define the same symbol, `bad_function`, though their definitions are not the same. This is an ODR violation in C/C++, and no diagnostic is required. The linker silently picks one and merges it into the final Rust binary without error.

In `build.rs`, you can alter the final application by changing the order in which the two files are passed to the linker.

## Why is this bad?

Nobly, Rust aims to be safe by default. However, this demonstration shows how easy it is for `cargo` to introduce unsafe behaviors into an otherwise safe ecosystem.

## "So don't do this"

The argument could be made that the author of `build.rs` has shot themselves in the foot, and they deserve what is coming to them. While that may be true in this oversimplified case, it highlights a weakness in Rust's build ecosystem that can trip up even the most well-meaning Rust developers.

Consider the case where Rust is performing dependency resolution, and determines that more than one variant of the same dependency should be brought in to the application. [Rust does not consider this a problem](https://stephencoakley.com/2019/04/24/how-rust-solved-dependency-hell), and indeed has a very complex name mangling scheme to ensure that this can be done safely by default for Rust-based symbols. The problem is that the name mangling does not apply for non-Rust based dependencies.

If (twenty layers down in my dependency hierarchy) more than one variant of the same Rust library is brought in to my application _that each depend on different variants of the same non-Rust dependency_, I now have an accidental ODR violation in my Rust application.
