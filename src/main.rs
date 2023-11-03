use factory::compiler::Compiler;
use factory::parser::program as parse_program;
use nom::Finish;
use nom_locate::LocatedSpan;
use std::io::{Read, Write};

fn main() {
    // read stdin until eof
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // parse input
    let program = parse_program(LocatedSpan::new(&input)).finish();
    if let Err(e) = program {
        println!("{:#?}", e);
        return;
    }
    let program = program.unwrap();
    println!("remaining: {:?}", program);
    let program = program.1;
    // let program = product_expression(&input).unwrap().1;

    for stmt in program.iter() {
        println!("{:#?}", stmt);
    }

    let mut compiler = Compiler::new();
    let module = compiler.compile_top(&program[0]);

    Compiler::dump_module(&module);
}
