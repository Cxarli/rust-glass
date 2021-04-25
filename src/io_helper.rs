use std::fs::File;
use std::io::*;

use lexer;
use parser;
use transformer::javascript as js_transform;
use generator::javascript as js_generator;


pub fn execute_file(mut file: File, output_stream: &mut Write) {
    // Read input file
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Failed to read input file");


    // Execute lexer
    let tokens = lexer::lex(&buf);
    // output_stream.write_fmt(format_args!("tokens: {:?}\n\n", tokens)).ok();

    // Execute parser
    let ast = parser::parse(&tokens);
    // output_stream.write_fmt(format_args!("ast: {:?}\n\n", ast)).ok();

    // Execute transformer
    let js_ast = js_transform::transform(ast);
    // output_stream.write_fmt(format_args!("js_ast: {:?}\n\n", js_ast)).ok();

    // Execute generator
    let js_code = js_generator::generate_with_library(&js_ast);
    //output_stream.write_fmt(format_args!("js_code: {:?}\n\n", js_code)).ok();


    output_stream.write(js_code.as_bytes()).ok();
}
