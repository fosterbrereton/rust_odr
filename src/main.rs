extern "C" {
    fn bad_function() -> ();
}

fn main() {
    unsafe {
        bad_function();
    }
}
