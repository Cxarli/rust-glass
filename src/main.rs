extern crate glass;
use glass::*;

use std::env;

use generator::javascript as js_gen;


fn main() {
    let mut argv = env::args();
    // Drop program name
    argv.next();


    let filename = match argv.next() {
        Some(x) => x,
        None => "src/tests/examples/fibonacci.glass".to_string(),
    };


    let file = ::std::fs::File::open(&filename).expect(&format!("Failed to find file {}", &filename));
    execute_file(file, &mut ::std::io::stdout());

    /*
    println!("{}",
        js_gen::generate_with_library(
            &transformer::javascript::transform(
                parser::parse(
                    &lexer::lex(
                        "{M [m  ] }"
                    )
                )
            )
        )
    );
    // */
}
