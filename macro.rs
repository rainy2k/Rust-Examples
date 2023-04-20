// macro looks like functions,
// except that their name ends with a bang!

// but instead of generating a function call,
// macros are expanded into source code that gets compiled with the rest of the program.

// However,
// unlike macros in C and other languages,
// Rust macros are expanded into abstract syntax trees,
// rather than string preprocessing,
// so you don't get unexpected precedence bugs.

macro_rules! say_hello {
    //  `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block
        println!("hello rust!");
    };
}

fn main() {
    say_hello!();
}
