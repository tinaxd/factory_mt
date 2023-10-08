use factory::parser::program as parse_program;
use factory::{compiler::Compiler, vm::VM};
use std::io::Read;

fn main() {
    // read stdin until eof
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // parse input
    let program = parse_program(&input).unwrap().1;
    // let program = product_expression(&input).unwrap().1;

    for stmt in program.iter() {
        println!("{:#?}", stmt);
    }

    let mut compiler = Compiler::new();
    compiler.compile_stmt(&program[0], None);
    compiler.link();

    let mut vm = VM::new(1024);
    vm.set_code(compiler.code());
    for _ in 0..1000 {
        vm.step_code();
        if let Some(v) = vm.stack_top() {
            println!("{:?}", v);
        }
    }
}
